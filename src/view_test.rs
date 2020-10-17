use std::collections::VecDeque;
use std::marker::PhantomData;

use crate::types::*;
use crate::pugl as p;

pub type PuglViewFFI = *mut PuglViewMock;

pub struct PuglView<T: PuglViewTrait> {
    ui_type: std::marker::PhantomData<T>,
    instance: PuglViewFFI
}


impl<T: PuglViewTrait> PuglView<T> {
    pub fn new<F>(_parent_window: *mut std::ffi::c_void, new: F) -> Box<Self>
    where F: FnOnce(PuglViewFFI) -> T {

        let view = Box::new(PuglView::<T> {
            instance: Box::into_raw(Box::new(PuglViewMock::default())),
            ui_type: PhantomData
        });

        let ui = Box::new(new(view.instance));

        unsafe {
            let ui_ptr = Box::leak(ui) as *mut T;
            (*view.instance).ui_ptr = ui_ptr as *mut std::ffi::c_void;
        }

        view
    }

    pub fn handle(&mut self) -> &mut T {
        unsafe {
            &mut *((*self.instance).ui_ptr as *mut T) as &mut T
        }
    }

    pub fn mock_instance(&self) -> &PuglViewMock {
        unsafe {
            &*self.instance
        }
    }

    pub fn fake_resize(&mut self, size: Size) {
	unsafe {
	    (*self.instance).frame.size = size;
	}

	self.handle().resize(size)
    }

    pub fn fake_focus_in(&mut self) {
        self.handle().focus_in();
    }

    pub fn fake_focus_out(&mut self) {
        self.handle().focus_out();
    }

    pub fn queue_event(&mut self, ev: Event) {
        unsafe {
            (*self.instance).queue_event(ev);
        }
    }
}



pub trait PuglViewTrait {

    fn event(&mut self, ev: Event) -> Status;

    fn exposed (&mut self, _expose: &ExposeArea, _cr: &cairo::Context) {}

    fn resize (&mut self, _size: Size) {}

    fn close_request(&mut self) {}

    fn focus_in(&mut self) -> Status { Status::Success }

    fn focus_out(&mut self) -> Status { Status::Success }

    fn timer_event(&mut self, _id: usize) -> Status { Status::Success }

    fn view (&self) -> PuglViewFFI;

    fn world (&self) -> *mut p::PuglWorld {
        std::ptr::null_mut::<p::PuglWorld>()
    }

    fn post_redisplay (&self) -> Status {
        Status::Success
    }

    fn post_redisplay_rect(&self, pos: Coord, size: Size) -> Status {
        let _p_rect = p::PuglRect {
            x: pos.x,
            y: pos.y,
            width: size.w,
            height: size.h
        };
        Status::Success
    }

    fn get_frame(&self) -> Rect {
        unsafe { (*self.view()).frame }
    }

    fn set_frame (&self, frame: Rect) -> Status {
        unsafe {
            (*self.view()).frame = frame;
        }
        Status::Success
    }

    fn set_default_size(&self, width: i32, height: i32) -> Status {
        unsafe {
            (*self.view()).default_width = width;
            (*self.view()).default_height = height;
        }
        Status::Success
    }

    fn set_min_size (&self, width: i32, height: i32) -> Status {
        unsafe {
            (*self.view()).min_width = width;
            (*self.view()).min_height = height;
        }
        Status::Success
    }

    fn set_max_size (&self, width: i32, height: i32) -> Status {
        unsafe {
            (*self.view()).max_width = width;
            (*self.view()).max_height = height;
        }
        Status::Success
    }

    fn set_aspect_ratio(&self, min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Status {
        unsafe {
            (*self.view()).min_aspect_x = min_x;
            (*self.view()).max_aspect_x = max_x;
            (*self.view()).min_aspect_y = min_y;
            (*self.view()).max_aspect_y = max_y;
        }
        Status::Success
    }

    fn make_resizable(&self) -> Status {
        unsafe {
            (*self.view()).resizable = true;
        }
        Status::Success
    }

    fn set_window_title (&self, title: &str) -> Status {
        unsafe {
            (*self.view()).window_title = title.to_string();
        }
        Status::Success
    }

    fn realize(&self) -> Status {
        let view = unsafe { &mut (*self.view()) };
	if view.default_width * view.default_height == 0 {
	    return Status::BadConfiguration
	}
	view.realized = true;
	view.frame.size.w = view.default_width as f64;
	view.frame.size.h = view.default_height as f64;
        Status::Success
    }

    fn show_window(&self) -> Status {
	let view = unsafe {
	    &mut (*self.view())
	};

	let status = if !view.realized {
	    self.realize()
	} else {
	    Status::Success
	};

	if status == Status::Success {
	    view.visible = true
	}

	status
    }

    fn hide_window(&self) -> Status {
        unsafe {
            (*self.view()).visible = false;
        }
        Status::Success
    }

    fn is_visible(&self) -> bool {
        unsafe { (*self.view()).visible }
    }

    fn set_cursor(&self, c: Cursor) -> Status {
        unsafe {
            (*self.view()).cursor = c;
        }
        Status::Success
    }

    fn update(&mut self, timeout: f64) -> Status {
        let view = unsafe {
            &mut (*self.view())
        };
        view.update_timeout = Some(timeout);
        if let Some(ev) = view.event_queue.pop_front() {
            //eprintln!("Issuing event {:?}", ev);
            self.event(ev);
        }
        Status::Success
    }

    fn start_timer(&self, id: usize, timeout: f64) -> Status {
        unsafe {
            (*self.view()).timer_time.insert(id, timeout);
        }
        Status::Success
    }

    fn stop_timer(&self, id: usize) -> Status {
        match unsafe {
            (*self.view()).timer_time.remove(&id)
        } {
            None => Status::Failure,
            Some(_) => Status::Success
        }
    }
}


pub struct PuglViewMock {
    ui_ptr: *mut std::ffi::c_void,

