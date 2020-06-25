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

Early prototype stage. Not all features of pugl are available yet. Currently
only tested on Linux/X11.


## How to use

### Prerequisites

You need to have the following stuff installed

* python3 (to make waf, the build system of the pugl library run)
* a C compiler and the usual libraries to compile X11 apps
* clang as the `pugl` bindings are accessed through `bindgen`
* developer files of cairo


### Build

* Clone this repo and `cd` into it.
* Setup the `pugl` submodule by `git submodule update --init --recursive`
* Run `cargo build`


### Usage

This crate does not have any examples. There is the
[pugl-ui](https://github.com/johannes-mueller/pugl-ui) crate that is a stub of
a GUI toolkit implementing widget layout and event propagation. Check this out
for examples.


## Todo

Still a lot.

* Make available the remaining features from pugl.

* Figure out how to distribute it via https://crates.io with all the build
  dependencies (python, waf)

* For sure a lot mode
