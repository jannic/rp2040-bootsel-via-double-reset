//! Enable bootsel via double reset.
#![no_std]

use core::{arch::asm, mem::MaybeUninit};

use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use rp2040_hal::Timer;

pub fn probe_double_reset(delay: &mut Timer) {
    #[link_section = ".uninit"]
    static mut FLAG: MaybeUninit<u32> = MaybeUninit::uninit();

    let mut flag: u32;
    unsafe {
        asm!(
            "ldr {flag}, [{addr}]",
            addr  = in(reg) FLAG.as_ptr(),
            flag = out(reg) flag,
        );
    }

    if flag == 0x0B0075E1 {
        unsafe { FLAG.write(0) };
        delay.delay_ms(500);
        // trigger bootsel
        rp2040_hal::rom_data::reset_to_usb_boot(0, 0);
        #[allow(clippy::empty_loop)]
        loop {}
    } else {
        let value = 0x0B0075E1;
        unsafe {
            asm!(
                "str {value}, [{addr}]",
                addr = in(reg) FLAG.as_ptr(),
                value = in(reg) value,
            );
        }
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        delay.delay_ms(500);
        let value = 0;
        unsafe {
            asm!(
                "str {value}, [{addr}]",
                addr = in(reg) FLAG.as_ptr(),
                value = in(reg) value,
            );
        }
    }
}
