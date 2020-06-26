//! A Rust wrapper for pugl
//!
//! This crate is wraper for [pugl](https://gitlab.com/lv2/pugl/), a
//! minimal portable API for embeddable GUIs.
//!
//! Since this crate depends on `-sys` crates that use `bindgen` to
//! create the C API bindings, you need to have clang installed on
//! your machine.
//!
//! `pugl` in principle supports several graphical backends. This
//! crate so far supports only the Cairo backend. So all the drawing
//! is done using the [cairo-rs](https://crates.io/crates/cairo-rs)
//! crate.
//!
//! # Example
//!
//! ```
//! use pugl_sys::pugl::*;
//! use std::f64::consts::PI;
//!
//! // Make an struct which has all the application logic
//! struct UI {
//!     view: PuglViewFFI,
//!     red: f64,
//!     green: f64,
//!     width: f64,
//!     height: f64,
//!     close_requested: bool
//! }
//!
//! // Make the UI struct implement the PuglViewTrait
//! impl PuglViewTrait for UI {
//!     fn new(view: PuglViewFFI) -> Box<Self> {
//!         Box::new(Self {
//!             view,
//!             red: 1.0,
//!             green: 0.0,
//!             width: 800.0,
//!             height: 600.0,
//!             close_requested: false
//!         })
//!     }
//!     // exposure events
//!     fn exposed(&mut self, _expose: &ExposeArea, cr: &cairo::Context) {
//!         let radius = self.width.min(self.height) / 3.0;
//!
//!         cr.set_source_rgb(0., 0., 1.);
//!         cr.rectangle(0., 0., self.width, self.height);
//!         cr.fill();
//!
//!         cr.set_source_rgb(self.red, self.green, 0.0);
//!         cr.arc(self.width/2.0, self.height/2.0, radius, 0.0, 2.0 * PI);
//!         cr.fill();
//!     }
//!
//!     // input events
//!     fn event(&mut self, ev: Event) -> Status {
//!         match ev.data {
//!             EventType::MouseMove(_) => {
//!                 let pos = ev.pos();
//!                 self.red = pos.x / self.width;
//!                 self.green = pos.y / self.height;
//!                 self.post_redisplay();
//!             }
//!             _ => {}
//!         }
//!         Status::Success
//!     }
//!
//!     // a window resize event
//!     fn resize(&mut self, size: Size) {
//!         self.width = size.w;
//!         self.height = size.h;
//!         self.post_redisplay();
//!     }
//!
//!     // a window close event
//!     fn close_request(&mut self) {
//!         self.close_requested = true;
//!     }
//!
//!     fn view(&self) -> PuglViewFFI {
//!         self.view
//!     }
//! }
//!
//! fn main() {
//!     // Request a PuglView passing an initialized UI struct in a Box
//!     let mut view = PuglView::<UI>::new(std::ptr::null_mut());
//!     // borrow the UI handle from the view and do some window initialization
//!     let ui = view.handle();
//!     ui.set_window_title("Test Pugl");
//!     ui.make_resizable();
//!     ui.set_default_size(ui.width.round() as i32, ui.height.round() as i32);
//!     ui.show_window();
//!
//!     // event loop until a close even occurs
//!     while !ui.close_requested {
//!         ui.update(-1.0);
//!     }
//! }
//! ```
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate cairo;
extern crate cairo_sys;

#[macro_use]
extern crate bitflags;

#[macro_use]
pub mod pugl;

pub use pugl::*;
