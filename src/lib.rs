

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
