// Copyright 2026 Ethan Wu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! Manifest-generated shared-memory endpoint instances.

use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::mem::MaybeUninit;

use consortium_codec::CodecFor;
use consortium_ipc::{Channel, Connect, Doorbell, Rx, Side, Transceiver as CoreTransceiver, Tx};
use embassy_hal_internal::{Peri, PeripheralType};

use crate::{
    DescriptorError, DescriptorLayout, Error, RegionSharedMemoryTransportable,
    SharedMemoryTransport, SharedMemoryTransportRx, SharedMemoryTransportTx,
};

/// Descriptor-handshake failure with the doorbell-specific payload erased.
///
/// Erasing the doorbell error lets one application-level error enum cover
/// endpoints backed by different hardware doorbells without allocation.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ConnectError {
    /// The descriptor header or channel table was invalid.
    Descriptor(DescriptorError),
    /// Hardware signalling failed during the handshake.
    Doorbell,
    /// Another transport failure occurred during the handshake.
    Other,
}

impl<D: Doorbell> From<Error<D>> for ConnectError {
    fn from(error: Error<D>) -> Self {
        match error {
            Error::Descriptor(error) => Self::Descriptor(error),
            Error::Doorbell(_) => Self::Doorbell,
            _ => Self::Other,
        }
    }
}

struct EndpointResources<T: Instance> {
    transport: InstanceTransport<T>,
    tx_buf: &'static mut [u8],
    rx_buf: &'static mut [u8],
}

const EMPTY: u8 = 0;
const INSTALLED: u8 = 1;
const TAKEN: u8 = 2;
#[cfg(feature = "std")]
const INSTALLING: u8 = 3;

/// Cross-platform once guard used by generated singleton bundles.
#[doc(hidden)]
pub struct SingletonGuard {
    #[cfg(feature = "std")]
    taken: core::sync::atomic::AtomicBool,
    #[cfg(not(feature = "std"))]
    taken: UnsafeCell<bool>,
}

// SAFETY: the std representation is atomic, and every access to the no-std
// representation occurs inside a critical section.
unsafe impl Sync for SingletonGuard {}

impl SingletonGuard {
    /// Create an untaken guard suitable for a `static`.
    pub const fn new() -> Self {
        Self {
            #[cfg(feature = "std")]
            taken: core::sync::atomic::AtomicBool::new(false),
            #[cfg(not(feature = "std"))]
            taken: UnsafeCell::new(false),
        }
    }

    /// Mark the guard taken, returning whether this was the first call.
    pub fn take(&self) -> bool {
        #[cfg(feature = "std")]
        {
            !self.taken.swap(true, core::sync::atomic::Ordering::AcqRel)
        }
        #[cfg(not(feature = "std"))]
        {
            critical_section::with(|_| {
                // SAFETY: protected by the critical section.
                unsafe {
                    let taken = &mut *self.taken.get();
                    if *taken {
                        false
                    } else {
                        *taken = true;
                        true
                    }
                }
            })
        }
    }
}

impl Default for SingletonGuard {
    fn default() -> Self {
        Self::new()
    }
}

/// Take-once storage for one generated endpoint instance.
///
/// Generated `init` installs the transport. Consuming the matching channel
/// singleton is then the only safe way to remove it and its two scratch buffers.
pub struct EndpointState<T: Instance> {
    resources: UnsafeCell<MaybeUninit<EndpointResources<T>>>,
    #[cfg(feature = "std")]
    state: core::sync::atomic::AtomicU8,
    #[cfg(not(feature = "std"))]
    state: UnsafeCell<u8>,
}

// SAFETY: `resources` is touched only while performing the single EMPTY ->
// INSTALLED -> TAKEN transition. The std path uses atomics and requires the
// transported value to be `Send`; the no-std path uses a critical section,
// including on targets without compare-and-swap such as thumbv6m.
unsafe impl<T> Sync for EndpointState<T>
where
    T: Instance,
    InstanceTransport<T>: Send,
{
}

impl<T: Instance> EndpointState<T> {
    /// Create an empty endpoint slot suitable for a `static`.
    pub const fn new() -> Self {
        Self {
            resources: UnsafeCell::new(MaybeUninit::uninit()),
            #[cfg(feature = "std")]
            state: core::sync::atomic::AtomicU8::new(EMPTY),
            #[cfg(not(feature = "std"))]
            state: UnsafeCell::new(EMPTY),
        }
    }

