# rust-vst2
[![Travis Build][trav-img]][trav-url]
[![Appveyor Build][appv-img]][appv-url]
[![crates.io][crates-img]][crates-url]

This crate is no longer maintained.  Please refer to [the `vst` crate repo](https://github.com/rust-dsp/rust-vst) for an actively developed repository.
----

A library to help facilitate creating VST plugins in rust.

This library is a work in progress and as such does not yet implement all
opcodes. It is enough to create basic VST plugins without an editor interface.

*Please note: This api may be subject to rapid changes and the current state of
this library is not final.*

## Library Documentation
  * http://overdrivenpotato.github.io/rust-vst2

## TODO
  - Implement all opcodes
  - Proper editor support (possibly [conrod] + [sdl2]?)
  - Write more tests
  - Provide better examples

## Usage
To create a plugin, simply create a type which implements `plugin::Plugin` and
`std::default::Default`. Then call the macro `plugin_main!`, which will export
the necessary functions and handle dealing with the rest of the API.

## Example Plugin
A simple plugin that bears no functionality. The provided Cargo.toml has a
crate-type directive which builds a dynamic library, usable by any VST host.

`src/lib.rs`

```rust
#[macro_use]
extern crate vst2;

use vst2::plugin::{Info, Plugin};

#[derive(Default)]
struct BasicPlugin;

impl Plugin for BasicPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Basic Plugin".to_string(),
            unique_id: 1357, // Used by hosts to differentiate between plugins.

            ..Default::default()
        }
    }
}

plugin_main!(BasicPlugin); // Important!
```

`Cargo.toml`

```toml
[package]
name = "basic_vst"
version = "0.0.1"
authors = ["Author <author@example.com>"]

[dependencies]
vst2 = "0.0.1"

[lib]
name = "basicvst"
crate-type = ["dylib"]
```

[trav-img]: https://travis-ci.org/overdrivenpotato/rust-vst2.svg?branch=master
[trav-url]: https://travis-ci.org/overdrivenpotato/rust-vst2
[appv-img]: https://ci.appveyor.com/api/projects/status/4kg8efxas08b72bp?svg=true
[appv-url]: https://ci.appveyor.com/project/overdrivenpotato/rust-vst2
[crates-img]: https://img.shields.io/crates/v/vst2.svg
[crates-url]: https://crates.io/crates/vst2
[sdl2]: https://github.com/AngryLawyer/rust-sdl2
[conrod]: https://github.com/PistonDevelopers/conrod


#### Packaging on OS X

On OS X VST plugins are packaged inside of loadable bundles. 
To package your VST as a loadable bundle you may use the `osx_vst_bundler.sh` script this library provides. 

Example: 

```
./osx_vst_bundler.sh Plugin target/release/plugin.dylib
Creates a Plugin.vst bundle
```
