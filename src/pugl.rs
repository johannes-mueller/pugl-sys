#![allow(dead_code)]

use std::ops::{Add, AddAssign};
use std::str;
use std::char;
use std::marker::PhantomData;

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
    pub y: f64
}

impl Coord {
    /// Scales the `Coord` by a `scale_factor`
    ///
    /// ```
    /// let c = pugl_sys::Coord { x: 1., y: 1. };
    /// let scaled = c.scale(2.);
    ///
    /// assert!(scaled.x - 2. < f64::EPSILON);
    /// assert!(scaled.y - 2. < f64::EPSILON);
    pub fn scale(&self, factor: f64) -> Coord {
        Coord {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

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
    pub h: f64
}

impl Size {
    /// Scales the `Size` by a `scale_factor`
    ///
    /// ```
    /// let s = pugl_sys::Size { w: 1., h: 1. };
    /// let scaled = s.scale(2.);
    ///
    /// assert!(scaled.w - 2. < f64::EPSILON);
    /// assert!(scaled.w - 2. < f64::EPSILON);
    pub fn scale(&self, factor: f64) -> Size {
        Size {
            w: self.w * factor,
            h: self.h * factor
        }
    }
}

impl Add for Size {
    type Output = Size;

    fn add (self, other: Size) -> Size {
        Size {
            w: self.w + other.w,
            h: self.h + other.h
        }
    }
}


/// A rectangle
///
/// This This is used to describe things like view position and size.  Pugl generally
/// uses coordinates where the top left corner is 0,0.
#[derive(Copy, Clone, Default, Debug)]
pub struct Rect {
    /// The position of the upper left corner of the `Rect`
    pub pos: Coord,
    /// The size of the `Rect`
    pub size: Size
}

impl From<p::PuglRect> for Rect {
    fn from(pr: p::PuglRect) -> Rect {
        Rect {
            pos: Coord { x: pr.x, y: pr.y },
            size: Size { w: pr.width, h: pr.height }
        }
    }
}

impl From <Rect> for p::PuglRect {
    fn from(r: Rect) -> p::PuglRect {
        p::PuglRect {
            x: r.pos.x,
            y: r.pos.y,
            width: r.size.w,
            height: r.size.h
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
#[derive(Copy, Clone, PartialEq)]
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
    /// A Character key
    Character(char),
    /// A special key (non-character)
    Special(SpecialKey)
}

/// Key with keyboard modifiers
#[derive(Copy, Clone)]
pub struct Key {
    /// The actual key
    pub key: KeyVal,
    /// The modifiers to be used with the [`Modifiers`](struct.Modifiers.html) struct
    pub modifiers: Modifier,
    /// System's code for the key
    pub code: KeyCode
}

impl Key {
    /// Returns the character if the key is a character key, otherwise `None`
    ///
    /// ```
    /// let char_key = pugl_sys::Key {
    ///     key: pugl_sys::KeyVal::Character('A'),
    ///     modifiers: pugl_sys::Modifiers::SHIFT.bits(),
    ///     code: 38
    /// };
    /// let special_key = pugl_sys::Key {
    ///     key: pugl_sys::KeyVal::Special(pugl_sys::SpecialKey::F1),
    ///     modifiers: pugl_sys::Modifiers::NONE.bits(),
    ///     code: 67
    /// };
    ///
    /// assert_eq!(char_key.try_char(), Some('A'));
    /// assert_eq!(special_key.try_char(), None);
    /// ```
    pub fn try_char(&self) -> Option<char> {
        match self.key {
            KeyVal::Character(c) => Some(c),
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
    /// to be used with the [`Modifiers`](struct.Modifiers.html) struct.
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

type EventFlag = u32;

bitflags! {
    pub struct EventFlags: u32 {
        const NONE = 0;
        const IS_SEND_EVENT = 1;
        const IS_HINT = 2;
    }
}

/// Context of a pointer event
#[derive(Copy, Clone)]
pub struct MotionContext {
    /// Keyboard modifiers to be used with the [`Modifiers`](struct.Modifiers.html) struct.
    pub modifiers: Modifier,
    /// The event flags
    pub flags: EventFlags
}

impl From<p::PuglEventMotion> for MotionContext {
    fn from (me: p::PuglEventMotion) -> MotionContext {
        MotionContext {
            modifiers: me.state,
            flags: EventFlags::from_bits_truncate(me.flags)
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
}

impl From<p::PuglEventExpose> for ExposeArea {
    fn from(e: p::PuglEventExpose) -> ExposeArea {
        ExposeArea {
            pos: Coord { x: e.x, y: e.y },
            size: Size { w: e.width, h: e.height },
        }
    }
}

/// Event types
#[derive(Copy, Clone)]
pub enum EventType {
    KeyPress(Key),
    KeyRelease(Key),
    MouseButtonPress(MouseButton),
    MouseButtonRelease(MouseButton),
    MouseMove(MotionContext),
    Scroll(Scroll)
}

/// An event signaled by the windowing system
#[derive(Copy, Clone)]
pub struct Event {
    pub data: EventType,
    pub context: EventContext
}

impl Event {
    /// Returns the key if the event is a `KeyPress`, otherwise `None`.
    pub fn try_keypress(&self) -> Option<Key> {
        match self.data {
            EventType::KeyPress (k) => Some (k),
            _ => None
        }
    }

    /// Returns the position where the mouse cursor was, when the event happened
    /// relative to the top left corner of the View's window.
    pub fn pos(&self) -> Coord {
        self.context.pos
    }

    /// Retuns the position where the mouse cursor was, when the event happened
    /// relative to the top left corner of the View's window scaled by `scale_factor`.
    pub fn scale_pos(self, scale_factor: f64) -> Event {
        let mut ev = self;
        ev.context.pos = self.context.pos.scale(scale_factor);
        ev
    }

    /// Retuns the position where the mouse cursor was, when the event happened
    /// relative to the top left corner of the root window.
    pub fn pos_root(&self) -> Coord {
        self.context.pos_root
    }
}

/// Available mouse cursors
#[derive(Copy, Clone)]
pub enum Cursor {
    Arrow,
    Caret,
    CrossHair,
    Hand,
    No,
    LeftRight,
    UpDown
}

impl From<Cursor> for p::PuglCursor {
    fn from(c: Cursor) -> p::PuglCursor {
        match c {
            Cursor::Arrow => p::PuglCursor_PUGL_CURSOR_ARROW,
            Cursor::Caret => p::PuglCursor_PUGL_CURSOR_CARET,
            Cursor::CrossHair => p::PuglCursor_PUGL_CURSOR_CROSSHAIR,
            Cursor::Hand => p::PuglCursor_PUGL_CURSOR_HAND,
            Cursor::No => p::PuglCursor_PUGL_CURSOR_NO,
            Cursor::LeftRight => p::PuglCursor_PUGL_CURSOR_LEFT_RIGHT,
            Cursor::UpDown => p::PuglCursor_PUGL_CURSOR_UP_DOWN,
        }
    }
}


pub type PuglViewFFI = *mut p::PuglView;

/// Return status code.
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

/// The central trait for an object of a pugl "UI"
///
/// A UI implementation needs to have an object to manage the state of
/// the UI as well as serving as an interface to the actual
/// application. Such an object must implement the required methods of
/// `PuglViewTrait`. The provided methods
/// [`focus_in()`](#method.focus_in) and
/// [`focus_out()`](#method.focus_out) as wellas
/// [`timer_event()`](#method.timer_event] can be implmentat
/// optionally.
/// All the other provided methods should not be reimplemented.
pub trait PuglViewTrait {

    /// Called if an event happened that is to be processed.
    ///
    /// The data of the `Event` comes withe the argument `ev`.
    ///
    /// Shall return a result `Status`.
    fn event(&mut self, ev: Event) -> Status;

    /// Called when a part of the view needs to be redrawn due to an
    /// exposure.
    ///
    /// The `cr` reference can be used to draw on.
    ///
    /// The `expose` argument provides information on the area that
    /// needs to be redrawn.
    fn exposed (&mut self, expose: &ExposeArea, cr: &cairo::Context);

    /// Called when the view has been resized
    ///
    /// The UI should relayout its contents to make it fit the size
    /// provided by `size`.
    fn resize (&mut self, size: Size);

    /// Called when the view is requested to close by the window system
    ///
    /// The UI should exit the event loop before the next cycle after
    /// this method has been called.
    fn close_request(&mut self);

    /// Called when the view recieves the focus
    ///
    /// Should be reimplemented if the application needs to react on
    /// getting the focus.
    ///
    /// Shall return a result Status.
    fn focus_in(&mut self) -> Status { Status::Success }

    /// Called when the view gives the focus away
    ///
    /// Should be reimplemented if the application needs to react on
    /// giving the focus away.
    ///
    /// Shall return a result Status.
    fn focus_out(&mut self) -> Status { Status::Success }

    /// Called when a timer launched by
    /// [`start_timer()`](#method.start_timer) finished.
    ///
    /// Should be reimplemented if the application at some point calls
    /// [`start_timer()`](#method.start_timer)
    ///
    /// Shall return a result Status.
    fn timer_event(&mut self, _id: usize) -> Status { Status::Success }

    /// Returns a handle to the window system's view
    fn view (&self) -> PuglViewFFI;

    /// Returns a pointer to the `PugleWorld`
    fn world (&self) -> *mut p::PuglWorld {
        unsafe { p::puglGetWorld(self.view() as *mut p::PuglView) }
    }

    /// Request a redisplay for the entire view.
    ///
    /// This will cause an expose event to be dispatched later. If
    /// called from within the event handler, the expose should arrive
    /// at the end of the current event loop iteration, though this is
    /// not strictly guaranteed on all platforms. If called elsewhere,
    /// an expose will be enqueued to be processed in the next event
    /// loop iteration.
    fn post_redisplay (&self) -> Status {
        unsafe { Status::from(p::puglPostRedisplay(self.view())) }
    }

    /// Request a redisplay of the given rectangle within the view.
    ///
    /// This has the same semantics as [`post_redisplay()`](#method.post_redisplay),
    /// but allows giving a precise region for redrawing only a
    /// portion of the view.
    fn post_redisplay_rect(&self, pos: Coord, size: Size) -> Status {
        let p_rect = p::PuglRect {
            x: pos.x,
            y: pos.y,
            width: size.w,
            height: size.h
        };
        unsafe { Status::from(p::puglPostRedisplayRect(self.view(), p_rect)) }
    }

    ///  Get the current position and size of the view.
    ///
    ///  The position is in screen coordinates with an upper left origin.
    fn get_frame(&self) -> Rect {
        unsafe { p::puglGetFrame(self.view()).into() }
    }

    /// Set the current position and size of the view.
    ///
    /// The position is in screen coordinates with an upper left origin.
    fn set_frame (&self, frame: Rect) -> Status {
        unsafe { Status::from(p::puglSetFrame(self.view(), frame.into())) }
    }

    /// Set the default size of the view.
    ///
    /// This should be called before [`show_window()`](#method.show_window) and
    /// [`realize()`](#method.realize) to set the default size of
    /// the view, which will be the initial size of the window if this
    /// is a top level view.
    fn set_default_size(&self, width: i32, height: i32) -> Status {
        unsafe { Status::from(p::puglSetDefaultSize(self.view(), width, height)) }
    }

    /// Set the minimum size of the view.
    ///
    /// If an initial minimum size is known, this should be called
    /// before [`realize()`](#method.realize) and [`show_window()`](#method.show_window)
    /// to avoid stutter, though it can be called afterwards as well.
    fn set_min_size (&self, width: i32, height: i32) -> Status {
        unsafe { Status::from(p::puglSetMinSize(self.view(), width, height)) }
    }

    /// Set the maximum size of the view.
    ///
    /// If an initial maximum size is known, this should be called
    /// before [`realize()`](#method.realize) and [`show_window()`](#method.show_window) to
    /// avoid stutter, though it can be called afterwards as well.
    fn set_max_size (&self, width: i32, height: i32) -> Status {
        unsafe { Status::from(p::puglSetMaxSize(self.view(), width, height)) }
    }

    /// Set the view aspect ratio range.
    ///
    /// The x and y values here represent a ratio of width to height.  To set a
    /// fixed aspect ratio, set the minimum and maximum values to the same ratio.

    /// Note that setting different minimum and maximum constraints does not
    /// currenty work on MacOS (the minimum is used), so only setting a fixed aspect
    /// ratio works properly across all platforms.
    ///
    /// If an initial aspect ratio is known, this should be called
    /// before [`realize()`](#method.realize) and [`show_window()`](#method.show_window) to avoid stutter,
    /// though it can be called afterwards as well
    fn set_aspect_ratio(&self, min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Status {
        unsafe { Status::from(p::puglSetAspectRatio(self.view(), min_x, min_y, max_x, max_y)) }
    }

    /// Make the view resizable.
    ///
    /// This should be called before [[`show_window()`](#method.show_window)](#method.show_window) and [`realize()`](#method.realize).
    fn make_resizable(&self) -> Status {
        unsafe {
            Status::from(p::puglSetViewHint(
                self.view(),
                p::PuglViewHint_PUGL_RESIZABLE, p::PuglViewHintValue_PUGL_TRUE))
        }
    }

    /// Sets the window title
    fn set_window_title (&self, title: &str) -> Status {
        unsafe { Status::from(p::puglSetWindowTitle(self.view(), title.as_ptr() as *const i8)) }
    }

    /// Realize a view by creating a corresponding system view or window.
    ///
    /// After this call, the (initially invisible) underlying system
    /// view exists and can be accessed with
    /// [`PuglView::native_window()`](struct.PuglView.html#method.native_window).
    /// There is currently no corresponding unrealize function, the
    /// system view will be destroyed along with the view when the
    /// [`PuglView`](struct.PuglView.html) is dropped.
    ///
    /// The view should be fully configured using the above functions before this is
    /// called.  This function may only be called once per view.
    fn realize(&self) -> Status {
        unsafe { Status::from(p::puglRealize(self.view())) }
    }

    /// Show the view.
    ///
    /// If the view has not yet been realized, the first call to this
    /// function will do so automatically.
    ///
    /// If the view is currently hidden, it will be shown and possibly
    /// raised to the top depending on the platform.
    fn show_window(&self) -> Status {
        unsafe { Status::from(p::puglShowWindow(self.view())) }
    }

    /// Hide the current window
    fn hide_window(&self) -> Status {
        unsafe { Status::from(p::puglHideWindow(self.view())) }
    }

    /// Return true iff the view is currently visible.
    fn is_visible(&self) -> bool {
        unsafe { p::puglGetVisible(self.view()) }
    }

    /// Set the mouse cursor.
    ///
    /// This changes the system cursor that is displayed when the
    /// pointer is inside the view.  May fail if setting the cursor is
    /// not supported on this system, for example if compiled on X11
    /// without Xcursor support.
    fn set_cursor(&self, c: Cursor) -> Status {
        unsafe { Status::from(p::puglSetCursor(self.view(), c.into())) }
    }

    /// Update by processing events from the window system.
    ///
    /// This function is a single iteration of the main loop, and
    /// should be called repeatedly to update all views.
    ///
    /// If `timeout` is zero, then this function will not block.
    /// Plugins should always use a timeout of zero to avoid blocking
    /// the host.
    ///
    /// If a positive `timeout` is given, then events will be
    /// processed for that amount of time, starting from when this
    /// function was called.
    ///
    /// If a negative `timeout` is given, this function will block
    /// indefinitely until an event occurs.
    ///
    /// For continuously animating programs, a timeout that is a
    /// reasonable fraction of the ideal frame period should be used,
    /// to minimise input latency by ensuring that as many input
    /// events are consumed as possible before drawing.
    ///
    /// ## Returns
    /// `Status::Success` if events are read,
    /// `Status::Failure` if not, or an error.
    fn update (&self, timeout: f64) -> Status {
        unsafe { Status::from(p::puglUpdate(self.world(), timeout)) }
    }

    /// Activate a repeating timer event.
    ///
    /// This starts a timer which will send a PuglEventTimer to view
    /// every timeout seconds.  This can be used to perform some
    /// action in a view at a regular interval with relatively low
    /// frequency. Note that the frequency of timer events may be
    /// limited by how often puglUpdate() is called.
    ///
    ///
    /// If the given timer already exists, it is replaced.

    /// ## Parameters
    ///
    /// * `id` – The identifier for this timer. This is an
    ///    application-specific ID that should be a low number,
    ///    typically the value of a constant or enum that starts from
    ///    0. There is a platform-specific limit to the number of
    ///    supported timers, and overhead associated with each, so
    ///    applications should create only a few timers and perform
    ///    several tasks in one if necessary.
    ///
    /// * `timeout` – The period, in seconds, of this timer. This is
    ///   not guaranteed to have a resolution better than 10ms (the
    ///   maximum timer resolution on Windows) and may be rounded up
    ///   if it is too short. On X11 and MacOS, a resolution of about
    ///   1ms can usually be relied on.
    ///
    /// ## Returns
    /// `Status::Success` or `Status::Failure` if timers are not
    /// supported on the system
    fn start_timer(&self, id: usize, timeout: f64) -> Status {
        unsafe { Status::from(p::puglStartTimer(self.view(), id, timeout)) }
    }

    /// Stop an active timer
    ///
    /// ## Parameters
    /// * `id` – The ID previously passed to [`start_timer()`](#method.start_timer)
    ///
    /// ## Returns
    /// `Status::Success` or `Status::Failure` if no such timer was found.
    fn stop_timer(&self, id: usize) -> Status {
        unsafe { Status::from(p::puglStopTimer(self.view(), id)) }
    }
}

/// A struct for a pugl UI object
/// `T` is struct implementing the [`PuglViewTrait`](trait.PuglViewTrait.html),
/// representing the UI's state
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
        },
        p::PuglEventType_PUGL_FOCUS_IN => {
            return handle.focus_in() as p::PuglStatus
        },
        p::PuglEventType_PUGL_FOCUS_OUT => {
            return handle.focus_out() as p::PuglStatus
        },
        p::PuglEventType_PUGL_TIMER => {
            return handle.timer_event(ev.timer.id) as p::PuglStatus
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
    /// Sets up a new `PuglView` for a heap allocated object of `T` implementing
    /// [`PuglViewTrait`](trait.PuglViewTrait.html).
    ///
    /// Can be called with a closure taking a [`PuglViewFFI`](type.PuglViewFFI.html)
    /// returning an [`PuglViewTrait`](trait.PuglViewTrait) object.
    ///
    /// The trait object should retain the `PuglViewFFI` pointer to implement
    /// [`PuglViewTrait:view()`](trait.PuglViewTrait.html#tymethod.view).
    pub fn new<F>(parent_window: *mut std::ffi::c_void, new: F) -> Box<Self>
    where F: FnOnce(PuglViewFFI) -> T {
        let view = Box::new(PuglView::<T> {
            instance: unsafe {
                p::puglNewView(p::puglNewWorld(p::PuglWorldType_PUGL_PROGRAM, 0))
            },
            ui_type: PhantomData
        });

        let ui = Box::new(new(view.instance));
        unsafe {
            if !parent_window.is_null() {
                p::puglSetParentWindow(view.instance, parent_window as usize);
            }
            p::puglSetHandle(view.instance, Box::into_raw(ui) as p::PuglHandle);
            p::puglSetEventFunc(view.instance, Some(event_handler::<T>));
            p::puglSetBackend(view.instance, p::puglCairoBackend());
            p::puglSetViewHint(view.instance, p::PuglViewHint_PUGL_IGNORE_KEY_REPEAT, true as i32);
        }
        view
    }

    /// Returns a handle to the object `T`
    pub fn handle(&mut self) -> &mut T {
        unsafe {
            &mut *(p::puglGetHandle(self.instance) as *mut T) as &mut T
        }
    }

    /// Returns a handle to the window system's view
    pub fn view(&self) -> PuglViewFFI {
        self.instance
    }

    /// Retuns a handle to the native window
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_coord() {
        let c = Coord { x: 2., y: 3. };
        let a = Coord { x: 3., y: 4. };

        let s = c + a;
        assert_eq!(s.x, 5.);
        assert_eq!(s.y, 7.);
    }

    #[test]
    fn add_assign_coord() {
        let mut c = Coord { x: 2., y: 3. };
        let a = Coord { x: 3., y: 4. };

        c += a;
        assert_eq!(c.x, 5.);
        assert_eq!(c.y, 7.);
    }

    #[test]
    fn add_size() {
        let c = Size { w: 2., h: 3. };
        let a = Size { w: 3., h: 4. };

        let s = c + a;
        assert_eq!(s.w, 5.);
        assert_eq!(s.h, 7.);
    }

    #[test]
    fn from_pugl_rect_to_rect() {
        let pr = p::PuglRect {
            x: 2.,
            y: 3.,
            width: 4.,
            height: 5.,
        };

        let r = Rect::from(pr);
        assert_eq!(r.pos.x, 2.);
        assert_eq!(r.pos.y, 3.);
        assert_eq!(r.size.w, 4.);
        assert_eq!(r.size.h, 5.);
    }

    #[test]
    fn from_rect_to_pugl_rect() {
        let r = Rect {
            pos: Coord { x: 2., y: 3. },
            size: Size { w: 4., h: 5. }
        };

        let pr = p::PuglRect::from(r);
        assert_eq!(pr.x,  2.);
        assert_eq!(pr.y,  3.);
        assert_eq!(pr.width, 4.);
        assert_eq!(pr.height, 5.);
    }


    fn key_tuples() -> Vec<(p::PuglKey, SpecialKey)> {
        vec![
            (p::PuglKey_PUGL_KEY_BACKSPACE, SpecialKey::Backspace),
            (p::PuglKey_PUGL_KEY_ESCAPE, SpecialKey::Escape),
            (p::PuglKey_PUGL_KEY_DELETE, SpecialKey::Delete),
            (p::PuglKey_PUGL_KEY_F1, SpecialKey::F1),
            (p::PuglKey_PUGL_KEY_F2, SpecialKey::F2),
            (p::PuglKey_PUGL_KEY_F3, SpecialKey::F3),
            (p::PuglKey_PUGL_KEY_F4, SpecialKey::F4),
            (p::PuglKey_PUGL_KEY_F5, SpecialKey::F5),
            (p::PuglKey_PUGL_KEY_F6, SpecialKey::F6),
            (p::PuglKey_PUGL_KEY_F7, SpecialKey::F7),
            (p::PuglKey_PUGL_KEY_F8, SpecialKey::F8),
            (p::PuglKey_PUGL_KEY_F9, SpecialKey::F9),
            (p::PuglKey_PUGL_KEY_F10, SpecialKey::F10),
            (p::PuglKey_PUGL_KEY_F11, SpecialKey::F11),
            (p::PuglKey_PUGL_KEY_F12, SpecialKey::F12),
            (p::PuglKey_PUGL_KEY_LEFT, SpecialKey::Left),
            (p::PuglKey_PUGL_KEY_UP, SpecialKey::Up),
            (p::PuglKey_PUGL_KEY_RIGHT, SpecialKey::Right),
            (p::PuglKey_PUGL_KEY_DOWN, SpecialKey::Down),
            (p::PuglKey_PUGL_KEY_PAGE_UP, SpecialKey::PageUp),
            (p::PuglKey_PUGL_KEY_PAGE_DOWN, SpecialKey::PageDown),
            (p::PuglKey_PUGL_KEY_HOME, SpecialKey::Home),
            (p::PuglKey_PUGL_KEY_END, SpecialKey::End),
            (p::PuglKey_PUGL_KEY_INSERT, SpecialKey::Insert),
            (p::PuglKey_PUGL_KEY_SHIFT_L, SpecialKey::ShiftL),
            (p::PuglKey_PUGL_KEY_SHIFT_R, SpecialKey::ShiftR),
            (p::PuglKey_PUGL_KEY_CTRL_L, SpecialKey::CtrlL),
            (p::PuglKey_PUGL_KEY_ALT_L, SpecialKey::AltL),
            (p::PuglKey_PUGL_KEY_CTRL_R, SpecialKey::CtrlR),
            (p::PuglKey_PUGL_KEY_ALT_R, SpecialKey::AltR),
            (p::PuglKey_PUGL_KEY_SUPER_L, SpecialKey::SuperL),
            (p::PuglKey_PUGL_KEY_SUPER_R, SpecialKey::SuperR),
            (p::PuglKey_PUGL_KEY_MENU, SpecialKey::KeyMenu),
            (p::PuglKey_PUGL_KEY_CAPS_LOCK, SpecialKey::KeyCapsLock),
            (p::PuglKey_PUGL_KEY_SCROLL_LOCK, SpecialKey::KeyScrollLock),
            (p::PuglKey_PUGL_KEY_NUM_LOCK, SpecialKey::KeyNumLock),
            (p::PuglKey_PUGL_KEY_PRINT_SCREEN, SpecialKey::KeyPrintScreen),
            (p::PuglKey_PUGL_KEY_PAUSE, SpecialKey::KeyPause)
        ]
    }

    #[test]
    fn from_pugl_key_to_special_key() {
        let kt = key_tuples();
        for (pk, sk) in kt {
            assert!(SpecialKey::from(pk) == sk)
        }
    }

    #[test]
    fn from_pugl_key_to_special_key_no_special_key() {
        assert!(SpecialKey::from(42) == SpecialKey::None)
    }

    #[test]
    fn from_pugl_event_flags_default() {
        let ef: EventFlag = 0;
        assert!(!EventFlags::from_bits_truncate(ef).contains(EventFlags::IS_SEND_EVENT));
        assert!(!EventFlags::from_bits_truncate(ef).contains(EventFlags::IS_HINT));
    }

    #[test]
    fn from_pugl_event_flags_is_send_event() {
        let send_event: EventFlag = p::PuglEventFlag_PUGL_IS_SEND_EVENT;
        assert!(EventFlags::from_bits_truncate(send_event).contains(EventFlags::IS_SEND_EVENT));
        assert!(!EventFlags::from_bits_truncate(send_event).contains(EventFlags::IS_HINT));
    }

    #[test]
    fn from_pugl_event_flags_is_hint() {
        let is_hint: EventFlag = p::PuglEventFlag_PUGL_IS_HINT;
        assert!(!EventFlags::from_bits_truncate(is_hint).contains(EventFlags::IS_SEND_EVENT));
        assert!(EventFlags::from_bits_truncate(is_hint).contains(EventFlags::IS_HINT));
    }

    #[test]
    fn from_pugl_event_flags_is_both() {
        let is_both: EventFlag = p::PuglEventFlag_PUGL_IS_HINT | p::PuglEventFlag_PUGL_IS_SEND_EVENT;
        assert!(EventFlags::from_bits_truncate(is_both).contains(EventFlags::IS_SEND_EVENT));
        assert!(EventFlags::from_bits_truncate(is_both).contains(EventFlags::IS_HINT));
    }

}
