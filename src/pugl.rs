#![allow(dead_code)]
#![allow(unused_macros)]

use std::ops::{Add, AddAssign};
use std::str;
use std::char;
use std::marker::PhantomData;

use cairo;
use cairo_sys;

mod p {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}


/// Representing coordinates on a widget
///
#[derive(Copy, Clone, Default, Debug)]
pub struct Coord {
    /// x coordinate
    pub x: f64,
    /// y coordinate
    pub y: f64 }

impl Add for Coord {
    type Output = Coord;

    fn add (self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Coord) {
        *self = Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

/// Representing a size of a rectangle
#[derive(Copy, Clone, Default, Debug)]
pub struct Size {
    /// width
    pub w: f64,
    /// height
    pub h: f64 }

impl Add for Size {
    type Output = Size;

    fn add (self, other: Size) -> Size {
        Size {
            w: self.w + other.w,
            h: self.h + other.h
        }
    }
}

/// The context of a GUI event
#[derive(Copy, Clone, Default)]
pub struct EventContext {
    /// View relative position of the event
    pub pos: Coord,
    /// Root relative position of the event
    pub pos_root: Coord,
    /// The time of the event in seconds
    pub time: f64
}

/// Keys not representing a character
#[derive(Copy, Clone)]
pub enum SpecialKey {
    Backspace,
    Escape,
    Delete,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Left,
    Up,
    Right,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
    Insert,
    ShiftL,
    ShiftR,
    CtrlL,
    CtrlR,
    AltL,
    AltR,
    SuperL,
    SuperR,
    KeyMenu,
    KeyCapsLock,
    KeyScrollLock,
    KeyNumLock,
    KeyPrintScreen,
    KeyPause,
    None
}

impl From<p::PuglKey> for SpecialKey {
    fn from (k: p::PuglKey) -> SpecialKey {
        match k {
	    p::PuglKey_PUGL_KEY_BACKSPACE => SpecialKey::Backspace,
	    p::PuglKey_PUGL_KEY_ESCAPE => SpecialKey::Escape,
	    p::PuglKey_PUGL_KEY_DELETE => SpecialKey::Delete,
            p::PuglKey_PUGL_KEY_F1 => SpecialKey::F1,
            p::PuglKey_PUGL_KEY_F2 => SpecialKey::F2,
            p::PuglKey_PUGL_KEY_F3 => SpecialKey::F3,
            p::PuglKey_PUGL_KEY_F4 => SpecialKey::F4,
            p::PuglKey_PUGL_KEY_F5 => SpecialKey::F5,
            p::PuglKey_PUGL_KEY_F6 => SpecialKey::F6,
            p::PuglKey_PUGL_KEY_F7 => SpecialKey::F7,
            p::PuglKey_PUGL_KEY_F8 => SpecialKey::F8,
            p::PuglKey_PUGL_KEY_F9 => SpecialKey::F9,
            p::PuglKey_PUGL_KEY_F10 => SpecialKey::F10,
            p::PuglKey_PUGL_KEY_F11 => SpecialKey::F11,
            p::PuglKey_PUGL_KEY_F12 => SpecialKey::F12,
            p::PuglKey_PUGL_KEY_LEFT => SpecialKey::Left,
            p::PuglKey_PUGL_KEY_UP => SpecialKey::Up,
            p::PuglKey_PUGL_KEY_RIGHT => SpecialKey::Right,
            p::PuglKey_PUGL_KEY_DOWN => SpecialKey::Down,
            p::PuglKey_PUGL_KEY_PAGE_UP => SpecialKey::PageUp,
            p::PuglKey_PUGL_KEY_PAGE_DOWN => SpecialKey::PageDown,
            p::PuglKey_PUGL_KEY_HOME => SpecialKey::Home,
            p::PuglKey_PUGL_KEY_END => SpecialKey::End,
            p::PuglKey_PUGL_KEY_INSERT => SpecialKey::Insert,
            p::PuglKey_PUGL_KEY_SHIFT_L => SpecialKey::ShiftL,
	    p::PuglKey_PUGL_KEY_SHIFT_R => SpecialKey::ShiftR,
            p::PuglKey_PUGL_KEY_CTRL_L => SpecialKey::CtrlL,
            p::PuglKey_PUGL_KEY_ALT_L => SpecialKey::AltL,
            p::PuglKey_PUGL_KEY_CTRL_R => SpecialKey::CtrlR,
            p::PuglKey_PUGL_KEY_ALT_R => SpecialKey::AltR,
            p::PuglKey_PUGL_KEY_SUPER_L => SpecialKey::SuperL,
            p::PuglKey_PUGL_KEY_SUPER_R => SpecialKey::SuperR,
	    p::PuglKey_PUGL_KEY_MENU => SpecialKey::KeyMenu,
	    p::PuglKey_PUGL_KEY_CAPS_LOCK => SpecialKey::KeyCapsLock,
	    p::PuglKey_PUGL_KEY_SCROLL_LOCK => SpecialKey::KeyScrollLock,
	    p::PuglKey_PUGL_KEY_NUM_LOCK => SpecialKey::KeyNumLock,
	    p::PuglKey_PUGL_KEY_PRINT_SCREEN => SpecialKey::KeyPrintScreen,
	    p::PuglKey_PUGL_KEY_PAUSE => SpecialKey::KeyPause,
	    _ => SpecialKey::None
        }
    }
}

bitflags! {
    /// Keyboard modifiers
    pub struct Modifiers: u32 {
        const NONE  = 0;
        const SHIFT = 1;
        const CTRL  = 2;
        const ALT   = 4;
        const SUPER = 8;
    }
}

/// System's key code
type KeyCode = u32;

/// Bitflag of Keyboard modifiers
type Modifier = u32;

/// Representing a key from the keyboard
#[derive(Copy, Clone)]
pub enum KeyVal {
    /// character key
    Character (char),
    /// special key (non-character)
    Special (SpecialKey)
}

/// Key with keyboard modifiers
#[derive(Copy, Clone)]
pub struct Key {
    /// actual key
    pub key: KeyVal,
    /// modifiers
    pub modifiers: Modifier,
    /// system's code for the key
    pub code: KeyCode
}

impl Key {
    /// Returns the character if the key is a character key, otherwise none
    pub fn try_char(&self) -> Option<char> {
        match self.key {
            KeyVal::Character (c) => Some (c),
            _ => None
        }
    }
}

impl From<p::PuglEventKey> for KeyVal {
    fn from(ke: p::PuglEventKey) -> KeyVal {
        match ke.key {
            0 => KeyVal::Special (SpecialKey::from(ke.keycode)),
            _ => KeyVal::Character (char::from_u32(ke.key).unwrap())
        }
    }
}

impl From<p::PuglEventKey> for Key {
    fn from(ke: p::PuglEventKey) -> Key {
        Key {
            key: KeyVal::from (ke),
            code: ke.keycode,
            modifiers: ke.state
        }
    }
}

impl From<p::PuglEventKey> for EventContext {
    fn from(ke: p::PuglEventKey) -> EventContext {
        EventContext {
            pos: Coord { x: ke.x, y: ke.y },
            pos_root: Coord { x: ke.xRoot, y: ke.yRoot },
            time: ke.time
        }
    }
}

/// Representing a mouse button
#[derive(Copy, Clone, PartialEq)]
pub struct MouseButton {
    /// The number of the mouse button
    pub num: u32,
    /// Keyboard modifiers, when the mouse button event happened
    pub modifiers: Modifier
}

impl From<p::PuglEventButton> for MouseButton {
    fn from(be: p::PuglEventButton) -> MouseButton {
        MouseButton {
            num: be.button,
            modifiers: be.state
        }
    }
}

impl From<p::PuglEventButton> for EventContext {
        fn from(be: p::PuglEventButton) -> EventContext {
        EventContext {
            pos: Coord { x: be.x, y: be.y },
            pos_root: Coord { x: be.xRoot, y: be.yRoot },
            time: be.time
        }
    }
}

/// Context of a pointer event
#[derive(Copy, Clone)]
pub struct MotionContext {
    /// Keyboard modifiers
    pub state: Modifier,
    /// True iff this event is a motion hint
    pub is_hint: bool,
    /// True iff this is the focus view
    pub focus: bool
}

impl From<p::PuglEventMotion> for MotionContext {
    fn from (me: p::PuglEventMotion) -> MotionContext {
        MotionContext {
            state: me.state,
            is_hint: me.isHint,
            focus: me.focus
        }
    }
}

impl From<p::PuglEventMotion> for EventContext {
    fn from (me: p::PuglEventMotion) -> EventContext {
        EventContext {
            pos: Coord { x: me.x, y: me.y },
            pos_root: Coord { x: me.xRoot, y: me.yRoot },
            time: me.time
        }
    }
}

/// A mouse wheel scroll event
///
/// The scroll distance is expressed in "lines", an arbitrary unit
/// that corresponds to a single tick of a detented mouse wheel.  For
/// example, `dy` = 1.0 scrolls 1 line up.  Some systems and devices
/// support finer resolution and/or higher values for fast scrolls, so
/// programs should handle any value gracefully.
#[derive(Copy, Clone)]
pub struct Scroll {
    /// horizontal scroll distance
    pub dx: f64,
    /// vertical scroll distance
    pub dy: f64,
    pub modifiers: Modifier
}

impl From<p::PuglEventScroll> for Scroll {
    fn from (se: p::PuglEventScroll) -> Scroll {
        Scroll {
            dx: se.dx, dy: se.dy,
            modifiers: se.state
        }
    }
}

impl From<p::PuglEventScroll> for EventContext {
    fn from (se: p::PuglEventScroll) -> EventContext {
        EventContext {
            pos: Coord { x: se.x, y: se.y },
            pos_root: Coord { x: se.xRoot, y: se.yRoot },
            time: se.time
        }
    }
}

impl From<p::PuglEventConfigure> for Size {
    fn from (ce: p::PuglEventConfigure) -> Size {
        Size { w: ce.width, h: ce.height }
    }
}

/// The area that needs to be redrawn due to an expose event
#[derive(Copy, Clone)]
pub struct ExposeArea {
    /// The view relative coordinate
    pub pos: Coord,
    /// The size
    pub size: Size,
    /// Number of expose events to follow
    pub count: i32
}

impl From<p::PuglEventExpose> for ExposeArea {
    fn from(e: p::PuglEventExpose) -> ExposeArea {
        ExposeArea {
            pos: Coord { x: e.x, y: e.y },
            size: Size { w: e.width, h: e.height },
            count: e.count
        }
    }
}

/// Event types
#[derive(Copy, Clone)]
pub enum EventType {
    KeyPress (Key),
    KeyRelease (Key),
    MouseButtonPress (MouseButton),
    MouseButtonRelease (MouseButton),
    MouseMove (MotionContext),
    Scroll (Scroll)
}

#[derive(Copy, Clone)]
pub struct Event {
    pub data: EventType,
    pub context: EventContext
}

impl Event {
    /// Returns the key if the event is a KeyPress, otherwise None.
    pub fn try_keypress(&self) -> Option<Key> {
        match self.data {
            EventType::KeyPress (k) => Some (k),
            _ => None
        }
    }
}

pub type PuglViewFFI = *mut p::PuglView;

/// "Return status code.
#[repr(u32)]
pub enum Status {
    /// Success
    Success,
    /// Non=fatal failure
    Failure,
    /// Unknown system error
    UnknownError,
    /// Invalid or missing backend
    BadBackend,
    /// Invalid parameter
    BadParameter,
    /// Backend initialisation failed
    BackendFailed,
    /// Class registration failed
    RegistrationFailed,
    /// System view realization failed
    RealizeFailed,
    /// Failed to set pixel format
    SetFormatFailed,
    /// Failed to create drawing context
    CreateContextFailed,
    /// Unsupported data type
    UnsupportedType
}

impl From<p::PuglStatus> for Status {
    fn from(ps: p::PuglStatus) -> Status {
	match ps {
	    p::PuglStatus_PUGL_SUCCESS => Status::Success ,
	    p::PuglStatus_PUGL_FAILURE => Status::Failure ,
	    p::PuglStatus_PUGL_UNKNOWN_ERROR => Status::UnknownError ,
	    p::PuglStatus_PUGL_BAD_BACKEND => Status::BadBackend ,
	    p::PuglStatus_PUGL_BAD_PARAMETER => Status::BadParameter ,
	    p::PuglStatus_PUGL_BACKEND_FAILED => Status::BackendFailed ,
	    p::PuglStatus_PUGL_REGISTRATION_FAILED => Status::RegistrationFailed ,
	    p::PuglStatus_PUGL_REALIZE_FAILED => Status::RealizeFailed ,
	    p::PuglStatus_PUGL_SET_FORMAT_FAILED => Status::SetFormatFailed ,
	    p::PuglStatus_PUGL_CREATE_CONTEXT_FAILED => Status::CreateContextFailed ,
	    p::PuglStatus_PUGL_UNSUPPORTED_TYPE => Status::UnsupportedType,
	    _ => Status::UnsupportedType
	}
    }
}

/// A trait for an object of a pugl "app"
pub trait PuglViewTrait {

    /// Called if an event happened that is to be processed. Shall return a result status.
    fn event(&mut self, ev: Event) -> Status;

    /// Called when a part of the view needs to be redrawn due to an exposure
    /// The cairo::Context object `cr` reference can be used to draw on.
    fn exposed (&mut self, _expose: &ExposeArea, cr: &cairo::Context);

    /// Called when the view has been resized
    fn resize (&mut self, size: Size);

    /// Called when the view is requested to close by the window system
    fn close_request(&mut self);

    /// set the handle of the windows system's view
    fn set_view (&mut self, view: PuglViewFFI);

    /// returns a handle to the window system's view
    fn view (&self) -> PuglViewFFI;

    /// returns a pointer to the PugleWorld
    fn world (&self) -> *mut p::PuglWorld {
	unsafe { p::puglGetWorld(self.view() as *mut p::PuglView) }
    }

    /// can be called to ask for a redisplay of the view's contents
    fn post_redisplay (&self) -> Status {
        unsafe { Status::from(p::puglPostRedisplay(self.view())) }
    }

    /// set the position and size of the view
    fn set_frame (&self, width: f64, height: f64) -> Status {
        unsafe { Status::from(p::puglSetFrame(self.view(), p::PuglRect { x:0.0, y:0.0, width, height})) }
    }

    /// set the minimum size of the view
    fn set_min_size (&self, width: i32, height: i32) -> Status {
        unsafe { Status::from(p::puglSetMinSize(self.view(), width, height)) }
    }

    /// set the window title
    fn set_window_title (&self, title: &str) -> Status {
        unsafe { Status::from(p::puglSetWindowTitle(self.view(), title.as_ptr() as *const i8)) }
    }

    /// can be called to show the view
    fn show_window (&self) -> Status {
        unsafe { Status::from(p::puglShowWindow(self.view())) }
    }

    /// Update by processing events from the window system.
    ///
    /// This function is a single iteration of the main loop, and should be called
    /// repeatedly to update all views.
    ///
    /// If `timeout` is zero, then this function will not block.  Plugins should
    /// always use a timeout of zero to avoid blocking the host.
    ///
    /// If a positive `timeout` is given, then events will be processed for that
    /// amount of time, starting from when this function was called.
    ///
    /// If a negative `timeout` is given, this function will block indefinitely
    /// until an event occurs.
    ///
    /// For continuously animating programs, a timeout that is a reasonable fraction
    /// of the ideal frame period should be used, to minimise input latency by
    /// ensuring that as many input events are consumed as possible before drawing.
    ///
    /// return #PUGL_SUCCESS if events are read, #PUGL_FAILURE if not, or an error.
    ///
    fn update (&self, timeout: f64) -> Status {
        unsafe { Status::from(p::puglUpdate(self.world(), timeout)) }
    }
}

/// A struct for a pugl "app" object
/// T is struct implementing the PuglViewTrait, representing the app's logic
pub struct PuglView<T: PuglViewTrait> {
    ui_type: std::marker::PhantomData<T>,
    instance: *mut p::PuglView
}


unsafe extern "C"
fn event_handler<T: PuglViewTrait> (view_ptr: *mut p::PuglView, event_ptr: *const p::PuglEvent) -> p::PuglStatus {
    let ev = *event_ptr;
    let handle: &mut T = &mut *(p::puglGetHandle(view_ptr) as *mut T);
    //eprintln!("event_handler: {:?}", ev.type_);
    let event = match ev.type_ {
        p::PuglEventType_PUGL_KEY_PRESS => {
            Event { data: EventType::KeyPress(Key::from (ev.key)), context: EventContext::from (ev.key) }
        },
        p::PuglEventType_PUGL_KEY_RELEASE => {
            Event { data: EventType::KeyRelease(Key::from (ev.key)), context: EventContext::from (ev.key) }
        },
        p::PuglEventType_PUGL_BUTTON_PRESS => {
            Event { data: EventType::MouseButtonPress(MouseButton::from (ev.button)), context: EventContext::from (ev.button) }
        },
        p::PuglEventType_PUGL_BUTTON_RELEASE => {
            Event { data: EventType::MouseButtonRelease(MouseButton::from (ev.button)), context: EventContext::from (ev.button) }
        },
        p::PuglEventType_PUGL_MOTION_NOTIFY => {
            Event { data: EventType::MouseMove(MotionContext::from (ev.motion)), context: EventContext::from (ev.motion) }
        },
        p::PuglEventType_PUGL_SCROLL => {
            Event { data: EventType::Scroll(Scroll::from (ev.scroll)), context: EventContext::from (ev.scroll) }
        }
        p::PuglEventType_PUGL_CLOSE => {
            handle.close_request ();
            return p::PuglStatus_PUGL_SUCCESS
        }
        p::PuglEventType_PUGL_EXPOSE => {
            let cr = cairo::Context::from_raw_borrow (p::puglGetContext(view_ptr) as *mut cairo_sys::cairo_t);
            handle.exposed (&ExposeArea::from (ev.expose), &cr);
            return p::PuglStatus_PUGL_SUCCESS
        },
        p::PuglEventType_PUGL_CONFIGURE => {
            let size = Size::from (ev.configure);
            handle.resize (size);
            return p::PuglStatus_PUGL_SUCCESS
        },
        _  => { return p::PuglStatus_PUGL_SUCCESS }

    };
    handle.event (event) as p::PuglStatus
}

impl<T: PuglViewTrait> PuglView<T> {
    /// Sets up a new PuglView for a heap allocated object of T implementing PuglViewTrait
    ///
    pub fn make_view(mut handle: Box<T>, parent_window: *mut std::ffi::c_void) -> Box<Self> {
        let view = Box::new(PuglView {
            instance: unsafe {
                let inst = p::puglNewView(p::puglNewWorld(p::PuglWorldType_PUGL_PROGRAM, 0));
                inst
            },
	    ui_type: PhantomData
        });
        handle.set_view(view.instance);
	let handle = Box::into_raw(handle) as *const T;
        unsafe {
	    if !parent_window.is_null() {
		println!("Parent window is {}", parent_window as usize);
		p::puglSetParentWindow(view.instance, parent_window as usize);
	    }
            p::puglSetHandle(view.instance, handle as p::PuglHandle);
            p::puglSetEventFunc(view.instance, Some(event_handler::<T>));
	    p::puglSetBackend(view.instance, p::puglCairoBackend());
            p::puglSetViewHint(view.instance, p::PuglViewHint_PUGL_RESIZABLE, true as i32);
            p::puglSetViewHint(view.instance, p::PuglViewHint_PUGL_IGNORE_KEY_REPEAT, true as i32);
        }
        view
    }

    /// returns a handle to the object T
    pub fn handle(&self) -> &mut T {
	unsafe {
	    &mut *(p::puglGetHandle(self.instance) as *mut T) as &mut T
	}
    }

    /// returns a handle to the window system's view
    pub fn view(&self) -> PuglViewFFI {
	self.instance
    }

    /// retuns a handle to the native window
    pub fn native_window(&self) -> p::PuglNativeView {
	unsafe { p::puglGetNativeWindow(self.view()) }
    }
}

impl<T: PuglViewTrait> Drop for PuglView<T> {
    fn drop(&mut self) {
        unsafe {
	    let instance = self.instance as *mut p::PuglView;
	    let world = p::puglGetWorld(instance);
            p::puglFreeView(instance);
	    p::puglFreeWorld(world);
        };
    }
}
