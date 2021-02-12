use raw_window_handle::windows::WindowsHandle;
use std::ffi::c_void;

use winapi::{
    shared::{
        ntdef::NULL,
        windef::{HBITMAP, HDC, HWND},
    },
    um::{
        wingdi::{
            BitBlt, CreateCompatibleDC, CreateDIBSection, DeleteObject, SelectObject, BITMAPINFO,
            BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, SRCCOPY,
        },
        winuser::{GetDC, ReleaseDC},
    },
};
pub enum Platform {
    Windows(WindowsPlatform),
}

macro_rules! platfrom_fn {
    ($e: ident, $func: ident; $($args: expr), *) => {
        match $e {
            Platform::Windows(windows_platform) => windows_platform.$func($($args, )*),
        }
    };
}

impl Platform {
    pub fn redraw(&self) {
        platfrom_fn!(self, redraw;)
    }

    pub fn write_buffer(&self, buffer: &[u8]) {
        platfrom_fn!(self, write_buffer; buffer)
    }
}

#[allow(dead_code)]
pub struct WindowsPlatform {
    hwnd: HWND,
    memory_dc: HDC,
    buffer: *mut c_void,
    width: i32,
    height: i32,
}

#[allow(dead_code)]
impl WindowsPlatform {
    pub fn new(handle: WindowsHandle, width: u32, height: u32) -> Self {
        let (width, height) = (width as i32, height as i32);
        let hwnd = handle.hwnd;
        let mut buffer = std::ptr::null_mut();
        //get HWND and memery dc
        let (hwnd, memory_dc) = unsafe {
            let hwnd = &mut *(hwnd as HWND);
            let hdc = GetDC(hwnd);
            let memory_dc = CreateCompatibleDC(hdc);
            ReleaseDC(hwnd, hdc);
            (hwnd, memory_dc)
        };

        unsafe {
            let mut bitmap_header = BITMAPINFOHEADER {
                biSize: 40,
                biWidth: width,
                biHeight: -height,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrImportant: 0,
                biClrUsed: 0,
            };
            let bitmap_header_ptr: *mut BITMAPINFOHEADER = &mut bitmap_header;
            let dib_bitmap = CreateDIBSection(
                memory_dc,
                bitmap_header_ptr as *mut BITMAPINFO,
                DIB_RGB_COLORS,
                &mut buffer,
                NULL,
                0,
            );
            assert_ne!(dib_bitmap, NULL as HBITMAP);
            let old_bitmap = SelectObject(memory_dc, dib_bitmap as *mut c_void);
            DeleteObject(old_bitmap);
        };

        WindowsPlatform {
            hwnd,
            memory_dc,
            buffer,
            width,
            height,
        }
    }

    pub fn redraw(&self) {
        let WindowsPlatform {
            hwnd,
            memory_dc,
            width,
            height,
            ..
        } = *self;
        unsafe {
            let windows_dc = GetDC(hwnd);
            BitBlt(windows_dc, 0, 0, width, height, memory_dc, 0, 0, SRCCOPY);
            ReleaseDC(hwnd, windows_dc);
        }
    }

    pub fn write_buffer(&self, new_buffer: &[u8]) {
        unsafe {
            std::ptr::copy(
                new_buffer.as_ptr() as *mut c_void,
                self.buffer,
                new_buffer.len(),
            )
        }
    }
}
