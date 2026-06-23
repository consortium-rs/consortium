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

        // SAFETY: `base` points to the leaked heap-backed test region.
        unsafe { ConsortiumLogger::acquire_shared_memory(base, REGION_LEN as u16) };

        // SAFETY: the producer initialized the descriptor at `base` above.
        let descriptor = unsafe { Descriptor::from_raw_ptr(base) };
        assert!(descriptor.check());
        assert_eq!(descriptor.capacity, REGION_LEN as u16);
        assert_eq!(descriptor.read_ptr, DESCRIPTOR_SIZE as u16);
        assert_eq!(descriptor.write_ptr, DESCRIPTOR_SIZE as u16);
        assert!(descriptor.flags.contains(Flags::Writing));

        // SAFETY: `base` points to the initialized debug ring.
        unsafe { ConsortiumLogger::write_shared_memory(base, b"alpha") };
        // SAFETY: `base` points to the initialized debug ring.
        unsafe { ConsortiumLogger::write_shared_memory(base, b"beta") };
        // SAFETY: `base` points to the initialized debug ring.
        unsafe { ConsortiumLogger::write_shared_memory(base, b"gamma") };
        // SAFETY: `base` points to the initialized debug ring.
        unsafe { ConsortiumLogger::release_shared_memory(base) };

        written_tx.send(()).expect("signal written frames");
    });

    let consumer = std::thread::spawn(move || {
        written_rx.recv().expect("wait for producer");
        let base = base_addr as *mut u8;

        // SAFETY: producer initialized the descriptor and signaled completion.
        let descriptor = unsafe { Descriptor::from_raw_ptr(base) };
        assert!(!descriptor.flags.contains(Flags::Writing));
        assert!(descriptor.flags.is_empty());

        // SAFETY: producer initialized the descriptor and signaled completion.
        let mut reader = unsafe { Descriptor::from_raw_ptr(base) };
        // SAFETY: `base` points to the initialized debug ring.
        let frames = unsafe { reader.read_bulk_frames(base) };
        // SAFETY: `base` points to the initialized debug ring.
        unsafe { ConsortiumLogger::flush_shared_memory(base) };
        frames
    });

    producer.join().expect("producer completes");
    let frames = consumer.join().expect("consumer completes");

    assert_eq!(
        frames,
        vec![b"alpha".to_vec(), b"beta".to_vec(), b"gamma".to_vec()]
    );
}
