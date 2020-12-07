use std::marker::PhantomData;

use crate::types::*;
use crate::pugl as p;

pub type PuglViewFFI = *mut p::PuglView;

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

    /// Returns true iff the window is resizable
    fn is_resizable(&self) -> bool {
        unsafe {
            p::puglGetViewHint(self.view(), p::PuglViewHint_PUGL_RESIZABLE) != 0
        }
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

    /// Returns a [`ViewHintBool`](enum.ViewHintBool.html) whether the view is ignoring
    /// key repeats.
    fn is_ignoring_key_repeats(&self) -> ViewHintBool {
        unsafe {
            ViewHintBool::from(p::puglGetViewHint(self.view(), p::PuglViewHint_PUGL_IGNORE_KEY_REPEAT))
        }
    }

    /// Gives the view the hint whether it should ignore key repeats.
    fn set_ignore_key_repeats(&self, value: ViewHintBool) -> Status {
        unsafe {
            Status::from(p::puglSetViewHint(
                self.view(),
                p::PuglViewHint_PUGL_IGNORE_KEY_REPEAT,
                p::PuglViewHintValue::from(value)))
        }
    }

    /// Returns the number of bits for the red channel of the view
    fn red_bits(&self) -> u32 {
        unsafe {
            p::puglGetViewHint(self.view(), p::PuglViewHint_PUGL_RED_BITS) as u32
        }
    }

    /// Returns the number of bits for the green channel of the view
    fn green_bits(&self) -> u32 {
        unsafe {
            p::puglGetViewHint(self.view(), p::PuglViewHint_PUGL_GREEN_BITS) as u32
        }
    }

    /// Returns the number of bits for the blue channel of the view
    fn blue_bits(&self) -> u32 {
        unsafe {
            p::puglGetViewHint(self.view(), p::PuglViewHint_PUGL_BLUE_BITS) as u32
        }
    }

    /// Returns the number of bits for the alpha channel of the view
    fn alpha_bits(&self) -> u32 {
        unsafe {
            p::puglGetViewHint(self.view(), p::PuglViewHint_PUGL_ALPHA_BITS) as u32
        }
    }

    /// Returns the number of bits for the depth buffer of the view
    fn depth_bits(&self) -> u32 {
        unsafe {
            p::puglGetViewHint(self.view(), p::PuglViewHint_PUGL_DEPTH_BITS) as u32
        }
    }

    /// Returns the number of bits for the stencil buffer of the view
    fn stencil_bits(&self) -> u32 {
        unsafe {
            p::puglGetViewHint(self.view(), p::PuglViewHint_PUGL_STENCIL_BITS) as u32
        }
    }

    /// Returns the number of samples per pixel
    fn samples(&self) -> u32 {
        unsafe {
            p::puglGetViewHint(self.view(), p::PuglViewHint_PUGL_SAMPLES) as u32
        }
    }

    /// Returns true iff double buffering should be used
    fn double_buffer(&self) -> bool {
        unsafe {
            p::puglGetViewHint(self.view(), p::PuglViewHint_PUGL_DOUBLE_BUFFER) == 1
        }
    }

    /// Sets whether double buffering should be used
    fn set_double_buffer(&self, yn: bool) -> Status {
        let v = if yn {
            1
        } else {
            0
        };
        unsafe {
            Status::from(p::puglSetViewHint(self.view(), p::PuglViewHint_PUGL_DOUBLE_BUFFER, v))
        }
    }

    /// Returns number of frames between buffer swaps
    fn swap_interval(&self) -> ViewHintInt {
        unsafe {
            ViewHintInt::from(p::puglGetViewHint(self.view(), p::PuglViewHint_PUGL_SWAP_INTERVAL))
        }
    }

    /// Returns the refresh rate in Hz
    fn refresh_rate(&self) -> ViewHintInt {
        unsafe {
            ViewHintInt::from(p::puglGetViewHint(self.view(), p::PuglViewHint_PUGL_REFRESH_RATE))
        }
    }

    /// Sets the window title
    fn set_window_title(&self, title: &str) -> Status {
        let title =
            std::ffi::CString::new(title.as_bytes())
                .expect("window title must not contain 0 bytes");
        unsafe {
            Status::from(p::puglSetWindowTitle(self.view(), title.into_raw()))
        }
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
    instance: PuglViewFFI
}


unsafe extern "C"
fn event_handler<T: PuglViewTrait>(view_ptr: *mut p::PuglView, event_ptr: *const p::PuglEvent) -> p::PuglStatus {
    let ev = *event_ptr;
    let handle: &mut T = &mut *(p::puglGetHandle(view_ptr) as *mut T);
    //eprintln!("event_handler: {:?}", ev.type_);
    let event = match ev.type_ {
        p::PuglEventType_PUGL_KEY_PRESS => {
            Event { data: EventType::KeyPress(Key::from(ev.key)), context: EventContext::from(ev.key) }
        },
        p::PuglEventType_PUGL_KEY_RELEASE => {
            Event { data: EventType::KeyRelease(Key::from(ev.key)), context: EventContext::from(ev.key) }
        },
        p::PuglEventType_PUGL_BUTTON_PRESS => {
            Event { data: EventType::MouseButtonPress(MouseButton::from(ev.button)), context: EventContext::from(ev.button) }
        },
        p::PuglEventType_PUGL_BUTTON_RELEASE => {
            Event { data: EventType::MouseButtonRelease(MouseButton::from(ev.button)), context: EventContext::from(ev.button) }
        },
        p::PuglEventType_PUGL_MOTION_NOTIFY => {
            Event { data: EventType::MouseMove(MotionContext::from(ev.motion)), context: EventContext::from(ev.motion) }
        },
        p::PuglEventType_PUGL_POINTER_IN => {
            Event { data: EventType::PointerIn, context: EventContext::from(ev.crossing) }
        }
        p::PuglEventType_PUGL_POINTER_OUT => {
            Event { data: EventType::PointerOut, context: EventContext::from(ev.crossing) }
        }
        p::PuglEventType_PUGL_SCROLL => {
            Event { data: EventType::Scroll(Scroll::from(ev.scroll)), context: EventContext::from(ev.scroll) }
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
            handle.exposed (&ExposeArea::from(ev.expose), &cr);
            return p::PuglStatus_PUGL_SUCCESS
        },
        p::PuglEventType_PUGL_CONFIGURE => {
            let size = Size::from(ev.configure);
            handle.resize (size);
            return p::PuglStatus_PUGL_SUCCESS
        },
        _  => { return p::PuglStatus_PUGL_SUCCESS }

    };
    handle.event (event) as p::PuglStatus
}

#[cfg(test)]
unsafe fn get_backend() -> *const p::PuglBackend {
    p::puglStubBackend()
}
#[cfg(not (test))]
unsafe fn get_backend() -> *const p::PuglBackend {
    p::puglCairoBackend()
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
            p::puglSetBackend(view.instance, get_backend());
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

    struct UI {
        view: PuglViewFFI
    }

    impl UI {
        fn new(view: PuglViewFFI) -> Self { Self { view } }
    }

    impl PuglViewTrait for UI {
        fn event(&mut self, _ev: Event) -> Status {
            Status::Success
        }
        fn exposed(&mut self, _expose: &ExposeArea, _cr: &cairo::Context) {}
        fn resize(&mut self, _size: Size) {}
        fn close_request(&mut self) {}
        fn view(&self) -> PuglViewFFI {
            self.view
        }
    }

    #[test]
    #[serial]
    fn unresizable() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        let ui = view.handle();

        ui.set_default_size(42, 23);
        ui.show_window();
        assert!(!ui.is_resizable());
    }

    #[test]
    #[serial]
    fn resizable() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        let ui = view.handle();

        ui.set_default_size(42, 23);
        ui.make_resizable();
        ui.show_window();
        assert!(ui.is_resizable())
    }

    #[test]
    #[serial]
    fn not_ignore_key_repeat() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        let ui = view.handle();

        ui.set_default_size(42, 23);
        ui.set_ignore_key_repeats(ViewHintBool::False);
        ui.show_window();
        assert_eq!(ui.is_ignoring_key_repeats(), ViewHintBool::False);
    }

    #[test]
    #[serial]
    fn ignore_key_repeat() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        let ui = view.handle();

        ui.set_default_size(42, 23);
        ui.show_window();
        assert_eq!(ui.is_ignoring_key_repeats(), ViewHintBool::True);
    }

    #[serial]
    #[test]
    fn red_bits() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        let ui = view.handle();
        assert_eq!(ui.red_bits(), 8);
    }

    #[serial]
    #[test]
    fn green_bits() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        let ui = view.handle();
        assert_eq!(ui.green_bits(), 8);
    }

    #[serial]
    #[test]
    fn blue_bits() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        let ui = view.handle();
        assert_eq!(ui.blue_bits(), 8);
    }

    #[serial]
    #[test]
    fn alpha_bits() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        let ui = view.handle();
        assert_eq!(ui.alpha_bits(), 8);
    }

    #[serial]
    #[test]
    fn depth_bits() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        let ui = view.handle();
        assert_eq!(ui.depth_bits(), 0);
    }

    #[serial]
    #[test]
    fn stencil_bits() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        let ui = view.handle();
        assert_eq!(ui.stencil_bits(), 0);
    }

    #[serial]
    #[test]
    fn samples() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        let ui = view.handle();
        assert_eq!(ui.samples(), 0);
    }

    #[serial]
    #[test]
    fn double_buffer() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        let ui = view.handle();
        assert_eq!(ui.double_buffer(), true);
        ui.set_double_buffer(false);
        assert_eq!(ui.double_buffer(), false);
    }

    #[serial]
    #[test]
    fn swap_interaval() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        let ui = view.handle();
        assert_eq!(ui.swap_interval(), ViewHintInt::DontCare);
        ui.set_default_size(42, 23);
        ui.show_window();
        assert_eq!(ui.swap_interval(), ViewHintInt::DontCare);
    }

    #[serial]
    #[test]
    fn refresh_rate() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        let ui = view.handle();
        assert_eq!(ui.refresh_rate(), ViewHintInt::DontCare);
        ui.set_default_size(42, 23);
        ui.show_window();
        assert_ne!(ui.refresh_rate(), ViewHintInt::DontCare);
    }
}
