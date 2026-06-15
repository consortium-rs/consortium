use consortium_common::descriptor::{DescriptorHeader, DescriptorHeaderError, DescriptorMagic};
use consortium_ipc_transport_memory::{
    ChannelDirection, ChannelLayout, DescState, Descriptor, DescriptorError, VERSION,
    channel_table_entry_offset, descriptor_ack, descriptor_complete, descriptor_init, get_state,
    set_state,
};
use std::sync::mpsc;

const DESCRIPTOR_LEN: usize = 0x20 + 2 * 12;
const NUM_CHANNELS_OFFSET: usize = 0x06;
const RX_TRANSFER_SIZE_OFFSET: usize = 0x10;
const STATE_OFFSET: usize = 0x18;

fn channel(offset: usize, size: usize, direction: ChannelDirection) -> ChannelLayout {
    let mut channel = ChannelLayout::default();
    channel.offset = offset as u32;
    channel.size = size as u32;
    channel.direction = direction;
    channel
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
    let channels = [
        channel(0x100, 0x80, ChannelDirection::AToM),
        channel(0x180, 0x40, ChannelDirection::MToA),
    ];

    unsafe {
        descriptor_init(base, channels.len() as u8, 96, &channels);
    }

    assert_eq!(backing[4], VERSION);
    assert_eq!(backing[5], 0, "header reserved byte must stay reserved");
    assert_eq!(backing[NUM_CHANNELS_OFFSET], 2);
    assert_eq!(backing[7], 0, "descriptor reserved byte must stay reserved");
    assert_eq!(read_u32(base, RX_TRANSFER_SIZE_OFFSET), 96);
    assert_eq!(read_u32(base, STATE_OFFSET), DescState::MReady as u32);

    let descriptor = unsafe { Descriptor::from_raw_ptr(base) };
    assert_eq!(descriptor.num_channels, 2);
    assert_eq!(descriptor.rx_mtu, 96);
    assert_eq!(descriptor.state, DescState::MReady);
    assert_eq!(descriptor.channels[0].offset, 0x100);
    assert_eq!(descriptor.channels[0].size, 0x80);
    assert_eq!(descriptor.channels[0].direction, ChannelDirection::AToM);
    assert_eq!(descriptor.channels[1].offset, 0x180);
    assert_eq!(descriptor.channels[1].size, 0x40);
    assert_eq!(descriptor.channels[1].direction, ChannelDirection::MToA);
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
    let channels = [
        channel(0x100, 0x80, ChannelDirection::AToM),
        channel(0x180, 0x40, ChannelDirection::MToA),
    ];

    unsafe {
        descriptor_init(base, channels.len() as u8, 64, &channels);
    }

    let descriptor = unsafe { descriptor_ack(base, channels.len(), 128) }
        .expect("descriptor should be ready to acknowledge");

    assert_eq!(descriptor.num_channels, 2);
    assert_eq!(descriptor.state, DescState::Ready);
    assert_eq!(descriptor.tx_mtu, 128);
    assert_eq!(unsafe { get_state(base) }, DescState::Ready);
    assert_eq!(read_u32(base, RX_TRANSFER_SIZE_OFFSET), 128);
    assert_eq!(descriptor.channels[0].direction, ChannelDirection::AToM);
    assert_eq!(descriptor.channels[1].direction, ChannelDirection::MToA);
}

