use core::ffi::c_void;
use std::ptr;
use winapi::um::winioctl::{CTL_CODE, FILE_WRITE_ACCESS, METHOD_BUFFERED};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{CloseHandle, BOOL, HANDLE, INVALID_HANDLE_VALUE},
        Storage::FileSystem::{
            CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
            FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
        },
        System::IO::DeviceIoControl,
    },
};

fn open() -> HANDLE {
    let path: Vec<u16> = "\\\\.\\Global\\ProcmonDebugLogger"
        .encode_utf16()
        .chain(Some(0))
        .collect();

    let h_device = unsafe {
        CreateFileW(
            PCWSTR(path.as_ptr()),
            FILE_GENERIC_READ | FILE_GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            ptr::null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )
    };

    h_device
}

fn debug(h_device: HANDLE) -> BOOL {
    let mut nb = 0;

    let mut text = "Hello world!";
    let text_ptr: *mut c_void = &mut text as *mut _ as *mut c_void;
    let textlen = text.len() as u32;

    let code = CTL_CODE(0x00009535, 0x81, METHOD_BUFFERED, FILE_WRITE_ACCESS);

    let result = unsafe {
        DeviceIoControl(
            h_device,
            code,
            text_ptr,
            textlen,
            ptr::null_mut(),
            0,
            &mut nb,
            ptr::null_mut(),
        )
    };

    unsafe {
        CloseHandle(h_device);
    }

    result
}

fn main() {
    let h_device = open();
    if h_device != INVALID_HANDLE_VALUE {
        debug(h_device);
    }
}
