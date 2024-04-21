use std::mem;

use metal::URL;

use crate::{Device, Error};

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IOCompressionMethod {
    Zlib = 0,
    LZFSE = 1,
    LZ4 = 2,
    LZMA = 3,
    LZBitmap = 4,
}

pub struct IOFileHandle {
    handle: metal::IOFileHandle,
}

impl IOFileHandle {
    pub fn new(device: &Device, path: &str) -> Result<Self, Error> {
        let url = URL::new_with_path(path);
        let handle = device.mtl_device().new_io_file_handle(&url)?;

        Ok(Self { handle })
    }

    pub fn new_with_compression(
        device: &Device,
        path: &str,
        compression: IOCompressionMethod,
    ) -> Result<Self, Error> {
        let url = URL::new_with_path(path);

        let handle = device
            .mtl_device()
            .new_io_file_handle_with_compression(&url, unsafe { mem::transmute(compression) })?;

        Ok(Self { handle })
    }

    #[inline]
    pub fn mtl_io_file_handle(&self) -> &metal::IOFileHandle {
        &self.handle
    }
}