    #[doc(hidden)]
    pub fn install(
        &self,
        transport: InstanceTransport<T>,
        tx_buf: &'static mut [u8],
        rx_buf: &'static mut [u8],
    ) {
        assert_eq!(tx_buf.len(), T::MTU, "endpoint TX buffer has the wrong MTU");
        assert_eq!(rx_buf.len(), T::MTU, "endpoint RX buffer has the wrong MTU");

        #[cfg(feature = "std")]
        {
            use core::sync::atomic::Ordering;
            self.state
                .compare_exchange(EMPTY, INSTALLING, Ordering::AcqRel, Ordering::Acquire)
                .unwrap_or_else(|_| panic!("endpoint transport has already been installed"));
            // SAFETY: this call alone won the EMPTY -> INSTALLING transition.
            let resources = unsafe { &mut *self.resources.get() };
            resources.write(EndpointResources {
                transport,
                tx_buf,
                rx_buf,
            });
            self.state.store(INSTALLED, Ordering::Release);
        }

        #[cfg(not(feature = "std"))]
        critical_section::with(|_| {
            // SAFETY: the flag is only accessed inside this critical section.
            let state = unsafe { &mut *self.state.get() };
            if *state != EMPTY {
                panic!("endpoint transport has already been installed");
            }
            // SAFETY: this is the only EMPTY -> INSTALLED transition.
            let resources = unsafe { &mut *self.resources.get() };
            resources.write(EndpointResources {
                transport,
                tx_buf,
                rx_buf,
            });
            *state = INSTALLED;
        });
    }

    fn take(&self) -> EndpointResources<T> {
        #[cfg(feature = "std")]
        {
            use core::sync::atomic::Ordering;
            self.state
                .compare_exchange(INSTALLED, TAKEN, Ordering::AcqRel, Ordering::Acquire)
                .unwrap_or_else(|state| match state {
                    EMPTY => panic!("endpoint transport has not been installed"),
                    _ => panic!("endpoint has already been taken"),
                });
            // SAFETY: this call alone won the INSTALLED -> TAKEN transition.
            let resources = unsafe { &*self.resources.get() };
            // SAFETY: INSTALLED proves the slot was initialized.
            unsafe { resources.assume_init_read() }
        }

        #[cfg(not(feature = "std"))]
        critical_section::with(|_| {
            // SAFETY: the flag is only accessed inside this critical section.
            let state = unsafe { &mut *self.state.get() };
            match *state {
                EMPTY => panic!("endpoint transport has not been installed"),
                TAKEN => panic!("endpoint has already been taken"),
                INSTALLED => {
                    *state = TAKEN;
                    // SAFETY: INSTALLED proves initialization, and setting TAKEN
                    // transfers the value exactly once in the same critical section.
                    let resources = unsafe { &*self.resources.get() };
                    // SAFETY: the slot is initialized and cannot be read again.
                    unsafe { resources.assume_init_read() }
                }
                _ => unreachable!(),
            }
        })
    }
}

impl<T: Instance> Default for EndpointState<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal hooks implemented by [`impl_channel!`](crate::impl_channel).
#[doc(hidden)]
pub trait SealedInstance: PeripheralType + Sized + 'static {
    /// Return this generated instance's take-once state.
    fn state() -> &'static EndpointState<Self>
    where
        Self: Instance;

    /// Install the transport and the macro-owned scratch buffers.
    fn install(transport: InstanceTransport<Self>)
    where
        Self: Instance;
}

/// A generated shared-memory channel instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static {
    /// Which descriptor side this endpoint runs on.
    type Side: Side;
    /// Hardware or software notification mechanism.
    type Doorbell: Doorbell;
    /// Shared-memory mapping and cache semantics.
    type Region: RegionSharedMemoryTransportable;
    /// Message sent by this endpoint.
    type TxMsg;
    /// Message received by this endpoint.
    type RxMsg;
    /// Serialization family for both directions.
    type Codec: CodecFor<Self::TxMsg> + CodecFor<Self::RxMsg>;

    /// Negotiated payload size and scratch-buffer length.
    const MTU: usize;

    /// Whether this endpoint wired acknowledgement doorbell lines.
    ///
    /// Defaulted so an endpoint declared before acknowledgement existed — or one that
    /// simply does not want it — needs to say nothing. Published in the descriptor during
    /// the handshake, where a peer that disagrees is rejected.
    const ACK: bool = false;
}

