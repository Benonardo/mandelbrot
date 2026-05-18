use std::{
    ffi::{CStr, c_char, c_void},
    marker::PhantomData,
};

use crate::math::{Vec2, Vec2I};

macro_rules! gl {
    ($($name:expr => $safety:tt $function:ident($($param:ident: $type:ty),*) $(-> $return:ty)?)*) => {
        #[derive(Debug)]
        pub struct GL {
            $(
                $function: gl_field!($safety $function($($param: $type),*) $(-> $return)*)
            ),*,
            _sendsync: PhantomData<*mut c_void>
        }
        impl GL {
            #[allow(clippy::missing_transmute_annotations)]
            pub unsafe fn initialize(load: unsafe fn(&CStr) -> *mut c_void) -> Self {
                unsafe {
                    Self {
                        $($function: std::mem::transmute(load($name))),*,
                        _sendsync: PhantomData
                    }
                }
            }

            $(
                gl_function!($safety $function($($param: $type),*) $(-> $return)?);
            )*
        }
    };
}

macro_rules! gl_field {
    (safe $function:ident($($param:ident: $type:ty),*) $(-> $return:ty)?) => {extern "C" fn ($($param: $type),*) $(-> $return)?};
    (unsafe $function:ident($($param:ident: $type:ty),*) $(-> $return:ty)?) => {unsafe extern "C" fn ($($param: $type),*) $(-> $return)?};
}

macro_rules! gl_function {
    (safe $function:ident($($param:ident: $type:ty),*) $(-> $return:ty)?) => {
        #[inline]
        pub fn $function(&self, $($param: $type),*) $(-> $return)? {
            (self.$function)($($param),*)
        }
    };
    (unsafe $function:ident($($param:ident: $type:ty),*) $(-> $return:ty)?) => {
        #[inline]
        pub unsafe fn $function(&self, $($param: $type),*) $(-> $return)? {
            unsafe { (self.$function)($($param),*) }
        }
    };
}

