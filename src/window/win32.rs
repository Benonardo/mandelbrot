#![expect(clippy::unnecessary_wraps, clippy::unused_self)]

use std::{
    error::Error,
    ffi::{CStr, c_char, c_int, c_long, c_uint, c_void},
    fmt::{Display, Formatter},
    mem::MaybeUninit,
};

use crate::math::Vec2;

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy)]
struct Handle(*mut c_void);
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct Atom(i16);
type WndProc = unsafe extern "system" fn(Handle, c_uint, usize, usize) -> usize;
#[repr(C)]
struct WndClass {
    style: c_uint,
    wnd_proc: WndProc,
    cb_cls_extra: c_int,
    cb_wnd_extra: c_int,
    instance: Handle,
    icon: Handle,
    cursor: Handle,
    background: Handle,
    menu_name: *const c_char,
    class_name: *const c_char,
}
#[repr(C)]
struct PixelFormatDescriptor {
    size: u16,
    version: u16,
    flags: u32,
    pixel_type: u8,
    color_bits: u8,
    red_bits: u8,
    red_shift: u8,
    green_bits: u8,
    green_shift: u8,
    blue_bits: u8,
    blue_shift: u8,
    alpha_bits: u8,
    alpha_shift: u8,
    accum_bits: u8,
    accum_red_bits: u8,
    accum_green_bits: u8,
    accum_blue_bits: u8,
    accum_alpha_bits: u8,
    depth_bits: u8,
    stencil_bits: u8,
    aux_buffers: u8,
    layer_type: u8,
    reserved: u8,
    layer_mask: u32,
    visible_mask: u32,
    damage_mask: u32,
}
#[repr(C)]
struct Point {
    x: c_long,
    y: c_long,
}
#[repr(C)]
struct Msg {
    wnd: Handle,
    message: c_uint,
    wparam: usize,
    lparam: usize,
    time: u32,
    pt: Point,
    lprivate: u32,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    #[link_name = "GetModuleHandleA"]
    fn get_module_handle(module_name: *const c_char) -> Handle;

    #[link_name = "LoadLibraryA"]
    fn load_library(lib_file_name: *const c_char) -> Handle;

    #[link_name = "GetProcAddress"]
    fn get_proc_address(module: Handle, proc_name: *const c_char) -> *mut c_void;
}

#[link(name = "user32")]
unsafe extern "system" {
    #[link_name = "RegisterClassA"]
    fn register_class(wnd_class: *const WndClass) -> Atom;

    #[link_name = "CreateWindowExA"]
    fn create_window_ex(
        ex_style: i32,
        class_name: *const c_char,
        window_name: *const c_char,
        style: i32,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        wnd_parent: Handle,
        menu: Handle,
        instance: Handle,
        param: *mut c_void,
    ) -> Handle;

    #[link_name = "ShowWindow"]
    fn show_window(wnd: Handle, cmd_show: c_int);

    #[link_name = "GetDC"]
    fn get_dc(wnd: Handle) -> Handle;

    #[link_name = "DefWindowProcA"]
    fn def_window_proc(wnd: Handle, msg: c_uint, wparam: usize, lparam: usize) -> usize;

    #[link_name = "GetMessageA"]
    fn get_message(
        msg: *mut Msg,
        wnd: Handle,
        msg_filter_min: c_uint,
        msg_filter_max: c_uint,
    ) -> c_int;

    #[link_name = "DispatchMessageA"]
    fn dispatch_message(msg: *mut Msg) -> usize;

    #[link_name = "GetWindowLongA"]
    fn get_window_long(wnd: Handle, index: c_int) -> c_long;

    #[link_name = "SetWindowLongA"]
    fn set_window_long(wnd: Handle, index: c_int, new_long: c_long) -> c_long;
}

#[link(name = "gdi32")]
unsafe extern "system" {
    #[link_name = "ChoosePixelFormat"]
    fn choose_pixel_format(dc: Handle, pfd: *const PixelFormatDescriptor) -> c_int;

    #[link_name = "SetPixelFormat"]
    fn set_pixel_format(dc: Handle, format: c_int, pfd: *const PixelFormatDescriptor) -> c_int;

    #[link_name = "SwapBuffers"]
    fn swap_buffers(dc: Handle) -> c_int;
}

