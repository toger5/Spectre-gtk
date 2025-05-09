#![allow(warnings)]
#![warn(dead_code)]

#[macro_use]
extern crate num_derive;

use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::mem;
use std::rc::Rc;
extern crate libc;
use gdk::gio;
use gio::prelude::*;
use glib::prelude::*;
use glib::{object::Object, GString, Variant};

use std::fs::File;
use std::path::Path;

use adw::prelude::*;
use adw::Application;

use gtk::gdk;
use gtk::{glib, Builder, Button, ButtonsType, DialogFlags, Entry, Label, ListItem, MessageDialog, MessageType};
use pango;
use std::time::SystemTime;

mod model;
mod ui;

use ui::password_list_box::PasswordListBox;
use ui::password_search_box::PasswordSearchBox;
use ui::spectre_app::SpectreApp;

// mod paths;
mod config;

fn main() {
    // Current App
    let windows: Rc<RefCell<HashMap<String, gtk::Window>>> = Rc::new(RefCell::new(HashMap::new()));
    let application = Application::new(Some("com.github.spectre"), Default::default());
    {
        let windows_clone = windows.clone();
        application.connect_activate(move |app| {
            load_custom_styling();
            let w = windows_clone.borrow();
            if let Some(pwd_win) = w.get(&"pwd_window".to_owned()) {
                println!("pwd_window exists. So we just show it");
                pwd_win.show();
            } else if let Some(login_win) = w.get(&"login_window".to_owned()) {
                println!("login_window exists. So we just show it");
                login_win.show();
            } else if w.is_empty() {
                println!("window does not exist so one gets created");
                drop(w);
                build_ui(app, windows_clone.clone());
            }
        });
    }
    application.run();

    // Test with coustom application
    // let app = SpectreApp::with_username("jung junge jugne\n jetzt wird gegessen");
    // let argv = std::env::args().collect::<Vec<_>>();
    // std::process::exit(app.run(&argv));

    // Test With coustom widget
    // let application = gtk::Application::new(
    //     Some("com.github.gtk-rs.examples.widget_subclass"),
    //     Default::default(),
    // );

    // application.run();
}

fn load_custom_styling() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("../data/style.css"));
    gtk::StyleContext::add_provider_for_display(&gtk::gdk::Display::default().unwrap(), &provider, 500);
}

fn build_ui(application: &adw::Application, mut windows: Rc<RefCell<HashMap<String, gtk::Window>>>) {
    // const version: spectre::AlgorithmVersion = AlgorithmVersionDefault;
    const password_type: spectre::ResultType = spectre::ResultType::TemplateLong;
    let glade_src = include_str!("gtk_ui_files/windows.ui");
    let builder = Builder::from_string(glade_src);

    //login Window
    let login_window: gtk::Window = builder.object("login_window").expect("Couldn't get login_window");
    login_window.set_resizable(false);
    application.add_window(&login_window);
    windows.borrow_mut().insert("login_window".to_owned(), login_window.clone().upcast());

    // LOGIN UI
    let name_entry: Entry = builder.object("username").expect("Couldn't get username Entry");
    let spectre_entry: Entry = builder.object("masterpassword").expect("Couldn't get masterpassword Entry");
    let identicon_label: Label = builder.object("identicon").expect("Couldn't get identicon Label");

    // LOGIN UI connections
    {
        let log_win = login_window.clone();
        let name = name_entry.clone();
        let pwd = spectre_entry.clone();

        // let s_store = pwd_list_store.clone();
        let windows_clone = windows.clone();
        spectre_entry.connect_changed(move |_| {
            identicon_label.set_markup(&spectre::identicon(name.text().as_str(), pwd.text().as_str()).to_markup_string());
        });
        // let name = name_entry.clone();
        // let pwd = spectre_entry.clone();
        // let app_clone = application.clone();
        spectre_entry.connect_activate(glib::clone!(@weak application, @weak spectre_entry, @weak name_entry => move |_| {
            // let mut m_k = user_key.clone();
            // let mut usr = user.clone();
            let user: Rc<RefCell<Option<spectre::User>>> = Rc::new(RefCell::new(None));
            // let user_key: Rc<RefCell<Option<spectre::UserKey>>> = Rc::new(RefCell::new(None));
            let mut path = crate::config::get_save_path();
            // let mut path = dirs::data_dir().unwrap();
            path.push(format!("{}", name_entry.text().as_str()));
            path.set_extension("mpsites");

            // check if user already exists:
            if path.exists() {
                match spectre::User::authenticate(&path, spectre_entry.text().as_str().to_string()){
                    Ok(user_ok) => *user.borrow_mut() = Some(user_ok),
                    Err(err) => handle_file_marshal_read_error(err, &log_win)
                }
                login(user.clone(), &log_win, &name_entry, &spectre_entry, &application, windows.clone());
            }else{
                let dialog = gtk::MessageDialog::new(
                    Some(&log_win),
                    DialogFlags::MODAL,
                    MessageType::Question,
                    ButtonsType::YesNo,
                    "There is no user with that name.\n Do you want to create a new User file?");
                dialog.set_secondary_text(Some(&format!("A new file will be created:\nat: {}", path.to_str().unwrap())));
                dialog.connect_response(
                    glib::clone!(@strong user,@weak log_win,@weak name_entry,@weak spectre_entry,@weak application,@weak windows => move |dialog, response| {
                    println!("{}",response);
                    match response {
                        gtk::ResponseType::Yes => {
                            *user.borrow_mut() = Some(spectre::User::create(
                            name_entry.text().as_str(),
                            spectre_entry.text().as_str(),
                            spectre::ALGORITHM_VERSION_DEFAULT,
                            ));
                            dialog.emit_close();
                            let p = path.clone();
                            match std::fs::create_dir_all(p.parent().unwrap()){
                                Ok(a) => println!("successfully created directories for save path"),
                                Err(r) => println!("err while creating directories {}", r),
                            };
                            match spectre::marshal_write_to_file(
                                spectre::MarshalFormat::flat,
                                user.borrow().unwrap(),
                            ) {
                                Ok(a) => println!("successfully wrote to file"),
                                Err(r) => println!("err {}", r),
                            }
                            login(user.clone(), &log_win, &name_entry, &spectre_entry, &application, windows.clone());
                        },
                        gtk::ResponseType::No => dialog.close(),
                        default => println!("Message Dialog dismissed"),
                    };
                }));
                dialog.show();
            }
        }));
    }
    fn login(
        user: Rc<RefCell<Option<spectre::User>>>,
        log_win: &gtk::Window,
        name_entry: &gtk::Entry,
        spectre_entry: &gtk::Entry,
        application: &adw::Application,
        windows: Rc<RefCell<HashMap<String, gtk::Window>>>,
    ) {
        if let Some(_) = *user.borrow() {
            log_win.hide();
            // pwd_win.hide_on_delete();
            // let user_rc = Rc::new(RefCell::new(user));
            // let user_key_rc = Rc::new(RefCell::new(user_key));
            let user_key = Some(spectre::user_key(
                name_entry.text().as_str(),
                spectre_entry.text().as_str(),
                spectre::ALGORITHM_VERSION_DEFAULT,
            ));
            let pwd_window = ui::password_window::PasswordWindow::new(user.clone(), Rc::new(RefCell::new(user_key)));
            pwd_window.fill_site_list();
            application.add_window(&pwd_window);
            pwd_window.show();
            // pwd_win.fullscreen();
            // pwd_win.set_default_size(500,500);
            // pwd_win.set_resizable(false);
            windows.borrow_mut().insert("pwd_window".to_owned(), pwd_window.clone().upcast());
            println!("{:?}", user.borrow().unwrap().userName);
        };
    }
    login_window.show();
}