    frame: Rect,

    default_width: i32,
    default_height: i32,

    min_width: i32,
    min_height: i32,

    max_width: i32,
    max_height: i32,

    min_aspect_x: i32,
    max_aspect_x: i32,
    min_aspect_y: i32,
    max_aspect_y: i32,

    resizable: bool,

    window_title: String,

    visible: bool,
    realized: bool,

    cursor: Cursor,

    update_timeout: Option<f64>,

    event_queue: VecDeque<Event>,

    timer_time: std::collections::HashMap<usize, f64>,
}




impl Default for PuglViewMock {
    fn default() -> Self {
        let mock = Self {
            ui_ptr: std::ptr::null_mut(),
            frame: Default::default(),

            default_width: Default::default(),
            default_height: Default::default(),

            min_width: Default::default(),
            min_height: Default::default(),

            max_width: Default::default(),
            max_height: Default::default(),

            min_aspect_x: Default::default(),
            max_aspect_x: Default::default(),
            min_aspect_y: Default::default(),
            max_aspect_y: Default::default(),

            resizable: Default::default(),

            window_title: Default::default(),

            visible: false,
	    realized: false,

            cursor: Default::default(),

            update_timeout: Default::default(),

            event_queue: VecDeque::new(),

            timer_time: Default::default(),

        };
        mock
    }
}

impl PuglViewMock {
    fn queue_event(&mut self, ev: Event) {
        self.event_queue.push_back(ev);
    }

    pub fn min_size(&self) -> Size {
        Size { w: self.min_width as f64, h: self.min_height as f64 }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[derive(PartialEq, Debug)]
    enum ClickState {
        None,
        Clicked,
        Released
    }

    struct UI {
        view: PuglViewFFI,

        click_state: ClickState,
        pointer_entered: bool,
    }


    impl UI {
        fn new(view: PuglViewFFI) -> Self {
            Self {
                view,
                click_state: ClickState::None,
                pointer_entered: false
            }
        }
    }

    impl PuglViewTrait for UI {
        fn view(&self) -> PuglViewFFI { self.view }

        fn event(&mut self, ev: Event) -> Status {
            match ev.data {
                EventType::MouseButtonPress(_) => self.click_state = ClickState::Clicked,
                EventType::MouseButtonRelease(_) => self.click_state = ClickState::Released,
                EventType::PointerIn => self.pointer_entered = true,
                EventType::PointerOut => self.pointer_entered = false,
                _ => {}
            }

            Status::Success
        }
    }


    fn make_view() -> Box<PuglView<UI>> {
	let view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
	view
    }

    #[test]
    fn default_hidden() {
	let mut view = make_view();
	let ui = view.handle();
	assert!(!ui.is_visible());
    }

    #[test]
    fn show_window_no_size() {
	let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
	let ui = view.handle();
	assert_eq!(ui.show_window(), Status::BadConfiguration);
	assert!(!ui.is_visible());
    }

    #[test]
    fn show_window_default_size() {
	let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
	let ui = view.handle();
	ui.set_default_size(32, 16);
	assert_eq!(ui.show_window(), Status::Success);
	assert!(ui.is_visible());
	let size = unsafe { (*ui.view()).frame.size };
	assert_eq!(size.w, 32.0);
	assert_eq!(size.h, 16.0);
    }

    #[test]
    fn hide_window() {
	let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
	let ui = view.handle();
	ui.set_default_size(32, 16);
	ui.show_window();
	ui.hide_window();
	assert!(!ui.is_visible());
    }

    #[test]
    fn show_resize_hide_show() {
	let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
	{
	    let ui = view.handle();
	    ui.set_default_size(32, 16);
	    ui.show_window();
	}
	view.fake_resize(Size { w: 42., h: 23. });
	let ui = view.handle();
	ui.hide_window();
	ui.show_window();

	let size = unsafe { (*ui.view()).frame.size };
	assert_eq!(size.w, 42.0);
	assert_eq!(size.h, 23.0);
    }

    #[test]
    fn mouse_click_event() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        {
            let ui = view.handle();
            ui.set_default_size(32, 16);
            ui.show_window();
        }
        let event_context = EventContext {
            pos: Coord { x: 16., y: 8.},
            pos_root: Coord { x: 16., y: 8.},
            time: 0.0
        };
        let press_event = Event {
            data: EventType::MouseButtonPress(MouseButton { num: 0, modifiers: 0 }),
            context: event_context
        };
        let release_event = Event {
            data: EventType::MouseButtonRelease(MouseButton { num: 0, modifiers: 0 }),
            context: event_context
        };

        view.queue_event(press_event);
        view.queue_event(release_event);

        let ui = view.handle();
        assert_eq!(ui.click_state, ClickState::None);
        ui.update(-1.0);
        assert_eq!(ui.click_state, ClickState::Clicked);
        ui.update(-1.0);
        assert_eq!(ui.click_state, ClickState::Released);
    }

    #[test]
    fn pointer_enter_leave_event() {
        let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
        view.queue_event(Event { data: EventType::PointerIn, context: EventContext::default() });
        view.queue_event(Event { data: EventType::PointerOut, context: EventContext::default() });

        let ui = view.handle();
        assert!(!ui.pointer_entered);
        ui.update(-1.0);
        assert!(ui.pointer_entered);
        ui.update(-1.0);
        assert!(!ui.pointer_entered);
    }

    #[test]
    fn window_title() {
	let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
	let ui = view.handle();
	ui.set_window_title("Test Pugl");
	unsafe {
	    assert_eq!((*ui.view()).window_title, "Test Pugl");
	}
    }

}