#[link(name = "opengl32")]
unsafe extern "system" {
    #[link_name = "wglCreateContext"]
    fn wgl_create_context(dc: Handle) -> Handle;

    #[link_name = "wglMakeCurrent"]
    fn wgl_make_current(dc: Handle, glrc: Handle) -> c_int;

    #[link_name = "wglGetProcAddress"]
    fn wgl_get_proc_address(name: *const c_char) -> *mut c_void;

    #[link_name = "glViewport"]
    safe fn gl_viewport(x: i32, y: i32, width: i32, height: i32);
}

static PIXEL_FORMAT: PixelFormatDescriptor = PixelFormatDescriptor {
    size: std::mem::size_of::<PixelFormatDescriptor>() as u16,
    version: 1,
    flags: 0x25,
    pixel_type: 0,
    color_bits: 24,
    red_bits: 8,
    red_shift: 0,
    green_bits: 8,
    green_shift: 0,
    blue_bits: 8,
    blue_shift: 0,
    alpha_bits: 0,
    alpha_shift: 0,
    accum_bits: 0,
    accum_red_bits: 0,
    accum_green_bits: 0,
    accum_blue_bits: 0,
    accum_alpha_bits: 0,
    depth_bits: 0,
    stencil_bits: 0,
    aux_buffers: 0,
    layer_type: 0,
    reserved: 0,
    layer_mask: 0,
    visible_mask: 0,
    damage_mask: 0,
};

macro_rules! window_data {
    (
        struct $struct:ident {
            $($field:ident: $type:ty [$set:ident]),*
        }
    ) => {
        struct $struct {
            $($field: $type),*
        }

        #[allow(unnecessary_transmutes)]
        impl $struct {
            $(
                fn $field(wnd: Handle) -> $type {
                    unsafe {
                        std::mem::transmute::<c_long, $type>(get_window_long(
                            wnd,
                            std::mem::offset_of!($struct, $field) as i32,
                        ))
                    }
                }

                fn $set(wnd: Handle, $field: $type) {
                    unsafe {
                        set_window_long(wnd, std::mem::offset_of!($struct, $field) as i32, std::mem::transmute::<$type, c_long>($field));
                    }
                }
            )*
        }
    };
}

window_data! {
    struct WindowData {
        button_keys: u32 [set_button_keys],
        pointer_position: Vec2<u16> [set_pointer_position],
        scroll: f32 [set_scroll]
    }
}

unsafe extern "system" fn wnd_proc(
    wnd: Handle,
    msg: c_uint,
    wparam: usize,
    lparam: usize,
) -> usize {
    match msg {
        0x2 => std::process::exit(0),
        0x5 => {
            let width = (lparam & 0xFFFF) as u16;
            let height = ((lparam >> 16) & 0xFFFF) as u16;
            gl_viewport(0, 0, i32::from(width), i32::from(height));
            0
        }
        0x100 => match wparam {
            0x0D => {
                WindowData::set_button_keys(wnd, WindowData::button_keys(wnd) | 0b10);
                0
            }
            0x26 => {
                WindowData::set_button_keys(wnd, WindowData::button_keys(wnd) | 0b100);
                0
            }
            0x28 => {
                WindowData::set_button_keys(wnd, WindowData::button_keys(wnd) | 0b1000);
                0
            }
            _ => unsafe { def_window_proc(wnd, msg, wparam, lparam) },
        },
        0x101 => match wparam {
            0x0D => {
                WindowData::set_button_keys(wnd, WindowData::button_keys(wnd) & !0b10);
                0
            }
            0x26 => {
                WindowData::set_button_keys(wnd, WindowData::button_keys(wnd) & !0b100);
                0
            }
            0x28 => {
                WindowData::set_button_keys(wnd, WindowData::button_keys(wnd) & !0b1000);
                0
            }
            _ => unsafe { def_window_proc(wnd, msg, wparam, lparam) },
        },
        0x200 => {
            let x = (lparam & 0xFFFF) as u16;
            let y = ((lparam >> 16) & 0xFFFF) as u16;
            WindowData::set_pointer_position(wnd, Vec2::new(x, y));
            0
        }
        0x201 => {
            WindowData::set_button_keys(wnd, WindowData::button_keys(wnd) | 0b1);
            0
        }
        0x202 => {
            WindowData::set_button_keys(wnd, WindowData::button_keys(wnd) & !0b1);
            0
        }
        0x20A => {
            let scroll = ((wparam >> 16) & 0xFFFF) as i16;
            WindowData::set_scroll(wnd, WindowData::scroll(wnd) + f32::from(scroll) / 120.0);
            0
        }
        _ => unsafe { def_window_proc(wnd, msg, wparam, lparam) },
    }
}

