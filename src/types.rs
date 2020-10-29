
use std::ops::{Add, AddAssign};
use std::char;

use crate::pugl as p;

/// Representing coordinates on a widget
///
#[derive(Copy, Clone, Default, Debug, PartialEq)]
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
#[derive(Copy, Clone, Default, Debug, PartialEq)]
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
#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct EventContext {
    /// View relative position of the event
    pub pos: Coord,
    /// Root relative position of the event
    pub pos_root: Coord,
    /// The time of the event in seconds
    pub time: f64
}

/// Keys not representing a character
#[derive(Copy, Clone, PartialEq, Debug)]
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
    #[derive(Default)]
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

/// Representing a key from the keyboard
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum KeyVal {
    /// A Character key
    Character(char),
    /// A special key (non-character)
    Special(SpecialKey)
}

/// Key with keyboard modifiers
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Key {
    /// The actual key
    pub key: KeyVal,
    /// The modifiers to be used with the [`Modifiers`](struct.Modifiers.html) struct
    pub modifiers: Modifiers,
    /// System's code for the key
    pub code: KeyCode
}

impl Key {
    /// Returns the character if the key is a character key, otherwise `None`
    ///
    /// ```
    /// let char_key = pugl_sys::Key {
    ///     key: pugl_sys::KeyVal::Character('A'),
    ///     modifiers: pugl_sys::Modifiers::SHIFT,
    ///     code: 38
    /// };
    /// let special_key = pugl_sys::Key {
    ///     key: pugl_sys::KeyVal::Special(pugl_sys::SpecialKey::F1),
    ///     modifiers: pugl_sys::Modifiers::NONE,
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
            modifiers: Modifiers::from_bits_truncate(ke.state)
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
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MouseButton {
    /// The number of the mouse button
    pub num: u32,
    /// Keyboard modifiers, when the mouse button event happened
    /// to be used with the [`Modifiers`](struct.Modifiers.html) struct.
    pub modifiers: Modifiers
}

impl From<p::PuglEventButton> for MouseButton {
    fn from(be: p::PuglEventButton) -> MouseButton {
        MouseButton {
            num: be.button,
            modifiers: Modifiers::from_bits_truncate(be.state)
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

bitflags! {
    #[derive(Default)]
    pub struct EventFlags: u32 {
        const NONE = 0;
        const IS_SEND_EVENT = 1;
        const IS_HINT = 2;
    }
}

/// Context of a pointer event
#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct MotionContext {
    /// Keyboard modifiers to be used with the [`Modifiers`](struct.Modifiers.html) struct.
    pub modifiers: Modifiers,
    /// The event flags
    pub flags: EventFlags
}

impl From<p::PuglEventMotion> for MotionContext {
    fn from (me: p::PuglEventMotion) -> MotionContext {
        MotionContext {
            modifiers: Modifiers::from_bits_truncate(me.state),
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
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Scroll {
    /// horizontal scroll distance
    pub dx: f64,
    /// vertical scroll distance
    pub dy: f64,
    pub modifiers: Modifiers
}

impl From<p::PuglEventScroll> for Scroll {
    fn from (se: p::PuglEventScroll) -> Scroll {
        Scroll {
            dx: se.dx, dy: se.dy,
            modifiers: Modifiers::from_bits_truncate(se.state)
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

impl From<p::PuglEventCrossing> for EventContext {
    fn from(pce: p::PuglEventCrossing) -> EventContext {
        EventContext {
            pos: Coord { x: pce.x, y: pce.y },
            pos_root: Coord { x: pce.xRoot, y: pce.yRoot },
            time: pce.time
        }
    }
}

impl From<p::PuglEventConfigure> for Size {
    fn from (ce: p::PuglEventConfigure) -> Size {
        Size { w: ce.width, h: ce.height }
    }
}

/// The area that needs to be redrawn due to an expose event
#[derive(Copy, Clone, Debug, PartialEq)]
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
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum EventType {
    KeyPress(Key),
    KeyRelease(Key),
    MouseButtonPress(MouseButton),
    MouseButtonRelease(MouseButton),
    MouseMove(MotionContext),
    PointerIn,
    PointerOut,
    Scroll(Scroll)
}

/// An event signaled by the windowing system
#[derive(Copy, Clone, PartialEq, Debug)]
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

impl Default for Cursor {
    fn default() -> Self {
        Cursor::Arrow
    }
}


/// Return status code.
#[repr(u32)]
#[derive(Debug, PartialEq)]
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
    BadConfiguration,
    /// Invalid view configuration
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
            p::PuglStatus_PUGL_BAD_CONFIGURATION => Status::BadConfiguration ,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewHintBool {
    True,
    False,
    DontCare
}

impl From<p::PuglViewHintValue> for ViewHintBool {
    fn from(pvhv: p::PuglViewHintValue) -> ViewHintBool {
        match pvhv {
            p::PuglViewHintValue_PUGL_TRUE => ViewHintBool::True,
            p::PuglViewHintValue_PUGL_FALSE => ViewHintBool::False,
            _ => ViewHintBool::DontCare
        }
    }
}

impl From<ViewHintBool> for p::PuglViewHintValue {
    fn from(vh: ViewHintBool) -> p::PuglViewHintValue{
        match vh {
            ViewHintBool::True => p::PuglViewHintValue_PUGL_TRUE,
            ViewHintBool::False => p::PuglViewHintValue_PUGL_FALSE,
            _ => p::PuglViewHintValue_PUGL_DONT_CARE
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewHintInt {
    Value(u32),
    DontCare
}

impl From<p::PuglViewHintValue> for ViewHintInt {
    fn from(pvhv: p::PuglViewHintValue) -> ViewHintInt {
        if pvhv < 0 {
            ViewHintInt::DontCare
        } else {
            ViewHintInt::Value(pvhv as u32)
        }
    }
}

impl From<ViewHintInt> for p::PuglViewHintValue {
    fn from(vh: ViewHintInt) -> p::PuglViewHintValue {
        match vh {
            ViewHintInt::Value(v) => v as i32,
            _ => p::PuglViewHintValue_PUGL_DONT_CARE
        }
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

    fn pugl_event_key_press_small_a() -> p::PuglEventKey {
        p::PuglEventKey {
            type_: p::PuglEventType_PUGL_KEY_PRESS,
            flags: 0,
            time: 2.0,
            x: 23.0,
            y: 42.0,
            xRoot: 123.0,
            yRoot: 142.0,
            state: 0,
            keycode: 38,
            key: 0x61, // 'a'
        }
    }

    fn pugl_event_key_release_capital_a() -> p::PuglEventKey {
        p::PuglEventKey {
            type_: p::PuglEventType_PUGL_KEY_PRESS,
            flags: 0,
            time: 2.0,
            x: 23.0,
            y: 42.0,
            xRoot: 123.0,
            yRoot: 142.0,
            state: p::PuglMod_PUGL_MOD_SHIFT,
            keycode: 38,
            key: 0x41, // 'A'
        }
    }

    fn pugl_event_key_press_f1() -> p::PuglEventKey {
        p::PuglEventKey {
            type_: p::PuglEventType_PUGL_KEY_PRESS,
            flags: 0,
            time: 2.0,
            x: 23.0,
            y: 42.0,
            xRoot: 123.0,
            yRoot: 142.0,
            state: p::PuglMod_PUGL_MOD_CTRL | p::PuglMod_PUGL_MOD_ALT,
            keycode: p::PuglKey_PUGL_KEY_F1,
            key: 0, // 'A'
        }
    }

    #[test]
    fn from_pugl_key_to_keyval() {
        let kv = KeyVal::from(pugl_event_key_press_small_a());
        assert_eq!(kv, KeyVal::Character('a'));
        let kv = KeyVal::from(pugl_event_key_release_capital_a());
        assert_eq!(kv, KeyVal::Character('A'));
        let kv = KeyVal::from(pugl_event_key_press_f1());
        assert_eq!(kv, KeyVal::Special(SpecialKey::F1));
    }

    #[test]
    fn from_pugl_key_to_key() {
        let key = Key::from(pugl_event_key_press_small_a());
        assert_eq!(key.modifiers, Modifiers::from_bits_truncate(0));
        let key = Key::from(pugl_event_key_release_capital_a());
        assert_eq!(key.modifiers, Modifiers::from_bits_truncate(1));
        let key = Key::from(pugl_event_key_press_f1());
        assert_eq!(key.modifiers, Modifiers::from_bits_truncate(6));
    }

    #[test]
    fn from_pugl_key_to_eventcontext() {
        let ec = EventContext::from(pugl_event_key_press_small_a());
        assert_eq!(ec.pos.x, 23.0);
        assert_eq!(ec.pos.y, 42.0);
        assert_eq!(ec.pos_root.x, 123.0);
        assert_eq!(ec.pos_root.y, 142.0);
        assert_eq!(ec.time, 2.0);
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
    fn from_special_key_to_special_key_no_special_key() {
        assert!(SpecialKey::from(42) == SpecialKey::None)
    }

    #[test]
    fn from_pugl_crossing_to_event_context() {
        let pev_crossing = p::PuglEventCrossing {
            type_: p::PuglEventType_PUGL_POINTER_IN,
            flags: 0,
            time: 2.0,
            x: 23.0,
            y: 42.0,
            xRoot: 123.0,
            yRoot: 142.0,
            state: 0,
            mode: 0
        };
        let ec = EventContext::from(pev_crossing);
        assert_eq!(ec.pos.x, 23.0);
        assert_eq!(ec.pos.y, 42.0);
        assert_eq!(ec.pos_root.x, 123.0);
        assert_eq!(ec.pos_root.y, 142.0);
        assert_eq!(ec.time, 2.0);
    }

    fn pugl_mouse_button() ->  p::PuglEventButton {
        p::PuglEventButton {
            type_: p::PuglEventType_PUGL_BUTTON_PRESS,
            flags: 0,
            time: 2.0,
            x: 23.0,
            y: 42.0,
            xRoot: 123.0,
            yRoot: 142.0,
            state: 2,
            button: 1
        }
    }

    #[test]
    fn from_pugl_button_to_mouse_button() {
        let mb = MouseButton::from(pugl_mouse_button());
        assert_eq!(mb.modifiers.bits, 2);
        assert_eq!(mb.num, 1);
    }

    #[test]
    fn from_pugl_button_to_event_context() {
        let ec = EventContext::from(pugl_mouse_button());
        assert_eq!(ec.pos.x, 23.0);
        assert_eq!(ec.pos.y, 42.0);
        assert_eq!(ec.pos_root.x, 123.0);
        assert_eq!(ec.pos_root.y, 142.0);
        assert_eq!(ec.time, 2.0);
    }

    fn pugl_event_motion() -> p::PuglEventMotion {
        p::PuglEventMotion {
            type_: p::PuglEventType_PUGL_MOTION,
            flags: 0,
            time: 2.0,
            x: 23.0,
            y: 42.0,
            xRoot: 123.0,
            yRoot: 142.0,
            state: 2,
        }
    }

    #[test]
    fn from_pugl_motion_to_motion_context() {
        let mc = MotionContext::from(pugl_event_motion());
        assert_eq!(mc, MotionContext { modifiers: Modifiers::from_bits_truncate(2), flags: EventFlags::NONE });
    }

    #[test]
    fn from_pugl_motion_to_event_context() {
        let ec = EventContext::from(pugl_event_motion());
        assert_eq!(ec.pos.x, 23.0);
        assert_eq!(ec.pos.y, 42.0);
        assert_eq!(ec.pos_root.x, 123.0);
        assert_eq!(ec.pos_root.y, 142.0);
        assert_eq!(ec.time, 2.0);
    }

    fn pugl_scroll_event() -> p::PuglEventScroll {
        p::PuglEventScroll {
            type_: p::PuglEventType_PUGL_SCROLL,
            flags: 0,
            time: 2.0,
            x: 23.0,
            y: 42.0,
            xRoot: 123.0,
            yRoot: 142.0,
            state: 2,
            dx: 3.14,
            dy: 2.71,
            direction: 0
        }
    }

    #[test]
    fn from_pugl_scroll_to_scroll() {
        let sc = Scroll::from(pugl_scroll_event());
        assert_eq!(sc, Scroll {
            dx: 3.14,
            dy: 2.71,
            modifiers: Modifiers::from_bits_truncate(2)
        });
    }

    #[test]
    fn from_pugl_scroll_to_event_context() {
        let ec = EventContext::from(pugl_scroll_event());
        assert_eq!(ec.pos.x, 23.0);
        assert_eq!(ec.pos.y, 42.0);
        assert_eq!(ec.pos_root.x, 123.0);
        assert_eq!(ec.pos_root.y, 142.0);
        assert_eq!(ec.time, 2.0);
    }

    fn pugl_event_expose() -> p::PuglEventExpose {
        p::PuglEventExpose {
            type_: p::PuglEventType_PUGL_EXPOSE,
            flags: 0,
            x: 23.0,
            y: 42.0,
            width: 12.0,
            height: 6.0
        }
    }

    #[test]
    fn from_pugl_expose_to_expose_area() {
        let ea = ExposeArea::from(pugl_event_expose());
        assert_eq!(ea, ExposeArea { pos: Coord { x: 23., y: 42. }, size: Size { w: 12.0, h: 6. }});
    }

    #[test]
    fn from_pugl_event_flags_default() {
        let ef = 0;
        assert!(!EventFlags::from_bits_truncate(ef).contains(EventFlags::IS_SEND_EVENT));
        assert!(!EventFlags::from_bits_truncate(ef).contains(EventFlags::IS_HINT));
    }

    #[test]
    fn from_pugl_event_flags_is_send_event() {
        let send_event = p::PuglEventFlag_PUGL_IS_SEND_EVENT;
        assert!(EventFlags::from_bits_truncate(send_event).contains(EventFlags::IS_SEND_EVENT));
        assert!(!EventFlags::from_bits_truncate(send_event).contains(EventFlags::IS_HINT));
    }

    #[test]
    fn from_pugl_event_flags_is_hint() {
        let is_hint = p::PuglEventFlag_PUGL_IS_HINT;
        assert!(!EventFlags::from_bits_truncate(is_hint).contains(EventFlags::IS_SEND_EVENT));
        assert!(EventFlags::from_bits_truncate(is_hint).contains(EventFlags::IS_HINT));
    }

    #[test]
    fn from_pugl_event_flags_is_both() {
        let is_both = p::PuglEventFlag_PUGL_IS_HINT | p::PuglEventFlag_PUGL_IS_SEND_EVENT;
        assert!(EventFlags::from_bits_truncate(is_both).contains(EventFlags::IS_SEND_EVENT));
        assert!(EventFlags::from_bits_truncate(is_both).contains(EventFlags::IS_HINT));
    }

    #[test]
    fn from_pugl_view_hint_bool_value_true() {
        let pugl_view_hint_true = p::PuglViewHintValue_PUGL_TRUE;
        assert_eq!(ViewHintBool::from(pugl_view_hint_true), ViewHintBool::True);
    }

    #[test]
    fn from_pugl_view_hint_bool_value_false() {
        let pugl_view_hint_false = p::PuglViewHintValue_PUGL_FALSE;
        assert_eq!(ViewHintBool::from(pugl_view_hint_false), ViewHintBool::False);
    }

    #[test]
    fn from_pugl_view_hint_bool_value_dontcare() {
        let pugl_view_hint_dontcare = p::PuglViewHintValue_PUGL_DONT_CARE;
        assert_eq!(ViewHintBool::from(pugl_view_hint_dontcare), ViewHintBool::DontCare);
    }

    #[test]
    fn to_pugl_view_hint_bool_true() {
        let view_hint_true = ViewHintBool::True;
        assert_eq!(p::PuglViewHintValue::from(view_hint_true), p::PuglViewHintValue_PUGL_TRUE)
    }

    #[test]
    fn to_pugl_view_hint_bool_false() {
        let view_hint_false = ViewHintBool::False;
        assert_eq!(p::PuglViewHintValue::from(view_hint_false), p::PuglViewHintValue_PUGL_FALSE)
    }

    #[test]
    fn to_pugl_view_hint_bool_dontcare() {
        let view_hint_dontcare = ViewHintBool::DontCare;
        assert_eq!(p::PuglViewHintValue::from(view_hint_dontcare), p::PuglViewHintValue_PUGL_DONT_CARE)
    }

    #[test]
    fn from_pugl_view_hint_int_value() {
        let pugl_view_hint: i32 = 42;
        assert_eq!(ViewHintInt::from(pugl_view_hint), ViewHintInt::Value(42));
    }

    #[test]
    fn from_pugl_view_hint_int_zero() {
        let pugl_view_hint: i32 = p::PuglViewHintValue_PUGL_FALSE;
        assert_eq!(ViewHintInt::from(pugl_view_hint), ViewHintInt::Value(0));
    }

    #[test]
    fn from_pugl_view_hint_int_dontcare() {
        let pugl_view_hint: i32 = p::PuglViewHintValue_PUGL_DONT_CARE;
        assert_eq!(ViewHintInt::from(pugl_view_hint), ViewHintInt::DontCare);
    }

    #[test]
    fn to_pugl_view_hint_bool_value() {
        let view_hint = ViewHintInt::Value(42);
        assert_eq!(p::PuglViewHintValue::from(view_hint), 42);
    }

    #[test]
    fn to_pugl_view_hint_bool_zero() {
        let view_hint = ViewHintInt::Value(0);
        assert_eq!(p::PuglViewHintValue::from(view_hint), p::PuglViewHintValue_PUGL_FALSE);
    }

    #[test]
    fn to_pugl_view_hint_bool_dont_care() {
        let view_hint = ViewHintInt::DontCare;
        assert_eq!(p::PuglViewHintValue::from(view_hint), p::PuglViewHintValue_PUGL_DONT_CARE);
    }

}
