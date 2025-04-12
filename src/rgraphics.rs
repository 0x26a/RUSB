use crate::rcore;
use crate::rhandler::{RusbError::*, GRAPHICS_OUTPUT, SIMPLE_FS};
use crate::uefi_alloc;
use crate::rconfig::{PICTURE_SIZE,PICTURE_PATH,PICTURE_WIDTH};
use alloc::vec::Vec;
use core::alloc::GlobalAlloc;
use core::mem::size_of;
use core::ffi::c_void;
use core::alloc::Layout;
use log::info;
use uefi::{guid, Guid};
use uefi::prelude::*;
use uefi::boot::*;
use uefi_raw::table::system::SystemTable;
use uefi_graphics2::UefiDisplay;
use uefi::proto::console::gop::{GraphicsOutput, ModeInfo};
use uefi::Identify;
use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::pixelcolor::Rgb888;
use embedded_graphics::{
    mono_font::{ascii, iso_8859_1, MonoTextStyle, MonoFont},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
    primitives::{circle::Circle, PrimitiveStyle},
    image::{Image, ImageRaw}
};

const MAX_WIDTH: usize = 2560;
const MAX_HEIGHT: usize = 1600;

impl rcore::Rusb<'_>{
	pub unsafe fn get_uefi_display(system_table: &SystemTable) -> UefiDisplay{
		let uefi_display = get_gfx(system_table).expect("error: no handle supporting GOP");
		return uefi_display;
	}
	pub unsafe fn write(&mut self, msg: &str){
		let style = MonoTextStyle::new(&self.font, Rgb888::new(250,0,250));
		Text::new(msg, Point::new((PICTURE_WIDTH as i32) + 20, self.line), style).draw(&mut self.uefi_display).unwrap();
		self.line += 14;
		self.swap();
	}
	pub unsafe fn draw_image(&mut self){
		let memory_layout = Layout::from_size_align(PICTURE_SIZE,1).unwrap();
		let a = uefi_alloc::Allocator;
		let ptr = a.alloc(memory_layout);
		let buffer_ref_mut = core::slice::from_raw_parts_mut(ptr, PICTURE_SIZE);
		match get_image_file_system(self.im_handle){
			Ok(mut scoped_sfs) => {
				match scoped_sfs.get_mut(){
					Some(fs_ref) => {
						let dir = fs_ref.open_volume().expect("error: couldn't open host volume");
						self.read_file_from_path(PICTURE_PATH, dir, buffer_ref_mut);
					},
					_ => self.handle(UnscopeError(SIMPLE_FS))
				}	
			},
			Err(e) => self.handle(OpenProtocol(SIMPLE_FS, e))
		}
		let buffer_ref = core::slice::from_raw_parts(ptr, PICTURE_SIZE);
    	let raw_image = ImageRaw::<Rgb888>::new(buffer_ref,PICTURE_WIDTH);
		let image = Image::with_center(&raw_image, Point::new(360,200));
		image.draw(&mut self.uefi_display).unwrap();
		a.dealloc(ptr,memory_layout);
		self.swap();  
	}

	pub unsafe fn show_system_details(&mut self){
		let mut name: Vec<u8> = vec![];
    	let mut ptr = self.system_table.firmware_vendor;
    	for i in 0..99{
    		let b = unsafe{ *ptr } as u8;
    		if b == 0{
    			break;
    		}
    		name.push(b);
    	    ptr = ((ptr as usize) + 2) as *const u16;
    	}
    	let n = core::str::from_utf8(&name).unwrap();
    	self.write("[ RUSB - RUSB Unveils Safe Booting ]");
    	self.write("");
    	self.write(&format!("Firmware Vendor: {}", n));
    	self.write(&format!("Firmware Revision: {}", self.system_table.firmware_revision));
    	self.write(&format!("System Table: 0x{:x}", (self.system_table as *const SystemTable) as usize));
    	self.write(&format!("Boot Services: 0x{:x}", self.system_table.boot_services as usize));
    	self.write(&format!("Runtime Services: 0x{:x}", self.system_table.runtime_services as usize));
    	self.write(&format!("Configuration Table entries: {}", self.system_table.number_of_configuration_table_entries));
    	self.write("");
	}

	pub unsafe fn clear(&mut self){
		self.uefi_display.clear(Rgb888::new(0,0,0));
		self.swap();
	}
	pub unsafe fn swap(&mut self){
		self.uefi_display.flush();
	}
}

unsafe fn get_gfx(system_table: &SystemTable) -> Option<UefiDisplay>{
	let mut current_width: usize = 0;
	let mut current_height: usize = 0;
	let mut mode_info: Option<ModeInfo> = None;
	let mut uefi_display: Option<UefiDisplay> = None;

    let handle_buffer = locate_handle_buffer(SearchType::ByProtocol(&GraphicsOutput::GUID)).unwrap();
    for handle in handle_buffer.iter(){
		match open_protocol::<GraphicsOutput>(
    		OpenProtocolParams{
    			handle: *handle,
    			agent: image_handle(),
    			controller: None
    		},
    		OpenProtocolAttributes::GetProtocol
    	){
    		Ok(mut scoped_gop) => {
    			let gop_ref = scoped_gop.get_mut().expect("error: couldn't retrieve scoped GOP");
    			let mode = gop_ref.current_mode_info();
    			for mode in gop_ref.modes(){
    				let m = mode.info();
    				let (x,y): (usize,usize) = m.resolution();
    				if (x >= current_width || y >= current_height) && x <= MAX_WIDTH && y <= MAX_HEIGHT{
    					current_width = x;
    					current_height = y;
    					mode_info = Some(*m);
    				}
    			}
    			let m = mode_info.expect("error: no graphical mode found");
    			uefi_display = Some(UefiDisplay::new(gop_ref.frame_buffer(),m).expect("error: couldn't get frame buffer"));
    		},
    		Err(e) => loop{}
    	}
    }
    return uefi_display;
}
