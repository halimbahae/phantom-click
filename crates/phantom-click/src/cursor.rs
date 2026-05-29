#[cfg(target_os = "macos")]
mod platform {
    use std::ffi::c_void;

    #[repr(C)]
    struct CGPoint { x: f64, y: f64 }

    extern "C" {
        fn CGEventCreate(source: *const c_void) -> *mut c_void;
        fn CGEventGetLocation(event: *mut c_void) -> CGPoint;
        fn CGWarpMouseCursorPosition(point: CGPoint) -> CGPoint;
        fn CFRelease(event: *mut c_void);
    }

    pub fn position() -> (f64, f64) {
        unsafe {
            let e = CGEventCreate(std::ptr::null());
            if e.is_null() { return (0.0, 0.0); }
            let p = CGEventGetLocation(e);
            CFRelease(e);
            (p.x, p.y)
        }
    }

    pub fn move_to(x: f64, y: f64) {
        unsafe {
            CGWarpMouseCursorPosition(CGPoint { x, y });
        }
    }
}

#[cfg(target_os = "linux")]
mod platform {
    use std::ffi::{c_int, c_uint, c_ulong, c_void};

    #[link(name = "X11")]
    extern "C" {
        fn XOpenDisplay(_: *const i8) -> *mut c_void;
        fn XDefaultRootWindow(_: *mut c_void) -> c_ulong;
        fn XQueryPointer(
            _: *mut c_void, _: c_ulong, _: *mut c_ulong, _: *mut c_ulong,
            _: *mut c_int, _: *mut c_int, _: *mut c_int, _: *mut c_int, _: *mut c_uint,
        ) -> c_int;
    }

    pub fn position() -> (f64, f64) {
        unsafe {
            let dpy = XOpenDisplay(std::ptr::null());
            if dpy.is_null() { return (0.0, 0.0); }
            let root = XDefaultRootWindow(dpy);
            let (mut rx, mut ry) = (0, 0);
            let mut mask: c_uint = 0;
            XQueryPointer(dpy, root, std::ptr::null_mut(), std::ptr::null_mut(),
                &mut rx, &mut ry, std::ptr::null_mut(), std::ptr::null_mut(), &mut mask);
            (rx as f64, ry as f64)
        }
    }

    pub fn move_to(x: f64, y: f64) {
        use std::ffi::c_int;
        extern "C" {
            fn XWarpPointer(
                dsp: *mut c_void, src_w: c_ulong, dest_w: c_ulong,
                src_x: c_int, src_y: c_int, src_w_: c_uint, src_h: c_uint,
                dest_x: c_int, dest_y: c_int,
            ) -> c_int;
        }
        unsafe {
            let dpy = XOpenDisplay(std::ptr::null());
            if dpy.is_null() { return; }
            let root = XDefaultRootWindow(dpy);
            XWarpPointer(dpy, 0, root, 0, 0, 0, 0, x as c_int, y as c_int);
        }
    }
}

#[cfg(target_os = "windows")]
mod platform {
    #[repr(C)]
    struct POINT { x: i32, y: i32 }

    #[link(name = "user32")]
    extern "system" {
        fn GetCursorPos(point: *mut POINT) -> i32;
    }

    pub fn position() -> (f64, f64) {
        unsafe {
            let mut pt = POINT { x: 0, y: 0 };
            if GetCursorPos(&mut pt) == 0 { return (0.0, 0.0); }
            (pt.x as f64, pt.y as f64)
        }
    }

    extern "system" {
        fn SetCursorPos(x: i32, y: i32) -> i32;
    }

    pub fn move_to(x: f64, y: f64) {
        unsafe { SetCursorPos(x as i32, y as i32); }
    }
}

pub fn position() -> (f64, f64) {
    platform::position()
}

pub fn move_to(x: f64, y: f64) {
    platform::move_to(x, y);
}