fn handle_file_marshal_read_error(err: spectre::FileMarshalReadError, log_win: &gtk::Window) {
    match err {
        spectre::FileMarshalReadError::File(io_err) => panic!(io_err),
        spectre::FileMarshalReadError::Marshal(marshal_err) => {
            match marshal_err.type_ {
                spectre::SpectreMarshalSuccess => {
                    MessageDialog::new(Some(log_win), DialogFlags::empty(), MessageType::Error, ButtonsType::Ok, "success").show()
                }
                spectre::SpectreMarshalErrorStructure => MessageDialog::new(
                    Some(log_win),
                    DialogFlags::empty(),
                    MessageType::Error,
                    ButtonsType::Ok,
                    "An error in the structure of the marshall file interrupted marshalling.",
                )
                .show(),
                spectre::SpectreMarshalErrorFormat => MessageDialog::new(
                    Some(log_win),
                    DialogFlags::empty(),
                    MessageType::Error,
                    ButtonsType::Ok,
                    "The marshall file uses an unsupported format version.",
                )
                .show(),
                spectre::SpectreMarshalErrorMissing => MessageDialog::new(
                    Some(log_win),
                    DialogFlags::empty(),
                    MessageType::Error,
                    ButtonsType::Ok,
                    "A required value is missing or not specified.",
                )
                .show(),
                spectre::SpectreMarshalErrorUserSecret => MessageDialog::new(
                    Some(log_win),
                    DialogFlags::empty(),
                    MessageType::Error,
                    ButtonsType::Ok,
                    "The given user secret is not valid.",
                )
                .show(),
                spectre::SpectreMarshalErrorIllegal => MessageDialog::new(
                    Some(log_win),
                    DialogFlags::empty(),
                    MessageType::Error,
                    ButtonsType::Ok,
                    "An illegal value was specified.",
                )
                .show(),
                spectre::SpectreMarshalErrorInternal => MessageDialog::new(
                    Some(log_win),
                    DialogFlags::empty(),
                    MessageType::Error,
                    ButtonsType::Ok,
                    "An internal system error interrupted marshalling.",
                )
                .show(),
                _ => panic!("unknown error type while reading marshaling from file"),
            }
            // panic!(spectre::c_char_to_string(marshal_err.message))
        }
    }
}
mod spectre;

// #[cfg(test)]
// mod testsWithPrint;