/// The unsplit transport stored for instance `T`.
pub type InstanceTransport<T> = SharedMemoryTransport<
    <T as Instance>::Side,
    <T as Instance>::Doorbell,
    <T as Instance>::Region,
>;

/// The connected typed transceiver produced for instance `T`.
pub type Connected<T> = CoreTransceiver<
    <T as Instance>::TxMsg,
    <T as Instance>::RxMsg,
    SharedMemoryTransportTx<
        <T as Instance>::Side,
        <T as Instance>::Doorbell,
        <T as Instance>::Region,
    >,
    SharedMemoryTransportRx<
        <T as Instance>::Side,
        <T as Instance>::Doorbell,
        <T as Instance>::Region,
    >,
    <T as Instance>::Codec,
>;

/// Install one transport into its generated endpoint slot.
///
/// This is intended for generated `init` code. The transport constructor itself
/// is unsafe, so safe application code cannot forge an installation.
#[doc(hidden)]
pub fn install<T: Instance>(transport: InstanceTransport<T>) {
    T::install(transport);
}

/// A constructed but not yet connected shared-memory endpoint.
pub struct Endpoint<T: Instance> {
    resources: EndpointResources<T>,
    _instance: PhantomData<T>,
}

impl<T: Instance> Endpoint<T> {
    /// Consume the channel singleton and take its installed transport.
    pub fn new(peri: Peri<'static, T>) -> Self {
        let _ = peri;
        Self {
            resources: T::state().take(),
            _instance: PhantomData,
        }
    }

    /// Handshake, then split, then construct the two typed channels.
    pub async fn connect(mut self) -> Result<Connected<T>, ConnectError>
    where
        T::Doorbell: Clone + Send + 'static,
        T::Region: Clone + Send,
        InstanceTransport<T>: Connect<Params = DescriptorLayout, Error = Error<T::Doorbell>>,
    {
        let (chan_tx, chan_rx) = self.resources.transport.channels();
        self.resources
            .transport
            .connect(DescriptorLayout::new(2, T::MTU).with_ack(T::ACK))
            .await
            .map_err(ConnectError::from)?;
        let (transport_tx, transport_rx) = self.resources.transport.split();
        let channel_tx =
            Channel::<Tx, T::TxMsg, _, T::Codec>::new(chan_tx, transport_tx, self.resources.tx_buf);
        let channel_rx =
            Channel::<Rx, T::RxMsg, _, T::Codec>::new(chan_rx, transport_rx, self.resources.rx_buf);
        Ok(CoreTransceiver::new(channel_tx, channel_rx))
    }
}

/// Constructors for typed shared-memory transceivers.
///
/// [`connect`](Self::connect) is the Embassy-shaped one-line bring-up path. The
/// existing [`new`](Self::new) spelling retains its meaning: pair two already
/// constructed typed channels without doing any transport bring-up.
pub struct Transceiver;

impl Transceiver {
    /// Consume an endpoint singleton and return its connected transceiver.
    pub async fn connect<T: Instance>(peri: Peri<'static, T>) -> Result<Connected<T>, ConnectError>
    where
        T::Doorbell: Clone + Send + 'static,
        T::Region: Clone + Send,
        InstanceTransport<T>: Connect<Params = DescriptorLayout, Error = Error<T::Doorbell>>,
    {
        Endpoint::new(peri).connect().await
    }

    /// Pair an existing typed send channel and receive channel.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<TyTx, TyRx, TrTx, TrRx, C>(
        tx: Channel<Tx, TyTx, TrTx, C>,
        rx: Channel<Rx, TyRx, TrRx, C>,
    ) -> CoreTransceiver<TyTx, TyRx, TrTx, TrRx, C>
    where
        C: CodecFor<TyTx> + CodecFor<TyRx>,
    {
        CoreTransceiver::new(tx, rx)
    }
}

