#![no_main]
#![no_std]
mod rcore;
mod rboot;
mod rconfig;
mod rhandler;
mod rgraphics;
mod uefi_alloc;

use rhandler::{RusbError::*, SIMPLE_FS, DEVICE_PATH, GRAPHICS_OUTPUT};
use log::info;
use core::ffi::c_void;
use core::mem::size_of;
use uefi::boot::*;
use uefi::Identify;
use uefi::prelude::*;
use uefi::{guid, Guid};
use uefi::proto::media::file::File;
use uefi::proto::device_path::DevicePath;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi_raw::table::system::SystemTable;
use uefi_raw::table::configuration::ConfigurationTable;
use uefi_raw::protocol::loaded_image::LoadedImageProtocol;
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
