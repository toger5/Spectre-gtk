# Masterpassword GTK
A gtk application using Rust. The masterpassowrd algorithm is added via the Rust FFI.

## Current state
 - The FFI bindings are implementd. The bidings just need to be cleand up quiet a lot and wrapped to nicer Rust objects.
 - The GTK ui is just a placeholder for now and basically nonexistent. A design needs to be made and implemented. (I would like to have a settings window, with login options and account settings like used passwords. But in actual usecase it should just be a fullscreen or popup window whcih only shows a textfild to search for the desired pwd)

## Compile
Compile with cargo (`cargo build` inside gui_app folder)
To execute the file you need to add `/src/masterpassword-c/core/lib/linux/x86_64/` to your `LD_LIBRARY_PATH`.\
To do so run:\
```
export LD_LIBRARY_PATH=$(pwd)/gui_app/src/masterpassword-c/core/lib/linux/x86_64/
```
inside this (rust-gtk-mpw) folder before executing.
### Other requirements
for Bindgen there needs to be clang installed (and maybe llvm devel packages) see: https://rust-lang.github.io/rust-bindgen/requirements.html
```
export LD_LIBRARY_PATH=$HOME/Projects/rust-gtk-mpw/gui_app/src/masterpassword-c/core/lib/linux/x86_64/;./$HOME/Projects/rust-gtk-mpw/gui_app/target/debug/rust-mpw
```