/// Declare the channel singleton markers and their once-only bundle.
#[macro_export]
macro_rules! channels {
    ($($name:ident),* $(,)?) => {
        /// Zero-sized marker types for generated IPC channel singletons.
        pub mod channels {
            $(
                #[allow(non_camel_case_types)]
                #[derive(Debug, Clone, Copy)]
                pub struct $name { _private: () }

                impl $name {
                    /// Unsafely create this singleton out of thin air.
                    ///
                    /// # Safety
                    /// The caller must ensure there is only one live instance.
                    pub const unsafe fn steal() -> $crate::Peri<'static, Self> {
                        // SAFETY: forwarded to the caller.
                        unsafe { $crate::Peri::new_unchecked(Self { _private: () }) }
                    }
                }

                impl $crate::PeripheralType for $name {}
            )*
        }

        /// Every manifest-generated IPC channel singleton.
        ///
        /// This type deliberately has no `Drop` implementation so fields may be
        /// moved out independently (`c.SENSOR`, `c.LP`, ...).
        #[allow(non_snake_case)]
        pub struct Channels {
            $(pub $name: $crate::Peri<'static, channels::$name>,)*
        }

        impl Channels {
            /// Return all channel singletons once.
            ///
            /// # Panics
            /// Panics when called more than once.
            pub(crate) fn take() -> Self {
                static TAKEN: $crate::SingletonGuard = $crate::SingletonGuard::new();
                if !TAKEN.take() {
                    panic!("the IPC channels have already been taken");
                }
                // SAFETY: the guard made this the sole successful call.
                unsafe { Self::steal() }
            }

            /// Unsafely create all channel singletons out of thin air.
            ///
            /// # Safety
            /// The caller must ensure each singleton is uniquely owned.
            pub unsafe fn steal() -> Self {
                Self {
                    $($name: unsafe { channels::$name::steal() },)*
                }
            }
        }
    };
}

/// Bind one generated channel marker to its endpoint types and storage.
///
/// The doorbell channel ids are not arguments: generated init builds the transport with
/// them before installing it, so the marker never needs to know them.
#[macro_export]
macro_rules! impl_channel {
    // With the acknowledgement capability, for a channel that sets `config.ack`.
    ($name:ident, $side:ty, $doorbell:ty, $region:ty, $tx:ty, $rx:ty, $codec:ty,
        $mtu:expr, ack $(,)?) => {
        $crate::impl_channel!(
            @impl $name, $side, $doorbell, $region, $tx, $rx, $codec, $mtu, true,
        );
    };
    // Without it — the ordinary form.
    ($name:ident, $side:ty, $doorbell:ty, $region:ty, $tx:ty, $rx:ty, $codec:ty,
        $mtu:expr $(,)?) => {
        $crate::impl_channel!(
            @impl $name, $side, $doorbell, $region, $tx, $rx, $codec, $mtu, false,
        );
    };
    // Internal arm carrying the resolved acknowledgement capability.
    (@impl $name:ident, $side:ty, $doorbell:ty, $region:ty, $tx:ty, $rx:ty, $codec:ty,
        $mtu:expr, $ack:expr $(,)?) => {
        impl $crate::SealedInstance for channels::$name {
            fn state() -> &'static $crate::EndpointState<Self> {
                static STATE: $crate::EndpointState<channels::$name> = $crate::EndpointState::new();
                &STATE
            }

            fn install(transport: $crate::InstanceTransport<Self>) {
                static mut TX_BUF: [u8; $mtu] = [0; $mtu];
                static mut RX_BUF: [u8; $mtu] = [0; $mtu];
                // SAFETY: the endpoint state accepts one installation only, and
                // thereafter moves each reference out exactly once.
                unsafe {
                    <Self as $crate::SealedInstance>::state().install(
                        transport,
                        &mut *::core::ptr::addr_of_mut!(TX_BUF),
                        &mut *::core::ptr::addr_of_mut!(RX_BUF),
                    );
                }
            }
        }

        impl $crate::Instance for channels::$name {
            type Side = $side;
            type Doorbell = $doorbell;
            type Region = $region;
            type TxMsg = $tx;
            type RxMsg = $rx;
            type Codec = $codec;

            const MTU: usize = $mtu;
            const ACK: bool = $ack;
        }
    };
}