pub struct Window {
    wnd: Handle,
    dc: Handle,
}

impl Window {
    pub fn create() -> Result<Self, WindowCreateError> {
        const WINDOW_CLASS: &CStr = c"Mandelbrot Window Class";

        unsafe {
            let instance = get_module_handle(std::ptr::null());

            let wnd_class = WndClass {
                style: 0,
                wnd_proc,
                cb_cls_extra: 0,
                cb_wnd_extra: std::mem::size_of::<WindowData>() as i32,
                instance,
                icon: Handle::default(),
                cursor: Handle::default(),
                background: Handle::default(),
                menu_name: std::ptr::null(),
                class_name: WINDOW_CLASS.as_ptr(),
            };
            register_class(&raw const wnd_class);

            let wnd = create_window_ex(
                0,
                WINDOW_CLASS.as_ptr(),
                c"Mandelbrot".as_ptr(),
                0xCF_0000,
                i32::MIN,
                i32::MIN,
                i32::MIN,
                i32::MIN,
                Handle::default(),
                Handle::default(),
                instance,
                std::ptr::null_mut(),
            );
            show_window(wnd, 1);

            let dc = get_dc(wnd);
            let format = choose_pixel_format(dc, &raw const PIXEL_FORMAT);
            set_pixel_format(dc, format, &raw const PIXEL_FORMAT);

            let glrc = wgl_create_context(dc);
            wgl_make_current(dc, glrc);

            Ok(Self { wnd, dc })
        }
    }

    pub unsafe fn load_fn(name: &CStr) -> *mut c_void {
        unsafe {
            let opengl = load_library(c"Opengl32.dll".as_ptr());
            let proc = get_proc_address(opengl, name.as_ptr());
            if proc.is_null() {
                wgl_get_proc_address(name.as_ptr())
            } else {
                proc
            }
        }
    }

    pub fn swap_buffers(&self) -> Result<(), BufferSwapError> {
        if unsafe { swap_buffers(self.dc) } == 0 {
            Err(BufferSwapError)
        } else {
            Ok(())
        }
    }

    pub fn check_events(&self) -> Result<(), EventCheckError> {
        let mut msg = MaybeUninit::uninit();
        unsafe {
            get_message(msg.as_mut_ptr(), self.wnd, 0, 0);
            dispatch_message(msg.as_mut_ptr());
        }
        Ok(())
    }

    pub fn pointer_position(&self) -> Vec2<f32> {
        let position = WindowData::pointer_position(self.wnd);
        Vec2::new(f32::from(position.x), f32::from(position.y))
    }

    pub fn total_scroll(&self) -> f64 {
        f64::from(WindowData::scroll(self.wnd))
    }

    pub fn left_button(&self) -> bool {
        WindowData::button_keys(self.wnd) & 0b1 != 0
    }

    pub fn enter_key(&self) -> bool {
        WindowData::button_keys(self.wnd) & 0b10 != 0
    }

    pub fn up_key(&self) -> bool {
        WindowData::button_keys(self.wnd) & 0b100 != 0
    }

    pub fn down_key(&self) -> bool {
        WindowData::button_keys(self.wnd) & 0b1000 != 0
    }
}

#[derive(Debug)]
pub enum WindowCreateError {}

impl Display for WindowCreateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {}
    }
}

impl Error for WindowCreateError {}

#[derive(Debug)]
pub struct BufferSwapError;

impl Display for BufferSwapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("BufferSwap returned false")
    }
}

impl Error for BufferSwapError {}

#[derive(Debug)]
pub enum EventCheckError {}

impl Display for EventCheckError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {}
    }
}

impl Error for EventCheckError {}
