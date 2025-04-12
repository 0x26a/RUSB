#![no_main]
#![no_std]
mod rcore;
mod rboot;
mod rconfig;
mod rhandler;
mod rgraphics;
mod uefi_alloc;

use uefi::entry;
use uefi::Status;
use embedded_graphics::mono_font::ascii;

#[macro_use]
extern crate alloc;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    let system_table = unsafe{ uefi::table::system_table_raw().expect("error: couldn't fetch system table").read() };
    unsafe{
        let mut rusb = rcore::Rusb::new(&system_table, ascii::FONT_9X15);
        rusb.draw_image();
        rusb.show_system_details();
        rusb.boot_efi();
        return rusb.press_key_exit();
    }
}
