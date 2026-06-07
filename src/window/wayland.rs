use std::{
    cell::Cell,
    error::Error,
    ffi::{CStr, c_char, c_int, c_uint, c_void},
    fmt::Display,
    ptr::NonNull,
};

use crate::math::Vec2;

static XDG_SHELL_TYPES: [Option<&'static WlInterface>; 26] = [
    None,
    None,
    None,
    None,
    Some(&XDG_POSITIONER_INTERFACE),
    Some(&XDG_SURFACE_INTERFACE),
    Some(&WL_SURFACE_INTERFACE),
    Some(&XDG_TOPLEVEL_INTERFACE),
    Some(&XDG_POPUP_INTERFACE),
    Some(&XDG_SURFACE_INTERFACE),
    Some(&XDG_POSITIONER_INTERFACE),
    Some(&XDG_TOPLEVEL_INTERFACE),
    Some(&WL_SEAT_INTERFACE),
    None,
    None,
    None,
    Some(&WL_SEAT_INTERFACE),
    None,
    Some(&WL_SEAT_INTERFACE),
    None,
    None,
    Some(&WL_OUTPUT_INTERFACE),
    Some(&WL_SEAT_INTERFACE),
    None,
    Some(&XDG_POSITIONER_INTERFACE),
    None,
];
static XDG_WM_BASE_REQUESTS: [WlMessage; 4] = [
    WlMessage {
        name: c"destroy".as_ptr(),
        signature: c"".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"create_positioner".as_ptr(),
        signature: c"n".as_ptr(),
        types: (&raw const XDG_SHELL_TYPES[4]).cast(),
    },
    WlMessage {
        name: c"get_xdg_surface".as_ptr(),
        signature: c"no".as_ptr(),
        types: (&raw const XDG_SHELL_TYPES[5]).cast(),
    },
    WlMessage {
        name: c"pong".as_ptr(),
        signature: c"u".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
];
static XDG_WM_BASE_EVENTS: [WlMessage; 1] = [WlMessage {
    name: c"ping".as_ptr(),
    signature: c"u".as_ptr(),
    types: XDG_SHELL_TYPES.as_ptr().cast(),
}];
static XDG_WM_BASE_INTERFACE: WlInterface = WlInterface {
    name: c"xdg_wm_base".as_ptr(),
    version: 7,
    method_count: 4,
    methods: XDG_WM_BASE_REQUESTS.as_ptr(),
    event_count: 1,
    events: XDG_WM_BASE_EVENTS.as_ptr(),
};
static XDG_POSITIONER_REQUESTS: [WlMessage; 10] = [
    WlMessage {
        name: c"destroy".as_ptr(),
        signature: c"".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"set_size".as_ptr(),
        signature: c"ii".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"set_anchor_rect".as_ptr(),
        signature: c"iiii".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"set_anchor".as_ptr(),
        signature: c"u".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"set_gravity".as_ptr(),
        signature: c"u".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"set_constraint_adjustment".as_ptr(),
        signature: c"u".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"set_offset".as_ptr(),
        signature: c"ii".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"set_reactive".as_ptr(),
        signature: c"3".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"set_parent_size".as_ptr(),
        signature: c"3ii".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"set_parent_configure".as_ptr(),
        signature: c"3u".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
];
static XDG_POSITIONER_INTERFACE: WlInterface = WlInterface {
    name: c"xdg_positioner".as_ptr(),
    version: 7,
    method_count: 10,
    methods: XDG_POSITIONER_REQUESTS.as_ptr(),
    event_count: 0,
    events: std::ptr::null(),
};
static XDG_SURFACE_REQUESTS: [WlMessage; 5] = [
    WlMessage {
        name: c"destroy".as_ptr(),
        signature: c"".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"get_toplevel".as_ptr(),
        signature: c"n".as_ptr(),
        types: (&raw const XDG_SHELL_TYPES[7]).cast(),
    },
    WlMessage {
        name: c"get_popup".as_ptr(),
        signature: c"n?oo".as_ptr(),
        types: (&raw const XDG_SHELL_TYPES[8]).cast(),
    },
    WlMessage {
        name: c"set_window_geometry".as_ptr(),
        signature: c"iiii".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"ack_configure".as_ptr(),
        signature: c"u".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
];
static XDG_SURFACE_EVENTS: [WlMessage; 1] = [WlMessage {
    name: c"configure".as_ptr(),
    signature: c"u".as_ptr(),
    types: XDG_SHELL_TYPES.as_ptr().cast(),
}];
static XDG_SURFACE_INTERFACE: WlInterface = WlInterface {
    name: c"xdg_surface".as_ptr(),
    version: 7,
    method_count: 5,
    methods: XDG_SURFACE_REQUESTS.as_ptr(),
    event_count: 1,
    events: XDG_SURFACE_EVENTS.as_ptr(),
};
static XDG_TOPLEVEL_REQUESTS: [WlMessage; 14] = [
    WlMessage {
        name: c"destroy".as_ptr(),
        signature: c"".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"set_parent".as_ptr(),
        signature: c"?o".as_ptr(),
        types: (&raw const XDG_SHELL_TYPES[11]).cast(),
    },
    WlMessage {
        name: c"set_title".as_ptr(),
        signature: c"s".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"set_app_id".as_ptr(),
        signature: c"s".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"show_window_menu".as_ptr(),
        signature: c"ouii".as_ptr(),
        types: (&raw const XDG_SHELL_TYPES[12]).cast(),
    },
    WlMessage {
        name: c"move".as_ptr(),
        signature: c"ou".as_ptr(),
        types: (&raw const XDG_SHELL_TYPES[16]).cast(),
    },
    WlMessage {
        name: c"resize".as_ptr(),
        signature: c"ouu".as_ptr(),
        types: (&raw const XDG_SHELL_TYPES[18]).cast(),
    },
    WlMessage {
        name: c"set_max_size".as_ptr(),
        signature: c"ii".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"set_min_size".as_ptr(),
        signature: c"ii".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"set_maximized".as_ptr(),
        signature: c"".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"unset_maximized".as_ptr(),
        signature: c"".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"set_fullscreen".as_ptr(),
        signature: c"?o".as_ptr(),
        types: (&raw const XDG_SHELL_TYPES[21]).cast(),
    },
    WlMessage {
        name: c"unset_fullscreen".as_ptr(),
        signature: c"".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"set_minimized".as_ptr(),
        signature: c"".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
];
static XDG_TOPLEVEL_EVENTS: [WlMessage; 4] = [
    WlMessage {
        name: c"configure".as_ptr(),
        signature: c"iia".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"close".as_ptr(),
        signature: c"".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"configure_bounds".as_ptr(),
        signature: c"4ii".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"wm_capabilities".as_ptr(),
        signature: c"5a".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
];
static XDG_TOPLEVEL_INTERFACE: WlInterface = WlInterface {
    name: c"xdg_toplevel".as_ptr(),
    version: 7,
    method_count: 14,
    methods: XDG_TOPLEVEL_REQUESTS.as_ptr(),
    event_count: 4,
    events: XDG_TOPLEVEL_EVENTS.as_ptr(),
};
static XDG_POPUP_REQUESTS: [WlMessage; 3] = [
    WlMessage {
        name: c"destroy".as_ptr(),
        signature: c"".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"grab".as_ptr(),
        signature: c"ou".as_ptr(),
        types: (&raw const XDG_SHELL_TYPES[22]).cast(),
    },
    WlMessage {
        name: c"reposition".as_ptr(),
        signature: c"3ou".as_ptr(),
        types: (&raw const XDG_SHELL_TYPES[24]).cast(),
    },
];
static XDG_POPUP_EVENTS: [WlMessage; 3] = [
    WlMessage {
        name: c"configure".as_ptr(),
        signature: c"iiii".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"popup_done".as_ptr(),
        signature: c"".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
    WlMessage {
        name: c"repositioned".as_ptr(),
        signature: c"3u".as_ptr(),
        types: XDG_SHELL_TYPES.as_ptr().cast(),
    },
];
static XDG_POPUP_INTERFACE: WlInterface = WlInterface {
    name: c"xdg_popup".as_ptr(),
    version: 7,
    method_count: 3,
    methods: XDG_POPUP_REQUESTS.as_ptr(),
    event_count: 3,
    events: XDG_POPUP_EVENTS.as_ptr(),
};

#[repr(transparent)]
struct WlDisplay(c_void);
#[repr(transparent)]
struct WlProxy(c_void);
#[repr(transparent)]
struct WlRegistry(c_void);
#[repr(transparent)]
struct WlCompositor(c_void);
#[repr(transparent)]
struct WlSurface(c_void);
#[repr(transparent)]
struct XdgWMBase(c_void);
#[repr(transparent)]
struct XdgSurface(c_void);
#[repr(transparent)]
struct XdgToplevel(c_void);
#[repr(transparent)]
struct WlSeat(c_void);
#[repr(transparent)]
struct WlPointer(c_void);
#[repr(transparent)]
struct WlKeyboard(c_void);
#[repr(C)]
struct WlInterface {
    name: *const c_char,
    version: c_int,
    method_count: c_int,
    methods: *const WlMessage,
    event_count: c_int,
    events: *const WlMessage,
}
unsafe impl Sync for WlInterface {}
#[repr(C)]
struct WlMessage {
    name: *const c_char,
    signature: *const c_char,
    types: *const *const WlInterface,
}
unsafe impl Sync for WlMessage {}
#[repr(C)]
struct WlArray {
    size: usize,
    alloc: usize,
    data: *mut c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
struct WlFixed(i32);
impl WlFixed {
    fn to_f64(self) -> f64 {
        f64::from_bits((((1023 + 44) << 52) + (1 << 51) + i64::from(self.0)) as u64)
            - (3u64 << 43) as f64
    }
}
#[repr(C)]
struct WlRegistryListener {
    global: unsafe extern "C" fn(
        data: *mut c_void,
        wl_registry: NonNull<WlRegistry>,
        name: u32,
        interface: *const c_char,
        version: u32,
    ),
    global_remove: unsafe extern "C" fn(data: *mut c_void, wl_registry: *mut WlRegistry, name: u32),
}
#[repr(C)]
struct XdgWMBaseListener {
    ping: unsafe extern "C" fn(data: *mut c_void, wm_base: NonNull<XdgWMBase>, version: u32),
}
#[repr(C)]
struct XdgSurfaceListener {
    configure:
        unsafe extern "C" fn(data: *mut c_void, xdg_surface: NonNull<XdgSurface>, version: u32),
}
#[repr(C)]
struct XdgToplevelListener {
    configure: unsafe extern "C" fn(
        data: *mut c_void,
        xdg_toplevel: NonNull<XdgToplevel>,
        width: i32,
        height: i32,
        states: *mut WlArray,
    ),
    close: unsafe extern "C" fn(data: *mut c_void, xdg_toplevel: NonNull<XdgToplevel>),
    configure_bounds: unsafe extern "C" fn(
        data: *mut c_void,
        xdg_toplevel: NonNull<XdgToplevel>,
        width: i32,
        height: i32,
    ),
    wm_capabilities: unsafe extern "C" fn(
        data: *mut c_void,
        xdg_toplevel: NonNull<XdgToplevel>,
        capabilities: *mut WlArray,
    ),
}
#[repr(C)]
struct PointerListener {
    enter: unsafe extern "C" fn(
        data: *mut c_void,
        pointer: NonNull<WlPointer>,
        serial: u32,
        surface: NonNull<WlSurface>,
        surface_x: WlFixed,
        surface_y: WlFixed,
    ),
    leave: unsafe extern "C" fn(
        data: *mut c_void,
        pointer: NonNull<WlPointer>,
        serial: u32,
        surface: NonNull<WlSurface>,
    ),
    motion: unsafe extern "C" fn(
        data: *mut c_void,
        pointer: NonNull<WlPointer>,
        time: u32,
        surface_x: WlFixed,
        surface_y: WlFixed,
    ),
    button: unsafe extern "C" fn(
        data: *mut c_void,
        pointer: NonNull<WlPointer>,
        serial: u32,
        time: u32,
        button: u32,
        state: u32,
    ),
    axis: unsafe extern "C" fn(
        data: *mut c_void,
        pointer: NonNull<WlPointer>,
        time: u32,
        axis: u32,
        value: WlFixed,
    ),
    frame: unsafe extern "C" fn(data: *mut c_void, pointer: NonNull<WlPointer>),
    axis_source:
        unsafe extern "C" fn(data: *mut c_void, pointer: NonNull<WlPointer>, axis_source: u32),
    axis_stop:
        unsafe extern "C" fn(data: *mut c_void, pointer: NonNull<WlPointer>, time: u32, axis: u32),
    axis_discrete: unsafe extern "C" fn(
        data: *mut c_void,
        pointer: NonNull<WlPointer>,
        axis: u32,
        discrete: i32,
    ),
    axis_value120: unsafe extern "C" fn(
        data: *mut c_void,
        pointer: NonNull<WlPointer>,
        axis: u32,
        value120: i32,
    ),
    axis_relative_direction: unsafe extern "C" fn(
        data: *mut c_void,
        pointer: NonNull<WlPointer>,
        axis: u32,
        direction: u32,
    ),
}
#[repr(C)]
struct KeyboardListener {
    keymap: unsafe extern "C" fn(
        data: *mut c_void,
        keyboard: NonNull<WlKeyboard>,
        format: u32,
        fd: i32,
        size: u32,
    ),
    enter: unsafe extern "C" fn(
        data: *mut c_void,
        keyboard: NonNull<WlKeyboard>,
        serial: u32,
        surface: NonNull<WlSurface>,
        keys: *mut WlArray,
    ),
    leave: unsafe extern "C" fn(
        data: *mut c_void,
        keyboard: NonNull<WlKeyboard>,
        serial: u32,
        surface: NonNull<WlSurface>,
    ),
    key: unsafe extern "C" fn(
        data: *mut c_void,
        keyboard: NonNull<WlKeyboard>,
        serial: u32,
        time: u32,
        key: u32,
        state: u32,
    ),
    modifiers: unsafe extern "C" fn(
        data: *mut c_void,
        keyboard: NonNull<WlKeyboard>,
        serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ),
    repeat_info: unsafe extern "C" fn(
        data: *mut c_void,
        keyboard: NonNull<WlKeyboard>,
        rate: i32,
        delay: i32,
    ),
}

#[link(name = "wayland-client")]
unsafe extern "C" {
    #[link_name = "wl_registry_interface"]
    safe static WL_REGISTRY_INTERFACE: WlInterface;
    #[link_name = "wl_compositor_interface"]
    safe static WL_COMPOSITOR_INTERFACE: WlInterface;
    #[link_name = "wl_surface_interface"]
    safe static WL_SURFACE_INTERFACE: WlInterface;
    #[link_name = "wl_seat_interface"]
    safe static WL_SEAT_INTERFACE: WlInterface;
    #[link_name = "wl_output_interface"]
    safe static WL_OUTPUT_INTERFACE: WlInterface;
    #[link_name = "wl_pointer_interface"]
    safe static WL_POINTER_INTERFACE: WlInterface;
    #[link_name = "wl_keyboard_interface"]
    safe static WL_KEYBOARD_INTERFACE: WlInterface;

    fn wl_display_connect(name: *const c_char) -> *mut WlDisplay;
    fn wl_display_dispatch(display: NonNull<WlDisplay>) -> c_int;
    fn wl_display_roundtrip(display: NonNull<WlDisplay>) -> c_int;
    fn wl_proxy_get_version(proxy: NonNull<WlProxy>) -> u32;
    fn wl_proxy_marshal_flags(
        proxy: NonNull<WlProxy>,
        opcode: u32,
        interface: *const WlInterface,
        version: u32,
        flags: u32,
        ...
    ) -> *mut WlProxy;
    fn wl_proxy_add_listener(
        proxy: NonNull<WlProxy>,
        implementation: *const extern "C" fn(c_void) -> c_void,
        data: *mut c_void,
    );
    fn wl_proxy_destroy(proxy: NonNull<WlProxy>);
    fn wl_display_dispatch_pending(display: NonNull<WlDisplay>) -> c_int;
}
const WL_DISPLAY_GET_REGISTRY: u32 = 1;
const WL_REGISTRY_BIND: u32 = 0;
const WL_COMPOSITOR_CREATE_SURFACE: u32 = 0;
const WL_SURFACE_COMMIT: u32 = 6;
const XDG_WM_BASE_GET_XDG_SURFACE: u32 = 2;
const XDG_WM_BASE_PONG: u32 = 3;
const XDG_SURFACE_GET_TOPLEVEL: u32 = 1;
const XDG_SURFACE_ACK_CONFIGURE: u32 = 4;
const WL_SEAT_GET_POINTER: u32 = 0;
const WL_SEAT_GET_KEYBOARD: u32 = 1;

#[link(name = "GL")]
unsafe extern "C" {
    #[link_name = "glViewport"]
    safe fn gl_viewport(x: i32, y: i32, width: i32, height: i32);
}

#[derive(Default, Debug)]
struct Interfaces {
    compositor: Option<(NonNull<WlCompositor>, u32)>,
    wm_base: Option<(NonNull<XdgWMBase>, u32)>,
    seat: Option<(NonNull<WlSeat>, u32)>,
}

static REGISTRY_LISTENER: WlRegistryListener = WlRegistryListener {
    global: registry_listener_global_listener,
    global_remove: registry_listener_global_remove_listener,
};
static WM_BASE_LISTENER: XdgWMBaseListener = XdgWMBaseListener {
    ping: wm_base_listener_ping_listener,
};
static XDG_SURFACE_LISTENER: XdgSurfaceListener = XdgSurfaceListener {
    configure: xdg_surface_listener_configure_listener,
};
static XDG_TOPLEVEL_LISTENER: XdgToplevelListener = XdgToplevelListener {
    configure: xdg_toplevel_listener_configure_listener,
    close: xdg_toplevel_listener_close_listener,
    configure_bounds: xdg_toplevel_listener_configure_bounds_listener,
    wm_capabilities: xdg_toplevel_listener_wm_capabilities_listener,
};
static POINTER_LISTENER: PointerListener = PointerListener {
    enter: pointer_listener_enter_listener,
    leave: pointer_listener_leave_listener,
    motion: pointer_listener_motion_listener,
    button: pointer_listener_button_listener,
    axis: pointer_listener_axis_listener,
    frame: pointer_listener_frame_listener,
    axis_source: pointer_listener_axis_source_listener,
    axis_stop: pointer_listener_axis_stop_listener,
    axis_discrete: pointer_listener_axis_discrete_listener,
    axis_value120: pointer_listener_axis_value120_listener,
    axis_relative_direction: pointer_listener_axis_relative_direction_listener,
};
static KEYBOARD_LISTENER: KeyboardListener = KeyboardListener {
    keymap: keyboard_listener_keymap_listener,
    enter: keyboard_listener_enter_listener,
    leave: keyboard_listener_leave_listener,
    key: keyboard_listener_key_listener,
    modifiers: keyboard_listener_modifiers_listener,
    repeat_info: keyboard_listener_repeat_info_listener,
};

unsafe extern "C" fn registry_listener_global_listener(
    data: *mut c_void,
    registry: NonNull<WlRegistry>,
    name: u32,
    interface: *const c_char,
    version: u32,
) {
    let objects = unsafe { &mut *data.cast::<Interfaces>() };
    match unsafe { CStr::from_ptr(interface).to_str() } {
        Ok("wl_compositor") => unsafe {
            objects.compositor = NonNull::new(
                wl_proxy_marshal_flags(
                    registry.cast(),
                    WL_REGISTRY_BIND,
                    &raw const WL_COMPOSITOR_INTERFACE,
                    version,
                    0,
                    name,
                    WL_COMPOSITOR_INTERFACE.name,
                    version,
                    std::ptr::null::<c_void>(),
                )
                .cast(),
            )
            .zip(Some(version));
        },
        Ok("xdg_wm_base") => unsafe {
            objects.wm_base = NonNull::new(
                wl_proxy_marshal_flags(
                    registry.cast(),
                    WL_REGISTRY_BIND,
                    &raw const XDG_WM_BASE_INTERFACE,
                    version,
                    0,
                    name,
                    XDG_WM_BASE_INTERFACE.name,
                    version,
                    std::ptr::null::<c_void>(),
                )
                .cast(),
            )
            .zip(Some(version));
        },
        Ok("wl_seat") => unsafe {
            objects.seat = NonNull::new(
                wl_proxy_marshal_flags(
                    registry.cast(),
                    WL_REGISTRY_BIND,
                    &raw const WL_SEAT_INTERFACE,
                    version,
                    0,
                    name,
                    WL_SEAT_INTERFACE.name,
                    version,
                    std::ptr::null::<c_void>(),
                )
                .cast(),
            )
            .zip(Some(version));
        },
        _ => {}
    }
}
const unsafe extern "C" fn registry_listener_global_remove_listener(
    _data: *mut c_void,
    _wl_registry: *mut WlRegistry,
    _name: u32,
) {
}
unsafe extern "C" fn wm_base_listener_ping_listener(
    _data: *mut c_void,
    wm_base: NonNull<XdgWMBase>,
    serial: u32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wm_base.cast(),
            XDG_WM_BASE_PONG,
            std::ptr::null(),
            wl_proxy_get_version(wm_base.cast()),
            0,
            serial,
        );
    }
}
unsafe extern "C" fn xdg_surface_listener_configure_listener(
    _data: *mut c_void,
    xdg_surface: NonNull<XdgSurface>,
    serial: u32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            xdg_surface.cast(),
            XDG_SURFACE_ACK_CONFIGURE,
            std::ptr::null(),
            wl_proxy_get_version(xdg_surface.cast()),
            0,
            serial,
        );
    }
}
unsafe extern "C" fn xdg_toplevel_listener_configure_listener(
    data: *mut c_void,
    _xdg_toplevel: NonNull<XdgToplevel>,
    width: i32,
    height: i32,
    _states: *mut WlArray,
) {
    if height == 0 || width == 0 {
        return;
    }
    unsafe {
        wl_egl_window_resize(NonNull::new_unchecked(data).cast(), width, height, 0, 0);
        gl_viewport(0, 0, width, height);
    }
}
unsafe extern "C" fn xdg_toplevel_listener_close_listener(
    _data: *mut c_void,
    _xdg_toplevel: NonNull<XdgToplevel>,
) {
    std::process::exit(0);
}
const unsafe extern "C" fn xdg_toplevel_listener_configure_bounds_listener(
    _data: *mut c_void,
    _xdg_toplevel: NonNull<XdgToplevel>,
    _width: i32,
    _height: i32,
) {
}
const unsafe extern "C" fn xdg_toplevel_listener_wm_capabilities_listener(
    _data: *mut c_void,
    _xdg_toplevel: NonNull<XdgToplevel>,
    _capabilities: *mut WlArray,
) {
}

#[derive(Default, Debug, Clone, Copy)]
struct PointerState {
    position: Vec2<f32>,
    total_scroll: f64,
    left: bool,
}
const unsafe extern "C" fn pointer_listener_enter_listener(
    _data: *mut c_void,
    _pointer: NonNull<WlPointer>,
    _serial: u32,
    _surface: NonNull<WlSurface>,
    _surface_x: WlFixed,
    _surface_y: WlFixed,
) {
}
const unsafe extern "C" fn pointer_listener_leave_listener(
    _data: *mut c_void,
    _pointer: NonNull<WlPointer>,
    _serial: u32,
    _surface: NonNull<WlSurface>,
) {
}
unsafe extern "C" fn pointer_listener_motion_listener(
    data: *mut c_void,
    _pointer: NonNull<WlPointer>,
    _time: u32,
    surface_x: WlFixed,
    surface_y: WlFixed,
) {
    let pointer_state = unsafe { &*data.cast::<Cell<PointerState>>() };
    pointer_state.set(PointerState {
        position: Vec2::new(surface_x.to_f64() as f32, surface_y.to_f64() as f32),
        ..pointer_state.get()
    });
}
unsafe extern "C" fn pointer_listener_button_listener(
    data: *mut c_void,
    _pointer: NonNull<WlPointer>,
    _serial: u32,
    _time: u32,
    button: u32,
    state: u32,
) {
    if button != 0x110 {
        return;
    }
    let pointer_state = unsafe { &*data.cast::<Cell<PointerState>>() };
    pointer_state.set(PointerState {
        left: state == 1,
        ..pointer_state.get()
    });
}
unsafe extern "C" fn pointer_listener_axis_listener(
    data: *mut c_void,
    _pointer: NonNull<WlPointer>,
    _time: u32,
    axis: u32,
    value: WlFixed,
) {
    if axis != 0 {
        return;
    }
    let pointer_state = unsafe { &*data.cast::<Cell<PointerState>>() };
    let previous = pointer_state.get();
    pointer_state.set(PointerState {
        total_scroll: previous.total_scroll - value.to_f64(),
        ..previous
    });
}
const unsafe extern "C" fn pointer_listener_frame_listener(
    _data: *mut c_void,
    _pointer: NonNull<WlPointer>,
) {
}
const unsafe extern "C" fn pointer_listener_axis_source_listener(
    _data: *mut c_void,
    _pointer: NonNull<WlPointer>,
    _axis_source: u32,
) {
}
const unsafe extern "C" fn pointer_listener_axis_stop_listener(
    _data: *mut c_void,
    _pointer: NonNull<WlPointer>,
    _time: u32,
    _axis: u32,
) {
}
const unsafe extern "C" fn pointer_listener_axis_discrete_listener(
    _data: *mut c_void,
    _pointer: NonNull<WlPointer>,
    _axis: u32,
    _discrete: i32,
) {
}
const unsafe extern "C" fn pointer_listener_axis_value120_listener(
    _data: *mut c_void,
    _pointer: NonNull<WlPointer>,
    _axis: u32,
    _value120: i32,
) {
}
const unsafe extern "C" fn pointer_listener_axis_relative_direction_listener(
    _data: *mut c_void,
    _pointer: NonNull<WlPointer>,
    _axis: u32,
    _direction: u32,
) {
}

#[derive(Default, Debug, Clone, Copy)]
struct KeyboardState {
    enter: bool,
    up: bool,
    down: bool,
}
const unsafe extern "C" fn keyboard_listener_keymap_listener(
    _data: *mut c_void,
    _keyboard: NonNull<WlKeyboard>,
    _format: u32,
    _fd: i32,
    _size: u32,
) {
}
const unsafe extern "C" fn keyboard_listener_enter_listener(
    _data: *mut c_void,
    _keyboard: NonNull<WlKeyboard>,
    _serial: u32,
    _surface: NonNull<WlSurface>,
    _keys: *mut WlArray,
) {
}
const unsafe extern "C" fn keyboard_listener_leave_listener(
    _data: *mut c_void,
    _keyboard: NonNull<WlKeyboard>,
    _serial: u32,
    _surface: NonNull<WlSurface>,
) {
}
unsafe extern "C" fn keyboard_listener_key_listener(
    data: *mut c_void,
    _keyboard: NonNull<WlKeyboard>,
    _serial: u32,
    _time: u32,
    key: u32,
    state: u32,
) {
    let keyboard_state = unsafe { &*data.cast::<Cell<KeyboardState>>() };
    keyboard_state.set(match key {
        28 => KeyboardState {
            enter: state > 0,
            ..keyboard_state.get()
        },
        103 => KeyboardState {
            up: state > 0,
            ..keyboard_state.get()
        },
        108 => KeyboardState {
            down: state > 0,
            ..keyboard_state.get()
        },
        _ => keyboard_state.get(),
    });
}
const unsafe extern "C" fn keyboard_listener_modifiers_listener(
    _data: *mut c_void,
    _keyboard: NonNull<WlKeyboard>,
    _serial: u32,
    _mods_depressed: u32,
    _mods_latched: u32,
    _mods_locked: u32,
    _group: u32,
) {
}
const unsafe extern "C" fn keyboard_listener_repeat_info_listener(
    _data: *mut c_void,
    _keyboard: NonNull<WlKeyboard>,
    _rate: i32,
    _delay: i32,
) {
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct EglDisplay(*mut c_void);
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct EglConfig(*mut c_void);
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct EglSurface(*mut c_void);
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct EglContext(*mut c_void);

#[link(name = "EGL")]
unsafe extern "C" {
    #[link_name = "eglGetPlatformDisplay"]
    fn egl_get_platform_display(
        platform: c_int,
        native_display: *mut c_void,
        attrib_list: *const isize,
    ) -> EglDisplay;
    #[link_name = "eglInitialize"]
    fn egl_initialize(dpy: EglDisplay, major: *mut i32, minor: *mut i32) -> c_uint;
    #[link_name = "eglGetConfigs"]
    fn egl_get_configs(
        dpy: EglDisplay,
        configs: *mut EglConfig,
        config_size: i32,
        num_config: *mut i32,
    );
    #[link_name = "eglCreatePlatformWindowSurface"]
    fn egl_create_platform_window_surface(
        dpy: EglDisplay,
        config: EglConfig,
        native_window: *mut c_void,
        attrib_list: *const isize,
    ) -> EglSurface;
    #[link_name = "eglBindAPI"]
    fn egl_bind_api(api: c_int) -> c_uint;
    #[link_name = "eglCreateContext"]
    fn egl_create_context(
        dpy: EglDisplay,
        config: EglConfig,
        share_context: EglContext,
        attrib_list: *const isize,
    ) -> EglContext;
    #[link_name = "eglMakeCurrent"]
    fn egl_make_current(
        dpy: EglDisplay,
        draw: EglSurface,
        read: EglSurface,
        ctx: EglContext,
    ) -> c_uint;
    #[link_name = "eglGetProcAddress"]
    fn egl_get_proc_address(procname: *const c_char) -> *mut c_void;
    #[link_name = "eglSwapBuffers"]
    fn egl_swap_buffers(dpy: EglDisplay, surface: EglSurface) -> c_uint;
}

const EGL_OPENGL_API: c_int = 0x30A2;

#[repr(transparent)]
struct WlEglWindow(c_void);

#[link(name = "wayland-egl")]
unsafe extern "C" {
    fn wl_egl_window_create(
        surface: NonNull<WlSurface>,
        width: c_int,
        height: c_int,
    ) -> *mut WlEglWindow;
    fn wl_egl_window_resize(
        egl_window: NonNull<WlEglWindow>,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    );
}

const EGL_PLATFORM_WAYLAND_KHR: c_int = 0x31D8;

#[allow(clippy::struct_field_names)]
pub(super) struct Window {
    wl_display: NonNull<WlDisplay>,
    egl_display: EglDisplay,
    egl_surface: EglSurface,
    pointer_state: Box<Cell<PointerState>>,
    keyboard_state: Box<Cell<KeyboardState>>,
}

impl Window {
    #[allow(clippy::too_many_lines, clippy::type_complexity)]
    pub fn create() -> Result<Self, WindowCreateError> {
        unsafe {
            let wl_display = NonNull::new(wl_display_connect(std::ptr::null()))
                .ok_or(WindowCreateError::NullDisplay)?;

            let registry = NonNull::new(
                wl_proxy_marshal_flags(
                    wl_display.cast(),
                    WL_DISPLAY_GET_REGISTRY,
                    &raw const WL_REGISTRY_INTERFACE,
                    0,
                    0,
                    std::ptr::null::<c_void>(),
                )
                .cast::<WlRegistry>(),
            )
            .ok_or(WindowCreateError::NullRegistry)?;
            let mut interfaces = Interfaces::default();
            wl_proxy_add_listener(
                registry.cast(),
                (&raw const REGISTRY_LISTENER).cast(),
                (&raw mut interfaces).cast(),
            );
            wl_display_dispatch(wl_display);
            wl_display_roundtrip(wl_display);
            wl_proxy_destroy(registry.cast());

            let (compositor, compositor_version) = interfaces
                .compositor
                .ok_or(WindowCreateError::NoCompositor)?;

            let surface = NonNull::new(
                wl_proxy_marshal_flags(
                    compositor.cast(),
                    WL_COMPOSITOR_CREATE_SURFACE,
                    &raw const WL_SURFACE_INTERFACE,
                    compositor_version,
                    0,
                    std::ptr::null::<c_void>(),
                )
                .cast::<WlSurface>(),
            )
            .ok_or(WindowCreateError::NullSurface)?;

            let (wm_base, wm_base_version) =
                interfaces.wm_base.ok_or(WindowCreateError::NoWMBase)?;
            wl_proxy_add_listener(
                wm_base.cast(),
                (&raw const WM_BASE_LISTENER).cast(),
                std::ptr::null_mut(),
            );

            let xdg_surface = NonNull::new(
                wl_proxy_marshal_flags(
                    wm_base.cast(),
                    XDG_WM_BASE_GET_XDG_SURFACE,
                    &raw const XDG_SURFACE_INTERFACE,
                    wm_base_version,
                    0,
                    std::ptr::null::<c_void>(),
                    surface,
                )
                .cast::<XdgSurface>(),
            )
            .ok_or(WindowCreateError::NullXdgSurface)?;
            wl_proxy_add_listener(
                xdg_surface.cast(),
                (&raw const XDG_SURFACE_LISTENER).cast(),
                std::ptr::null_mut(),
            );

            let xdg_toplevel = NonNull::new(
                wl_proxy_marshal_flags(
                    xdg_surface.cast(),
                    XDG_SURFACE_GET_TOPLEVEL,
                    &raw const XDG_TOPLEVEL_INTERFACE,
                    wl_proxy_get_version(xdg_surface.cast()),
                    0,
                    std::ptr::null::<c_void>(),
                )
                .cast::<XdgToplevel>(),
            )
            .ok_or(WindowCreateError::NullToplevel)?;

            wl_proxy_marshal_flags(
                surface.cast(),
                WL_SURFACE_COMMIT,
                std::ptr::null(),
                wl_proxy_get_version(surface.cast()),
                0,
            );
            wl_display_dispatch(wl_display);

            let egl_display = egl_get_platform_display(
                EGL_PLATFORM_WAYLAND_KHR,
                wl_display.as_ptr().cast(),
                std::ptr::null(),
            );
            if egl_display.0.is_null() {
                return Err(WindowCreateError::NullEglDisplay);
            }

            let mut egl_version = (0, 0);
            if egl_initialize(egl_display, &raw mut egl_version.0, &raw mut egl_version.1) != 1 {
                return Err(WindowCreateError::EglNoInit);
            }

            let mut config = EglConfig(std::ptr::null_mut());
            let mut config_count = 0;
            egl_get_configs(egl_display, &raw mut config, 1, &raw mut config_count);
            if config_count == 0 {
                return Err(WindowCreateError::EglNoConfigs);
            }

            let wl_egl_window = NonNull::new(wl_egl_window_create(surface, 800, 600))
                .ok_or(WindowCreateError::NullWlEglWindow)?;

            let egl_surface = egl_create_platform_window_surface(
                egl_display,
                config,
                wl_egl_window.as_ptr().cast(),
                std::ptr::null(),
            );
            if egl_surface.0.is_null() {
                return Err(WindowCreateError::NullEglSurface);
            }

            if egl_bind_api(EGL_OPENGL_API) != 1 {
                return Err(WindowCreateError::EglNoOpenGL);
            }

            let context = egl_create_context(
                egl_display,
                config,
                EglContext(std::ptr::null_mut()),
                std::ptr::null(),
            );
            if context.0.is_null() {
                return Err(WindowCreateError::NullEglContext);
            }

            if egl_make_current(egl_display, egl_surface, egl_surface, context) != 1 {
                return Err(WindowCreateError::EglContextNotCurrent);
            }

            wl_proxy_add_listener(
                xdg_toplevel.cast(),
                (&raw const XDG_TOPLEVEL_LISTENER).cast(),
                wl_egl_window.as_ptr().cast(),
            );

            let (seat, seat_version) = interfaces.seat.ok_or(WindowCreateError::NoSeat)?;

            let pointer_state: Box<Cell<PointerState>> = Box::default();
            let pointer = NonNull::new(
                wl_proxy_marshal_flags(
                    seat.cast(),
                    WL_SEAT_GET_POINTER,
                    &raw const WL_POINTER_INTERFACE,
                    seat_version,
                    0,
                    std::ptr::null::<c_void>(),
                )
                .cast::<WlPointer>(),
            )
            .ok_or(WindowCreateError::NullPointer)?;
            wl_proxy_add_listener(
                pointer.cast(),
                (&raw const POINTER_LISTENER).cast(),
                (&raw const *pointer_state).cast_mut().cast(),
            );

            let keyboard_state: Box<Cell<KeyboardState>> = Box::default();
            let keyboard = NonNull::new(
                wl_proxy_marshal_flags(
                    seat.cast(),
                    WL_SEAT_GET_KEYBOARD,
                    &raw const WL_KEYBOARD_INTERFACE,
                    seat_version,
                    0,
                    std::ptr::null::<c_void>(),
                )
                .cast::<WlKeyboard>(),
            )
            .ok_or(WindowCreateError::NullKeyboard)?;
            wl_proxy_add_listener(
                keyboard.cast(),
                (&raw const KEYBOARD_LISTENER).cast(),
                (&raw const *keyboard_state).cast_mut().cast(),
            );

            Ok(Self {
                wl_display,
                egl_display,
                egl_surface,
                pointer_state,
                keyboard_state,
            })
        }
    }

    pub unsafe fn load_fn(name: &CStr) -> *mut c_void {
        unsafe { egl_get_proc_address(name.as_ptr()) }
    }

    pub fn swap_buffers(&self) -> Result<(), BufferSwapError> {
        unsafe {
            if egl_swap_buffers(self.egl_display, self.egl_surface) == 1 {
                Ok(())
            } else {
                Err(BufferSwapError)
            }
        }
    }

    pub fn check_events(&self) -> Result<(), EventCheckError> {
        unsafe {
            if wl_display_dispatch_pending(self.wl_display) == -1 {
                Err(EventCheckError)
            } else {
                Ok(())
            }
        }
    }

    pub fn pointer_position(&self) -> Vec2<f32> {
        self.pointer_state.get().position
    }

    pub fn total_scroll(&self) -> f64 {
        self.pointer_state.get().total_scroll
    }

    pub fn left_button(&self) -> bool {
        self.pointer_state.get().left
    }

    pub fn enter_key(&self) -> bool {
        self.keyboard_state.get().enter
    }

    pub fn up_key(&self) -> bool {
        self.keyboard_state.get().up
    }

    pub fn down_key(&self) -> bool {
        self.keyboard_state.get().down
    }
}

#[derive(Debug)]
pub enum WindowCreateError {
    NullDisplay,
    NullRegistry,
    NoCompositor,
    NullSurface,
    NoWMBase,
    NullXdgSurface,
    NullToplevel,

    NullEglDisplay,
    EglNoInit,
    EglNoConfigs,
    NullWlEglWindow,
    NullEglSurface,
    EglNoOpenGL,
    NullEglContext,
    EglContextNotCurrent,

    NoSeat,
    NullPointer,
    NullKeyboard,
}
impl Display for WindowCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::NullDisplay => "returned display interface was null",
            Self::NullRegistry => "returned registry interface was null",
            Self::NoCompositor => "could not bind compositor from registry",
            Self::NullSurface => "returned surface interface was null",
            Self::NoWMBase => "could not bind WM base from registry",
            Self::NullXdgSurface => "returned XDG surface interface was null",
            Self::NullToplevel => "returned toplevel interface was null",
            Self::NullEglDisplay => "could not create EGL Wayland display",
            Self::EglNoInit => "could not initialize EGL",
            Self::EglNoConfigs => "no EGL configurations available",
            Self::NullWlEglWindow => "returned Wayland EGL window was null",
            Self::NullEglSurface => "returned EGL surface was null",
            Self::EglNoOpenGL => "could not bind OpenGL to EGL",
            Self::NullEglContext => "returned EGL context was null",
            Self::EglContextNotCurrent => "could not make EGL context current",
            Self::NoSeat => "could not bind seat from registry",
            Self::NullPointer => "returned pointer interface was null",
            Self::NullKeyboard => "returned keyboard interface was null",
        })
    }
}
impl Error for WindowCreateError {}

#[derive(Debug)]
pub struct BufferSwapError;
impl Display for BufferSwapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EGL buffer swap returned false")
    }
}
impl Error for BufferSwapError {}

#[derive(Debug)]
pub struct EventCheckError;
impl Display for EventCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Wayland display dispatch returned -1")
    }
}
impl Error for EventCheckError {}
