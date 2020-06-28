# pugl-sys – a rust wrapper for pugl

pugl is a minimal portable API for embeddable GUIs https://gitlab.com/lv2/pugl/

This crate aims to provide crate bindings for Rust.


## Goal

Basis for small light weight self contained GUI toolkits, especially to
implement UIs of [LV2 plugins](https://lv2plug.in). GUIs for LV2 plugins need
to be self contained, i.e. they should be statically linked and must not
dynamically link any other GUI toolkit. Otherwise symbols of the same GUI
toolkit in different versions used by different plugins running in the same
host would clash.


## Status

Beta testing stage. Not all features of pugl are available yet (see
below). Currently only tested on Linux/X11. Rust hackers interested in
programming small embeddable GUIs are encouraged to try it out.

### API stability

Before reaching the 1.0.0 release, incompatible API changes can happen. There
is no large base of applications using `pugl-sys` as of yet. So experience with
the API is limited. The 1.0.0 release will not happen before several developers
have used `pugl-sys` for real life applications and given feedback.

If it turns out that there is a better way to design the API it will be done.

After 1.0.0 incompatible API changes will be rare. They will happen if `pugl`
changes the API in an incompatible way which would not be sensible to hide
behind some abstraction layer.


## How to use

It's available at [crates.io](https://crates.io/crates/pugl-sys) so just add it
as dependency in your `Cargo.toml`


### Usage

This crate has only one minimal example in the docs. There is the
[pugl-ui](https://crates.io/crates/pugl-ui) crate that is a stub of a GUI
toolkit implementing widget layout and event propagation. Maybe you want to
check this out.


## Todo

Not all features of pugl are implemented.

* World as separately accessible entity

	So far the World is only accessible via the View and only by unsafe FFI
	functions. The only World function that is safely wrapped is `puglUpdate()`

	To implement that it takes a mechanism to share references to the same World
	across multiple views, that makes sure that the World is destroyed after the
	last View is destroyed but not while at least one View still exists.


* Support for manually sending events

	This is basically wrapping the function `puglSendEvent()`. To implement it it
	takes a conversion from `pugl::Event` to the `PuglEvent` FFI type.


* Support for Clipboard

	Probably easy for plain text. For other MIME data some other dependency would
	be required.


* Support for other backends than Cairo

    As of now, a view simply sets the Cairo backend and converts the handle
    to a `cairo::Context` when an exposure event happens.

* Some minor functions are not wrapped

* `PuglViewHint` is not properly wrapped except for `PUGL_RESIZABLE`

* Test (and make it work) on platforms other than Linux
