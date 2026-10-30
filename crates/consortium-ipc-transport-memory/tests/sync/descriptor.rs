use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use consortium_ipc::{Chan, Doorbell, MaybeSend, Primary, Secondary};
use consortium_ipc_transport_memory::{
    CHANNEL_NUM_MAX, ChannelDirection, ChannelLayout, DESCRIPTOR_LENGTH, DescState, Descriptor,
    DescriptorError, DescriptorLayout, Error, FLAG_CHAN_ACK_CAPABLE, SLOT_ALIGNMENT,
    SharedMemoryTransport, VERSION, channel_table_entry_offset, descriptor_complete, get_state,
    set_state, slot, slot_offset, slot_stride,
};
use consortium_shared_memory::{
    UncachedRegion,
    descriptor::{DescriptorHeader, DescriptorHeaderError, DescriptorMagic},
};
use futures::executor::block_on;

const DESCRIPTOR_LEN: usize = 0x800;
const NUM_CHANNELS_OFFSET: usize = 0x06;
const RX_TRANSFER_SIZE_OFFSET: usize = 0x10;
const STATE_OFFSET: usize = 0x18;

type PrimaryTransport = SharedMemoryTransport<Primary, ImmediateDoorbell, UncachedRegion>;
type SecondaryTransport = SharedMemoryTransport<Secondary, ImmediateDoorbell, UncachedRegion>;

#[derive(Clone, Copy, Debug)]
struct ImmediateDoorbell;

struct ImmediateWaitFut {
    ch: Chan,
}

impl Future for ImmediateWaitFut {
    type Output = Result<Chan, Infallible>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Ok(self.ch))
    }
}

impl Doorbell for ImmediateDoorbell {
    type Error = Infallible;

    fn ring(&self, _ch: Chan) -> Result<(), Self::Error> {
        Ok(())
    }

    fn wait(&self, ch: Chan) -> impl Future<Output = Result<Chan, Self::Error>> + MaybeSend {
        ImmediateWaitFut { ch }
    }
}

fn init_descriptor(base: *mut u8, num_channels: u8, transfer_size: usize) {
    init_descriptor_with(base, DescriptorLayout::new(num_channels, transfer_size));
}

fn init_descriptor_with(base: *mut u8, layout: DescriptorLayout) {
    let doorbell = ImmediateDoorbell;
    let ch = unsafe { Chan::new_unchecked(0) };

    block_on(async {
        unsafe {
            PrimaryTransport::descriptor_init(base, layout, &doorbell, ch, ch)
                .await
                .expect("primary descriptor init should complete");
        }
    });
}

fn ack_descriptor(
    base: *mut u8,
    num_channels: usize,
    transfer_size: usize,
) -> Result<Descriptor, Error<ImmediateDoorbell>> {
    ack_descriptor_with(
        base,
        DescriptorLayout::new(num_channels as u8, transfer_size),
    )
}

fn ack_descriptor_with(
    base: *mut u8,
    layout: DescriptorLayout,
) -> Result<Descriptor, Error<ImmediateDoorbell>> {
    let doorbell = ImmediateDoorbell;
    let ch = unsafe { Chan::new_unchecked(0) };

    block_on(async {
        unsafe { SecondaryTransport::descriptor_ack(base, layout, &doorbell, ch, ch).await }
    })
}

fn write_u8(base: *mut u8, offset: usize, value: u8) {
    unsafe {
        base.add(offset).write_volatile(value);
    }
}

fn write_u32(base: *mut u8, offset: usize, value: u32) {
    unsafe {
        (base.add(offset) as *mut u32).write_volatile(value);
    }
}

fn read_u32(base: *const u8, offset: usize) -> u32 {
    unsafe { (base.add(offset) as *const u32).read_volatile() }
}

