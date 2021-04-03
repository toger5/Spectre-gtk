# Masterpassword GTK
A Gtk application using Rust. The masterpassowrd algorithm is added via the Rust FFI. Using the new spectre api repository (https://gitlab.com/spectre.app/api)
(Spectre is the rebranded masterpassword algorithm)

## Current state
 - The FFI bindings are implementd and the api git submodule gets compiled by the build.rs script. The bidings just need to be cleand up quiet a lot and wrapped to nicer Rust objects.
 - The GTK ui is just a placeholder for now and basically nonexistent. A design needs to be made and implemented. (I would like to have a settings window, with login options and account settings like used passwords. But in actual usecase it should just be a fullscreen or popup window whcih only shows a textfild to search for the desired pwd)
 - The flatpak can be build using the rust extension and the `org.gnome.Platform` SDK.

## Compile
Compile with cargo `cargo build`
For the flatpak build and installation use:
`./build-aux/flatpak-install.sh`
To only build the flatpak:
`./build-aux/flatpak-build.sh`
_To uninstall_
`flatpak remove com.github.spectre`

### Other requirements
for Bindgen there needs to be clang installed (and maybe llvm devel packages) see: https://rust-lang.github.io/rust-bindgen/requirements.html
When using Flatpak this should all be available

## Screenshots
![](https://github.com/toger5/Spectre-gtk/blob/master/data/Screenshot_login.png)
## Design plans
![](https://raw.githubusercontent.com/toger5/Spectre-gtk/master/data/Design.svg)
