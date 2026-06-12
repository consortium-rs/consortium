use std::sync::mpsc;

use consortium_dbg_defmt::{
    descriptor::{DESCRIPTOR_SIZE, Descriptor, Flags},
    logger::ConsortiumLogger,
};

const REGION_LEN: usize = 256;

#[test]
fn logger_descriptor_round_trips_frames_over_heap_region_between_host_threads() {
    let backing = vec![0; REGION_LEN].into_boxed_slice();
    let backing = Box::leak(backing);
    let base_addr = backing.as_mut_ptr() as usize;
    let (written_tx, written_rx) = mpsc::channel();

    let producer = std::thread::spawn(move || {
        let base = base_addr as *mut u8;

        unsafe {
            ConsortiumLogger::acquire_shared_memory(base, REGION_LEN as u16);

            let descriptor = Descriptor::from_raw_ptr(base);
            assert!(descriptor.check());
            assert_eq!(descriptor.capacity, REGION_LEN as u16);
            assert_eq!(descriptor.read_ptr, DESCRIPTOR_SIZE as u16);
            assert_eq!(descriptor.write_ptr, DESCRIPTOR_SIZE as u16);
            assert!(descriptor.flags.contains(Flags::Writing));

            ConsortiumLogger::write_shared_memory(base, b"alpha");
            ConsortiumLogger::write_shared_memory(base, b"beta");
            ConsortiumLogger::write_shared_memory(base, b"gamma");
            ConsortiumLogger::release_shared_memory(base);
        }

        written_tx.send(()).expect("signal written frames");
    });

    let consumer = std::thread::spawn(move || {
        written_rx.recv().expect("wait for producer");
        let base = base_addr as *mut u8;

        unsafe {
            let descriptor = Descriptor::from_raw_ptr(base);
            assert!(!descriptor.flags.contains(Flags::Writing));
            assert!(descriptor.flags.is_empty());

            let mut reader = Descriptor::from_raw_ptr(base);
            let frames = reader.read_bulk_frames(base);
            ConsortiumLogger::flush_shared_memory(base);
            frames
        }
    });

    producer.join().expect("producer completes");
    let frames = consumer.join().expect("consumer completes");

    assert_eq!(
        frames,
        vec![b"alpha".to_vec(), b"beta".to_vec(), b"gamma".to_vec()]
    );
}
