use log::info;
use uefi::Error;
use uefi::Status;
use core::ffi::c_void;
use uefi_raw::table::system::SystemTable;
use crate::rhandler::RusbError::*;
use crate::rcore;
pub type ProtocolLabel = &'static str;

pub const SIMPLE_FS: ProtocolLabel = "SIMPLE_FILE_SYSTEM";
pub const DEVICE_PATH: ProtocolLabel = "DEVICE_PATH";
pub const GRAPHICS_OUTPUT: ProtocolLabel = "GRAPHICS_OUTPUT";


pub enum RusbError{
	OpenProtocol(ProtocolLabel,Error),
	UnscopeError(ProtocolLabel),
	OpenFileError(Error),
	StartError(Error),
	LoadError(Error),
	NotRegular
}

impl rcore::Rusb<'_>{
	pub unsafe fn handle(&mut self, flag: RusbError){
		match flag{
			OpenProtocol(label,e) => self.write(&format!("error ({:?}): couldn't open {} protocol", e, label)),
			UnscopeError(label) => self.write(&format!("error: couldn't unscope {} protocol", label)),
			OpenFileError(e) => self.write(&format!("error ({:?}): couldn't open file", e)),
			StartError(e) => self.write(&format!("error ({:?}): couldn't start file", e)),
			LoadError(e) => self.write(&format!("error ({:?}): couldn't load file", e)),
			NotRegular => self.write(&format!("error: not a regular file")),
		}
		self.press_key_exit();
	}	
	pub fn press_key_exit(&mut self) -> Status{
	    unsafe{
    		self.write("press any key to return to UEFI shell");
        	let mut out: usize = 0;
        	let mut e = (*(self.system_table.stdin)).wait_for_key;
        	((*self.system_table.boot_services).wait_for_event)(1,&mut e as *mut *mut c_void, &mut out as *mut usize);
			uefi::boot::exit(
				self.im_handle,
				Status::ABORTED,
				0,
				0x0 as *mut uefi::data_types::chars::Char16
			);
    	}   
    	Status::ABORTED
	}
}
