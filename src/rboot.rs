use crate::rhandler::{RusbError::*, SIMPLE_FS, DEVICE_PATH, GRAPHICS_OUTPUT};
use crate::rcore;
use crate::rconfig::EFI_APP_PATH;

use log::info;
use core::ffi::c_void;
use uefi::boot::*;
use uefi::Identify;
use uefi::prelude::*;
use uefi::{guid, Guid};
use uefi::proto::media::file::File;
use uefi::proto::device_path::DevicePath;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi_raw::table::system::SystemTable;
use uefi_raw::protocol::loaded_image::LoadedImageProtocol;



impl rcore::Rusb<'_>{

    pub unsafe fn load_efi(&mut self, mut dir: uefi::proto::media::file::Directory, handle: Handle, dp_ref: &DevicePath){
        let mut buffer: [u8;300000] = [0;300000];
        self.read_file_from_path(EFI_APP_PATH, dir, &mut buffer);
        self.close_protocol(handle);
        match load_image(
            self.im_handle,
            LoadImageSource::FromBuffer{
                buffer: &buffer,
                file_path: Some(dp_ref)
            }
        ){
            Ok(loaded_image_handle) => {
                self.write("");
                self.write(&format!("loaded EFI image {}", EFI_APP_PATH));
                self.write(&format!("starting loaded image in 3s"));
                boot::stall(3_000_000);
                loop{}
                match start_image(loaded_image_handle){
                    Err(e) => self.handle(StartError(e)),
                    _ => ()
                };
            },
            Err(e) => self.handle(LoadError(e))
        };

    }
    pub unsafe fn boot_efi(&mut self){
        let handle_buffer = locate_handle_buffer(SearchType::ByProtocol(&SimpleFileSystem::GUID)).unwrap();
        self.write(&format!("SIMPLE_FILE_SYSTEM compatible handles found: {}", handle_buffer.len()));
        for handle in handle_buffer.iter(){
            match open_protocol_exclusive::<uefi::proto::media::fs::SimpleFileSystem>(*handle){
                Ok(mut scoped_sfs) => {
                    self.write(&format!("retrieved protocol for {:?}", handle));
                    match scoped_sfs.get_mut(){
                        Some(fs_ref) => {
                            match open_protocol_exclusive::<DevicePath>(*handle){
                                Ok(scoped_dp) =>{
                                    match scoped_dp.get(){
                                        Some(dp_ref) => {
                                            let dir = fs_ref.open_volume().unwrap();

                                            self.load_efi(dir, *handle, dp_ref);

                                        },
                                        _ => self.handle(UnscopeError(DEVICE_PATH))
                                    }
                                },
                                Err(e) => self.handle(OpenProtocol(DEVICE_PATH,e))
                            }

                        }
                        _ => self.handle(UnscopeError(SIMPLE_FS))
                    }

                },
                Err(e) => self.handle(OpenProtocol(SIMPLE_FS,e))
            }
        }
    }

}