#[test]
fn descriptor_init_writes_documented_layout_offsets() {
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base = backing.as_mut_ptr();

    init_descriptor(base, 2, 96);

    assert_eq!(backing[4], VERSION);
    assert_eq!(backing[5], 0, "header reserved byte must stay reserved");
    assert_eq!(backing[NUM_CHANNELS_OFFSET], 2);
    assert_eq!(backing[7], 0, "descriptor reserved byte must stay reserved");
    assert_eq!(read_u32(base, RX_TRANSFER_SIZE_OFFSET), 96);
    assert_eq!(read_u32(base, STATE_OFFSET), DescState::Ready as u32);

    let descriptor = unsafe { Descriptor::from_raw_ptr(base) };
    let slot_size = (slot::SLOT_HEADER + 96) as u32;
    assert_eq!(descriptor.num_channels, 2);
    assert_eq!(descriptor.rx_mtu, 96);
    assert_eq!(descriptor.state, DescState::Ready);
    assert_eq!(descriptor.channels[0].size, slot_size);
    assert_eq!(
        descriptor.channels[0].direction,
        ChannelDirection::PrimaryToSecondary
    );
    assert_eq!(
        descriptor.channels[1].offset,
        descriptor.channels[0].offset + slot_stride(96) as u32
    );
    assert_eq!(descriptor.channels[1].size, slot_size);
    assert_eq!(
        descriptor.channels[1].direction,
        ChannelDirection::SecondaryToPrimary
    );

    // A plain link publishes no capability bits, and the two bytes after them stay
    // reserved.  This is what lets a peer built before acknowledgement existed pair with a
    // non-acknowledging peer built after it: both read zero and agree.
    for index in 0..2 {
        let entry = channel_table_entry_offset(index);
        assert_eq!(backing[entry + 9], 0, "channel {index} flags");
        assert_eq!(&backing[entry + 10..entry + 12], &[0, 0]);
        assert!(!descriptor.channels[index].ack_capable());
    }
}

#[test]
fn descriptor_init_publishes_the_ack_capability_per_channel() {
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base = backing.as_mut_ptr();

    init_descriptor_with(base, DescriptorLayout::new(2, 96).with_ack(true));

    let descriptor = unsafe { Descriptor::from_raw_ptr(base) };
    for index in 0..2 {
        let entry = channel_table_entry_offset(index);
        assert_eq!(backing[entry + 9], FLAG_CHAN_ACK_CAPABLE);
        assert!(descriptor.channels[index].ack_capable());
    }

    // The secondary declaring the same capability accepts it.
    assert!(ack_descriptor_with(base, DescriptorLayout::new(2, 96).with_ack(true)).is_ok());
}

#[test]
fn descriptor_ack_rejects_an_ack_capability_mismatch() {
    // Both ends are generated from one manifest, so a disagreement means one build is
    // stale.  Catching it during bring-up is the whole point of putting the capability on
    // the wire: the alternative is a sender that parks forever on a line the peer never
    // rings.
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base = backing.as_mut_ptr();

    init_descriptor_with(base, DescriptorLayout::new(2, 96).with_ack(true));

    let error = ack_descriptor_with(base, DescriptorLayout::new(2, 96))
        .expect_err("a secondary without ack lines must reject an ack-capable descriptor");
    assert!(matches!(
        error,
        Error::Descriptor(DescriptorError::AckMismatch {
            index: 0,
            expected: false,
            actual: true,
        })
    ));

    // ...and the mirror image, which is the more dangerous direction: here it is the
    // secondary that would park on an acknowledgement the primary never sends.
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base = backing.as_mut_ptr();
    init_descriptor(base, 2, 96);

    let error = ack_descriptor_with(base, DescriptorLayout::new(2, 96).with_ack(true))
        .expect_err("an ack-capable secondary must reject a plain descriptor");
    assert!(matches!(
        error,
        Error::Descriptor(DescriptorError::AckMismatch {
            index: 0,
            expected: true,
            actual: false,
        })
    ));
}

#[test]
fn descriptor_slots_do_not_share_cache_lines() {
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base = backing.as_mut_ptr();

    init_descriptor(base, 2, 128);

    let descriptor = unsafe { Descriptor::from_raw_ptr(base) };
    let first = descriptor.channels[0];
    let second = descriptor.channels[1];
    assert_eq!(first.offset as usize % SLOT_ALIGNMENT, 0);
    assert_eq!(second.offset as usize % SLOT_ALIGNMENT, 0);
    assert!(
        (first.offset as usize + first.size as usize).div_ceil(SLOT_ALIGNMENT) * SLOT_ALIGNMENT
            <= second.offset as usize
    );
}

#[test]
fn set_and_get_state_use_the_state_word() {
    let mut backing = vec![0; 0x20];
    let base = backing.as_mut_ptr();

    unsafe {
        set_state(base, DescState::Suspended);
    }

    assert_eq!(unsafe { get_state(base) }, DescState::Suspended);
    assert_eq!(read_u32(base, STATE_OFFSET), DescState::Suspended as u32);

    write_u32(base, STATE_OFFSET, 0x1234_5678);
    assert_eq!(unsafe { get_state(base) }, DescState::Unrecognized);
}

