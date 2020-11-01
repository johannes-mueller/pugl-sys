//! A Rust wrapper for pugl
//!
//! This crate is wrapper for [pugl](https://gitlab.com/lv2/pugl/), a
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
//! # Usage
//!
//! `pugl-sys` provides (maybe unlike classical `*-sys` crates) not
//! only access to the functionality of `pugl` via FFI but also a thin
//! layer to use `pugl` in a safe Rust-like way.
//!
//! In the center there is the
//! [`PuglViewTrait`](trait.PuglViewTrait.html). In order to use
//! `pugl-sys` for your application (or more precisely an UI for an
//! application, set up a struct that implements `PuglViewTrait` an
//! application.
//!
//! # GUI-Toolkit stub `pugl-ui`
//!
//! The crate [`pugl-ui`](https://crates.io/crates/pugl-ui) provides a
//! stub of a GUI toolkit. It features simple box layouting and event
//! propagation as well as a trait to implement widgets.
//!
//! # Example
//!
//! ``` ignore
//! use pugl_sys::*;
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
//! impl UI {
//!     fn new(view: PuglViewFFI) -> Self {
//!         Self {
//!             view,
//!             red: 1.0,
//!             green: 0.0,
//!             width: 800.0,
//!             height: 600.0,
//!             close_requested: false
//!         }
//!     }
//! }
//!
//! fn example() {
//!     // Request a PuglView passing a closure that returns an initialized `UI`.
//!     let mut view = PuglView::<UI>::new(std::ptr::null_mut(), |pv| UI::new(pv));
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
//!
//! example();
//! ```
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate cairo;
extern crate cairo_sys;

#[macro_use]
extern crate bitflags;

#[cfg(test)]
#[macro_use]
extern crate serial_test;

#[doc(hidden)]
pub(crate) mod pugl;

#[doc(hidden)]
#[macro_use]
pub mod types;

#[doc(inline)]
pub use types::*;

#[doc(hidden)]
#[cfg(not(feature="testing"))]
pub mod view;

#[doc(inline)]
#[cfg(not(feature="testing"))]
pub use view::*;


#[doc(hidden)]
#[cfg(feature="testing")]
pub mod view_test;

#[doc(inline)]
#[cfg(feature="testing")]
pub use view_test::*;