#[test]
fn two_threads_complete_descriptor_handshake() {
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base_addr = backing.as_mut_ptr() as usize;
    let (m_ready_tx, m_ready_rx) = mpsc::channel();
    let (a_ready_tx, a_ready_rx) = mpsc::channel();

    let m_side = std::thread::spawn(move || {
        let base = base_addr as *mut u8;
        let channels = [
            channel(0x100, 0x80, ChannelDirection::AToM),
            channel(0x180, 0x80, ChannelDirection::MToA),
        ];

        unsafe {
            descriptor_init(base, channels.len() as u8, 96, &channels);
        }
        assert_eq!(unsafe { get_state(base) }, DescState::MReady);
        m_ready_tx.send(()).expect("signal descriptor is MReady");

        a_ready_rx.recv().expect("wait for A-side ack");
        let transfer_size = unsafe { descriptor_complete(base) };
        assert_eq!(transfer_size, 128);
        assert_eq!(unsafe { get_state(base) }, DescState::Ready);
    });

    let a_side = std::thread::spawn(move || {
        m_ready_rx.recv().expect("wait for M-side init");

        let base = base_addr as *mut u8;
        let descriptor =
            unsafe { descriptor_ack(base, 2, 128) }.expect("A-side acknowledges descriptor");

        assert_eq!(descriptor.num_channels, 2);
        assert_eq!(descriptor.tx_mtu, 128);
        assert_eq!(descriptor.state, DescState::Ready);
        assert_eq!(unsafe { get_state(base) }, DescState::Ready);
        a_ready_tx.send(()).expect("signal descriptor is Ready");
    });

    m_side.join().expect("M-side handshake completes");
    a_side.join().expect("A-side handshake completes");
}

#[test]
fn descriptor_ack_rejects_bad_magic() {
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base = backing.as_mut_ptr();
    let channels = [channel(0x100, 0x80, ChannelDirection::AToM)];

    unsafe {
        descriptor_init(base, channels.len() as u8, 64, &channels);
        DescriptorHeader::new(DescriptorMagic::DEBUG, VERSION).write_to_raw_ptr(base);
    }

    let err = unsafe { descriptor_ack(base, channels.len(), 64) }
        .expect_err("debug descriptor magic should be rejected for IPC");
    assert!(matches!(
        err,
        DescriptorError::Header(DescriptorHeaderError::BadMagic { .. })
    ));
}

#[test]
fn descriptor_ack_rejects_bad_version() {
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base = backing.as_mut_ptr();
    let channels = [channel(0x100, 0x80, ChannelDirection::AToM)];

    unsafe {
        descriptor_init(base, channels.len() as u8, 64, &channels);
        base.add(4).write_volatile(VERSION + 1);
    }

    let err = unsafe { descriptor_ack(base, channels.len(), 64) }
        .expect_err("descriptor version mismatch should be rejected");
    assert!(matches!(
        err,
        DescriptorError::Header(DescriptorHeaderError::BadVersion(_, _))
    ));
}

#[test]
fn descriptor_ack_rejects_non_mready_state() {
    let mut backing = vec![0; DESCRIPTOR_LEN];
    let base = backing.as_mut_ptr();
    let channels = [channel(0x100, 0x80, ChannelDirection::AToM)];

    unsafe {
        descriptor_init(base, channels.len() as u8, 64, &channels);
        set_state(base, DescState::Proposed);
    }

    let err = unsafe { descriptor_ack(base, channels.len(), 64) }
        .expect_err("descriptor must be MReady before ack");
    assert!(matches!(
        err,
        DescriptorError::NotReady(DescState::Proposed)
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
        base.add(8).write_volatile(ChannelDirection::MToA as u8);
    }

    let parsed = unsafe { ChannelLayout::from_raw_ptr(base) };
    assert_eq!(parsed.offset, 0x200);
    assert_eq!(parsed.size, 0x80);
    assert_eq!(parsed.direction, ChannelDirection::MToA);

    unsafe {
        base.add(8).write_volatile(0x42);
    }

    let parsed = unsafe { ChannelLayout::from_raw_ptr(base) };
    assert_eq!(parsed.direction, ChannelDirection::Unrecognized);
}

#[test]
fn channel_table_offsets_cover_the_valid_channel_range() {
    assert_eq!(channel_table_entry_offset(0), 0x20);
    assert_eq!(channel_table_entry_offset(31), 0x20 + 31 * 12);
}

#[test]
#[should_panic(expected = "channel index out of range")]
fn channel_table_offset_rejects_out_of_range_channel() {
    let _ = channel_table_entry_offset(32);
}