#[test]
fn descriptor_ack_validates_and_marks_descriptor_ready() {
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base = backing.as_mut_ptr();

    init_descriptor(base, 2, 64);

    let descriptor =
        ack_descriptor(base, 2, 128).expect("descriptor should be ready to acknowledge");

    assert_eq!(descriptor.num_channels, 2);
    assert_eq!(descriptor.state, DescState::Ready);
    assert_eq!(descriptor.tx_mtu, 128);
    assert_eq!(unsafe { get_state(base) }, DescState::Ready);
    assert_eq!(read_u32(base, RX_TRANSFER_SIZE_OFFSET), 128);
    assert_eq!(
        descriptor.channels[0].direction,
        ChannelDirection::PrimaryToSecondary
    );
    assert_eq!(
        descriptor.channels[1].direction,
        ChannelDirection::SecondaryToPrimary
    );
}

#[test]
fn two_threads_complete_descriptor_handshake() {
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base_addr = backing.as_mut_ptr() as usize;
    let (init_tx, init_rx) = std::sync::mpsc::channel();
    let (ack_tx, ack_rx) = std::sync::mpsc::channel();

    let m_side = std::thread::spawn(move || {
        let base = base_addr as *mut u8;
        init_descriptor(base, 2, 96);
        assert_eq!(unsafe { get_state(base) }, DescState::Ready);
        init_tx.send(()).expect("signal descriptor is initialized");

        ack_rx.recv().expect("wait for secondary ack");
        let transfer_size = unsafe { descriptor_complete(base) };
        assert_eq!(transfer_size, 128);
        assert_eq!(unsafe { get_state(base) }, DescState::Ready);
    });

    let a_side = std::thread::spawn(move || {
        init_rx.recv().expect("wait for primary init");

        let base = base_addr as *mut u8;
        let descriptor = ack_descriptor(base, 2, 128).expect("secondary acknowledges descriptor");

        assert_eq!(descriptor.num_channels, 2);
        assert_eq!(descriptor.tx_mtu, 128);
        assert_eq!(descriptor.state, DescState::Ready);
        assert_eq!(unsafe { get_state(base) }, DescState::Ready);
        ack_tx.send(()).expect("signal descriptor is acknowledged");
    });

    m_side.join().expect("primary-side handshake completes");
    a_side.join().expect("secondary-side handshake completes");
}

#[test]
fn descriptor_ack_rejects_bad_magic() {
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base = backing.as_mut_ptr();

    init_descriptor(base, 1, 64);
    unsafe {
        DescriptorHeader::new(DescriptorMagic::DEBUG, VERSION).write_to_raw_ptr(base);
    }

    let err =
        ack_descriptor(base, 1, 64).expect_err("debug descriptor magic should be rejected for IPC");
    assert!(matches!(
        err,
        Error::Descriptor(DescriptorError::Header(
            DescriptorHeaderError::BadMagic { .. }
        ))
    ));
}

#[test]
fn descriptor_ack_rejects_bad_version() {
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base = backing.as_mut_ptr();

    init_descriptor(base, 1, 64);
    write_u8(base, 4, VERSION + 1);

    let err =
        ack_descriptor(base, 1, 64).expect_err("descriptor version mismatch should be rejected");
    assert!(matches!(
        err,
        Error::Descriptor(DescriptorError::Header(DescriptorHeaderError::BadVersion(
            _,
            _
        )))
    ));
}

#[test]
fn descriptor_ack_waits_then_rejects_unpublished_descriptor() {
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base = backing.as_mut_ptr();

    // Fully initialize the descriptor, then clear the header magic to simulate a
    // descriptor whose body is present but whose magic has not been published yet.
    // The state word is irrelevant to readiness now; the magic is the gate.
    init_descriptor(base, 1, 64);
    write_u32(base, 0, 0);

    // The magic does not match, so ack waits on the doorbell. The immediate doorbell
    // fires without the primary republishing the magic, so the re-check still fails.
    let err = ack_descriptor(base, 1, 64)
        .expect_err("unpublished descriptor magic should be rejected after the doorbell wait");
    assert!(matches!(
        err,
        Error::Descriptor(DescriptorError::Header(
            DescriptorHeaderError::UnknownMagic { .. }
        ))
    ));
}

#[test]
fn descriptor_ack_rejects_channel_count_mismatch() {
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base = backing.as_mut_ptr();

    init_descriptor(base, 1, 64);

    let err = ack_descriptor(base, 2, 64)
        .expect_err("descriptor channel count mismatch should be rejected");
    assert!(matches!(
        err,
        Error::Descriptor(DescriptorError::InvalidChannelCount {
            expected: 2,
            actual: 1
        })
    ));
}

