use std::sync::mpsc;

use consortium_dbg_defmt::descriptor::{DESCRIPTOR_SIZE, Descriptor, Flags};

const REGION_LEN: usize = 256;

fn init_region() -> Box<[u8]> {
    let mut region = vec![0; REGION_LEN].into_boxed_slice();
    let base = region.as_mut_ptr();
    unsafe {
        Descriptor::new(REGION_LEN as u16).write_to_raw_ptr(base);
    }
    region
}

#[test]
fn descriptor_initializes_data_pointers_after_header() {
    let region = init_region();
    let descriptor = unsafe { Descriptor::from_raw_ptr(region.as_ptr()) };

    assert!(descriptor.check());
    assert_eq!(descriptor.capacity, REGION_LEN as u16);
    assert_eq!(descriptor.read_ptr, DESCRIPTOR_SIZE as u16);
    assert_eq!(descriptor.write_ptr, DESCRIPTOR_SIZE as u16);
    assert!(descriptor.flags.is_empty());
}

#[test]
fn descriptor_writes_and_reads_one_frame_from_heap_region() {
    let mut region = init_region();
    let base = region.as_mut_ptr();

    unsafe {
        let writer = &mut *(base as *mut Descriptor);
        assert!(writer.write_one_frame(base, b"hello"));

        let mut reader = Descriptor::from_raw_ptr(base);
        let frames = reader.read_bulk_frames(base);

        assert_eq!(frames, vec![b"hello".to_vec()]);
    }
}

#[test]
fn descriptor_streams_frames_between_two_host_threads() {
    let mut region = init_region();
    let base_addr = region.as_mut_ptr() as usize;
    let _region = Box::leak(region);
    let (written_tx, written_rx) = mpsc::channel();

    let producer = std::thread::spawn(move || {
        let base = base_addr as *mut u8;
        unsafe {
            let writer = &mut *(base as *mut Descriptor);
            assert!(writer.write_one_frame(base, b"alpha"));
            assert!(writer.write_one_frame(base, b"beta"));
            assert!(writer.write_one_frame(base, b"gamma"));
        }
        written_tx.send(()).expect("signal written frames");
    });

    let consumer = std::thread::spawn(move || {
        written_rx.recv().expect("wait for producer");
        let base = base_addr as *mut u8;
        unsafe {
            let mut reader = Descriptor::from_raw_ptr(base);
            reader.read_bulk_frames(base)
        }
    });

    producer.join().expect("producer completes");
    let frames = consumer.join().expect("consumer completes");

    assert_eq!(
        frames,
        vec![b"alpha".to_vec(), b"beta".to_vec(), b"gamma".to_vec()]
    );
}

#[test]
fn descriptor_sets_overflow_when_frame_cannot_fit() {
    let mut region = vec![0; DESCRIPTOR_SIZE + 8].into_boxed_slice();
    let base = region.as_mut_ptr();

    unsafe {
        Descriptor::new(region.len() as u16).write_to_raw_ptr(base);
        let writer = &mut *(base as *mut Descriptor);

        assert!(!writer.write_one_frame(base, b"this frame is too large"));
        assert!(writer.flags.contains(Flags::Overflow));
    }
}
