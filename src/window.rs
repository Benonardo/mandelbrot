#[cfg(target_os = "linux")]
pub mod wayland;
#[cfg(target_os = "linux")]
use wayland as implementation;

use std::{error::Error, fmt::Display};

use crate::{gl::GL, math::Vec2};

pub struct Window(implementation::Window);

impl Window {
    #[allow(clippy::type_complexity)]
    pub fn create() -> Result<Self, WindowCreateError> {
        implementation::Window::create()
            .map(Self)
            .map_err(WindowCreateError)
    }

    #[allow(clippy::unused_self)]
    pub fn load_gl(&self) -> GL {
        unsafe { GL::initialize(implementation::Window::load_fn) }
    }

    pub fn swap_buffers(&self) -> Result<(), BufferSwapError> {
        self.0.swap_buffers().map_err(BufferSwapError)
    }

    pub fn check_events(&self) -> Result<(), EventCheckError> {
        self.0.check_events().map_err(EventCheckError)
    }

    pub fn pointer_position(&self) -> Vec2 {
        self.0.pointer_position()
    }

    pub fn total_scroll(&self) -> f64 {
        self.0.total_scroll()
    }

    pub fn left_button(&self) -> bool {
        self.0.left_button()
    }

    pub fn enter_key(&self) -> bool {
        self.0.enter_key()
    }

    pub fn up_key(&self) -> bool {
        self.0.up_key()
    }

    pub fn down_key(&self) -> bool {
        self.0.down_key()
    }
}

#[derive(Debug)]
pub struct WindowCreateError(pub implementation::WindowCreateError);
impl Display for WindowCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("window creation failed")
    }
}
impl Error for WindowCreateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

#[derive(Debug)]
pub struct BufferSwapError(pub implementation::BufferSwapError);
impl Display for BufferSwapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("buffer swap failed")
    }
}
impl Error for BufferSwapError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

#[derive(Debug)]
pub struct EventCheckError(pub implementation::EventCheckError);
impl Display for EventCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("checking for events failed")
    }
}
impl Error for EventCheckError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}
