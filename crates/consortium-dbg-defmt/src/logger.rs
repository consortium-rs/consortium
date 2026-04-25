use consortium_common::descriptor::DescriptorHeader;
use defmt::{Logger, global_logger};

use crate::descriptor::{Descriptor, Flags, MAGIC, VERSION};

unsafe extern "C" {
    static CONSORTIUM_DBG_SRAM_BASE: u32;
    static CONSORTIUM_DBG_SRAM_SIZE: u16;
}

#[global_logger]
pub struct ConsortiumLogger;

unsafe impl Logger for ConsortiumLogger {
    fn acquire() {
        let base = unsafe {
            CONSORTIUM_DBG_SRAM_BASE as *const u8
        };
        let size = unsafe {
            CONSORTIUM_DBG_SRAM_SIZE
        };
        let header = unsafe { DescriptorHeader::from_raw_ptr(base) };
        if header.check(MAGIC, VERSION).is_err() {
            let descriptor = Descriptor::new(size);
            unsafe { descriptor.write_to_raw_ptr(base as *mut u8); }
        }
        let descriptor = unsafe { &mut *(base as *mut Descriptor) };
        descriptor.flags.insert(Flags::Writing);
    }

    unsafe fn flush() {
        let base = unsafe {
            CONSORTIUM_DBG_SRAM_BASE as *const u8
        };
        let descriptor = unsafe { &mut *(base as *mut Descriptor) };
        if descriptor.flags.contains(Flags::Error) {
            return;
        }
        unsafe {
            descriptor.flush(base as *mut u8);
        }
    }

    unsafe fn release() {
        let base = unsafe {
            CONSORTIUM_DBG_SRAM_BASE as *const u8
        };
        let descriptor = unsafe { &mut *(base as *mut Descriptor) };
        descriptor.flags.remove(Flags::Writing);
    }

    unsafe fn write(bytes: &[u8]) {
        let base = unsafe {
            CONSORTIUM_DBG_SRAM_BASE as *const u8
        };
        let descriptor = unsafe { &mut *(base as *mut Descriptor) };
        if descriptor.flags.contains(Flags::Error) {
            return;
        }
        unsafe {
            if !descriptor.write_one_frame(base as *mut u8, bytes) {
                descriptor.flags.insert(Flags::Overflow);
            }
        }
    }
}