#[test]
fn descriptor_ack_rejects_too_many_channels_before_reading_table() {
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base = backing.as_mut_ptr();

    init_descriptor(base, 1, 64);
    write_u8(base, NUM_CHANNELS_OFFSET, (CHANNEL_NUM_MAX + 1) as u8);

    let err = ack_descriptor(base, CHANNEL_NUM_MAX + 1, 64)
        .expect_err("oversized channel table should be rejected");
    assert!(matches!(
        err,
        Error::Descriptor(DescriptorError::InvalidChannelCount { expected, actual })
            if expected == CHANNEL_NUM_MAX + 1 && actual == CHANNEL_NUM_MAX + 1
    ));
}

#[test]
fn descriptor_ack_rejects_invalid_channel_layouts() {
    let cases = [
        (
            0x20,
            (slot::SLOT_HEADER + 64) as u32,
            "slot overlaps descriptor header or channel table",
        ),
        (0x200, 8, "slot is smaller than the slot header"),
        (
            DESCRIPTOR_LENGTH as u32,
            (slot::SLOT_HEADER + 64) as u32,
            "slot offset does not match aligned layout",
        ),
        (
            slot_offset(0, 64) as u32,
            (slot::SLOT_HEADER + 32) as u32,
            "slot size does not match transfer size",
        ),
    ];

    for (offset, size, reason) in cases {
        let mut backing = vec![0; DESCRIPTOR_LEN];
        let base = backing.as_mut_ptr();
        let entry = channel_table_entry_offset(0);
        init_descriptor(base, 1, 64);
        write_u32(base, entry, offset);
        write_u32(base, entry + 4, size);

        let err =
            ack_descriptor(base, 1, 64).expect_err("invalid channel layout should be rejected");
        assert!(matches!(
            err,
            Error::Descriptor(DescriptorError::InvalidChannel {
                index: 0,
                reason: got
            }) if got == reason
        ));
    }
}

#[test]
fn descriptor_ack_rejects_unrecognized_channel_direction() {
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base = backing.as_mut_ptr();

    init_descriptor(base, 1, 64);
    write_u8(base, channel_table_entry_offset(0) + 8, 0x42);

    let err =
        ack_descriptor(base, 1, 64).expect_err("unrecognized channel direction should be rejected");
    assert!(matches!(
        err,
        Error::Descriptor(DescriptorError::InvalidChannel {
            index: 0,
            reason: "direction is unrecognized"
        })
    ));
}

#[test]
fn descriptor_complete_returns_rx_transfer_size_when_ready() {
    let mut backing = vec![0; 0x20];
    let base = backing.as_mut_ptr();

    write_u32(base, RX_TRANSFER_SIZE_OFFSET, 256);
    unsafe {
        set_state(base, DescState::Ready);
    }

    assert_eq!(unsafe { descriptor_complete(base) }, 256);
}

#[test]
#[should_panic(expected = "descriptor not in Ready state")]
fn descriptor_complete_panics_in_debug_when_not_ready() {
    let mut backing = vec![0; 0x20];
    let base = backing.as_mut_ptr();

    write_u32(base, RX_TRANSFER_SIZE_OFFSET, 256);
    unsafe {
        set_state(base, DescState::MReady);
        descriptor_complete(base);
    }
}

#[test]
fn channel_direction_parses_known_and_unknown_values() {
    let mut backing = vec![0; 12];
    let base = backing.as_mut_ptr();

    write_u32(base, 0, 0x200);
    write_u32(base, 4, 0x80);
    unsafe {
        base.add(8)
            .write_volatile(ChannelDirection::SecondaryToPrimary as u8);
    }

    let parsed = unsafe { ChannelLayout::from_raw_ptr(base) };
    assert_eq!(parsed.offset, 0x200);
    assert_eq!(parsed.size, 0x80);
    assert_eq!(parsed.direction, ChannelDirection::SecondaryToPrimary);

    unsafe {
        base.add(8).write_volatile(0x42);
    }

    let parsed = unsafe { ChannelLayout::from_raw_ptr(base) };
    assert_eq!(parsed.direction, ChannelDirection::Unrecognized);
}

#[test]
fn channel_table_offsets_cover_the_valid_channel_range() {
    assert_eq!(channel_table_entry_offset(0), 0x20);
    assert_eq!(channel_table_entry_offset(15), 0x20 + 15 * 12);
}

#[test]
#[should_panic(expected = "channel index out of range")]
fn channel_table_offset_rejects_out_of_range_channel() {
    let _ = channel_table_entry_offset(16);
}