gl! {
    c"glEnable" => safe enable(cap: u32)
    c"glDebugMessageCallback" => unsafe debug_message_callback_unchecked(
        callback: unsafe extern "C" fn(source: u32, r#type: u32, id: u32, severity: u32, length: u32, message: *const c_char, user_param: *const c_void),
        user_param: *const c_void)
    c"glGetIntegerv" => unsafe get_integer_v(pname: u32, data: *mut i32)

    c"glCreateBuffers" => unsafe create_buffers_unchecked(n: u32, buffers: *mut u32)
    c"glNamedBufferData" => unsafe named_buffer_data_unchecked(buffer: u32, size: usize, data: *const c_void, usage: u32)
    c"glCreateVertexArrays" => unsafe create_vertex_arrays_unchecked(n: u32, arrays: *mut u32)
    c"glVertexArrayVertexBuffer" => unsafe vertex_array_vertex_buffer(vaobj: u32, bindingindex: u32, buffer: u32, offset: isize, stride: u32)
    c"glEnableVertexArrayAttrib" => safe enable_vertex_array_attrib(vaobj: u32, index: u32)
    c"glVertexArrayAttribFormat" => safe vertex_array_attrib_format(vaobj: u32, attribindex: u32, size: i32, r#type: u32, normalized: bool, relativeoffset: u32)
    c"glVertexArrayAttribBinding" => safe vertex_array_attrib_binding(vaobj: u32, attribindex: u32, bindingindex: u32)
    c"glVertexArrayElementBuffer" => safe vertex_array_element_buffer(vaoobj: u32, buffer: u32)

    c"glCreateShader" => safe create_shader(shader_type: u32) -> u32
    c"glShaderSource" => unsafe shader_source_unchecked(shader: u32, count: u32, string: *const *const c_char, length: *const i32)
    c"glCompileShader" => safe compile_shader(shader: u32)
    c"glCreateProgram" => safe create_program() -> u32
    c"glAttachShader" => safe attach_shader(program: u32, shader: u32)
    c"glLinkProgram" => safe link_program(program: u32)
    c"glDeleteShader" => safe delete_shader(shader: u32)
    c"glUseProgram" => safe use_program(program: u32)

    c"glGetUniformLocation" => unsafe get_uniform_location_unchecked(program: u32, name: *const c_char) -> i32
    c"glUniform1f" => safe uniform_1f(location: i32, v0: f32)
    c"glUniform2fv" => unsafe uniform_2fv_unchecked(location: i32, count: u32, value: *const f32)
    c"glUniform1i" => safe uniform_1i(location: i32, v0: i32)
    c"glUniform2iv" => unsafe uniform_2iv_unchecked(location: i32, count: u32, value: *const i32)

    c"glBindVertexArray" => safe bind_vertex_array(array: u32)
    c"glDrawElements" => unsafe draw_elements_unchecked(mode: u32, count: u32, r#type: u32, indices: *const c_void)
}

impl GL {
    //pub const ARRAY_BUFFER: u32 = 0x8892;
    pub const DEBUG_OUTPUT: u32 = 0x92E0;
    pub const VIEWPORT: u32 = 0xBA2;
    pub const STATIC_DRAW: u32 = 0x88E4;
    pub const FLOAT: u32 = 0x1406;
    pub const VERTEX_SHADER: u32 = 0x8B31;
    pub const FRAGMENT_SHADER: u32 = 0x8B30;
    pub const TRIANGLES: u32 = 0x4;
    pub const UNSIGNED_BYTE: u32 = 0x1401;

    #[inline]
    pub fn debug_message_callback(
        &self,
        callback: fn(source: u32, r#type: u32, id: u32, severity: u32, message: &str),
    ) {
        unsafe {
            self.debug_message_callback_unchecked(
                Self::debug_message_callback_impl,
                callback as *const c_void,
            );
        }
    }

    unsafe extern "C" fn debug_message_callback_impl(
        source: u32,
        r#type: u32,
        id: u32,
        severity: u32,
        length: u32,
        message: *const c_char,
        user_param: *const c_void,
    ) {
        let user_callback = unsafe {
            std::mem::transmute::<
                *const c_void,
                fn(source: u32, r#type: u32, id: u32, severity: u32, message: &str),
            >(user_param)
        };
        let message_utf8 = String::from_utf8_lossy(unsafe {
            std::slice::from_raw_parts(message.cast(), length as usize)
        });
        user_callback(source, r#type, id, severity, &message_utf8);
    }

    #[inline]
    pub fn get_viewport(&self) -> Vec2I {
        let mut viewport = [0; 4];
        unsafe {
            self.get_integer_v(Self::VIEWPORT, viewport.as_mut_ptr());
        }
        Vec2I::new(viewport[2], viewport[3])
    }

    #[inline]
    pub fn create_buffer(&self) -> u32 {
        let mut buffer = 0;
        unsafe {
            self.create_buffers_unchecked(1, &raw mut buffer);
        }
        buffer
    }

    #[inline]
    pub fn named_buffer_data<T>(&self, buffer: u32, data: &[T], usage: u32) {
        unsafe {
            self.named_buffer_data_unchecked(
                buffer,
                std::mem::size_of_val(data),
                data.as_ptr().cast(),
                usage,
            );
        }
    }

    #[inline]
    pub fn shader_source(&self, shader: u32, source: &str) {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let length = source.len() as i32;
        let string = source.as_ptr().cast();
        unsafe {
            self.shader_source_unchecked(shader, 1, &raw const string, &raw const length);
        }
    }

    #[inline]
    pub fn create_vertex_array(&self) -> u32 {
        let mut vao = 0;
        unsafe {
            self.create_vertex_arrays_unchecked(1, &raw mut vao);
        }
        vao
    }

    #[inline]
    pub fn get_uniform_location(&self, program: u32, name: &CStr) -> i32 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        unsafe {
            self.get_uniform_location_unchecked(program, name.as_ptr())
        }
    }

    #[inline]
    pub fn uniform_2fv(&self, location: i32, value: Vec2) {
        unsafe {
            self.uniform_2fv_unchecked(location, 1, (&raw const value).cast());
        }
    }

    #[inline]
    pub fn uniform_2iv(&self, location: i32, value: Vec2I) {
        unsafe {
            self.uniform_2iv_unchecked(location, 1, (&raw const value).cast());
        }
    }

    #[inline]
    pub fn draw_elements<I: Index, E: Element<Index = I>>(&self, elements: &[E]) {
        unsafe {
            #[allow(clippy::cast_possible_truncation)]
            self.draw_elements_unchecked(
                E::GL_TYPE,
                elements.len() as u32 * E::INDICES,
                I::GL_TYPE,
                std::ptr::null(),
            );
        }
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe trait Element {
    type Index: Index;

    const GL_TYPE: u32;
    const INDICES: u32;
}
#[repr(C)]
pub struct Triangle<I: Index>(pub I, pub I, pub I);
unsafe impl<I: Index> Element for Triangle<I> {
    type Index = I;

    const GL_TYPE: u32 = GL::TRIANGLES;
    const INDICES: u32 = 3;
}

#[allow(clippy::missing_safety_doc)]
pub unsafe trait Index {
    const GL_TYPE: u32;
}
unsafe impl Index for u8 {
    const GL_TYPE: u32 = GL::UNSIGNED_BYTE;
}
