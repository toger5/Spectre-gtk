#![allow(warnings)]
#![warn(dead_code)]

#[macro_use]
extern crate num_derive;

use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::mem;
use std::rc::Rc;
extern crate gdk;
extern crate gtk;
extern crate libc;
use gdk::gio;
use gio::prelude::*;
use glib::prelude::*;
use glib::{object::Object, FromVariant, GString, ToVariant, Variant};
use gtk::prelude::*;
use std::fs::File;
// use glib;
use gtk::{
    glib, Application, ApplicationWindow, Builder, Button, ButtonsType, DialogFlags, Entry, Label,
    ListItem, MessageDialog, MessageType, TreeModelFilterExt, Window,
};
use pango;
use std::time::SystemTime;

mod ui;

use ui::password_list_box::PasswordListBox;
use ui::password_search_box::PasswordSearchBox;
use ui::spectre_app::SpectreApp;

mod paths;

fn main() {
    // Current App
    let windows: Rc<RefCell<HashMap<String, Window>>> = Rc::new(RefCell::new(HashMap::new()));
    let application = Application::new(Some("timo.gtk.spectre"), Default::default());
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

    application.run();
}
fn load_custom_styling() {
    let mut path = paths::Paths::new().prefix;
    path.push("data/style.css");
    let provider = gtk::CssProvider::new();
    provider.load_from_file(&gdk::gio::File::new_for_path(&path));
    gtk::StyleContext::add_provider_for_display(&gdk::Display::default().unwrap(), &provider, 500);
}

fn build_ui(application: &gtk::Application, mut windows: Rc<RefCell<HashMap<String, Window>>>) {
    // const version: spectre::AlgorithmVersion = AlgorithmVersionDefault;
    const password_type: spectre::ResultType = spectre::ResultType::TemplateLong;
    let glade_src = include_str!("gtk_ui_files/windows.ui");
    let builder = Builder::from_string(glade_src);

    // let user: Rc<RefCell<Option<spectre::User>>> = Rc::new(RefCell::new(None));
    // let user_key: Rc<RefCell<Option<spectre::UserKey>>> = Rc::new(RefCell::new(None));
    //login Window
    let login_window: Window = builder
        .object("login_window")
        .expect("Couldn't get login_window");
    login_window.set_resizable(false);
    application.add_window(&login_window);
    windows
        .borrow_mut()
        .insert("login_window".to_owned(), login_window.clone());

    //pwd Window
    // let (pwd_window, pwd_list_store) =
    //     build_pwd_window(application, user.clone(), user_key.clone());
    // let pwd_window: gtk::Window = builder.object("password_window").unwrap();
    

    // LOGIN UI
    let name_entry: Entry = builder
        .object("username")
        .expect("Couldn't get username Entry");
    let spectre_entry: Entry = builder
        .object("masterpassword")
        .expect("Couldn't get masterpassword Entry");
    let identicon_label: Label = builder
        .object("identicon")
        .expect("Couldn't get identicon Label");

    // let site_list_store = gtk::StringList::new(&[]);
    // LOGIN UI connections
    {
        let log_win = login_window.clone();
        // let pwd_win = pwd_window.clone();
        let name = name_entry.clone();
        let pwd = spectre_entry.clone();
        
        // let s_store = pwd_list_store.clone();
        let windows_clone = windows.clone();
        spectre_entry.connect_changed(move |_| {
            identicon_label.set_markup(
                &spectre::identicon(name.text().as_str(), pwd.text().as_str()).to_markup_string(),
            );
        });
        // let name = name_entry.clone();
        // let pwd = spectre_entry.clone();
        // let app_clone = application.clone();
        spectre_entry.connect_activate(glib::clone!{@weak application, @weak spectre_entry, @weak name_entry => move |_| {
            // let mut m_k = user_key.clone();
            // let mut usr = user.clone();
            let user: Rc<RefCell<Option<spectre::User>>> = Rc::new(RefCell::new(None));
            // let user_key: Rc<RefCell<Option<spectre::UserKey>>> = Rc::new(RefCell::new(None));
            let mut path = dirs::config_dir().unwrap();
            path.push(format!("{}", name_entry.text().as_str()));
            path.set_extension("mpsites");

            // check if user already exists:
            if path.exists() {
                match spectre::User::authenticate(&path, spectre_entry.text().as_str().to_string()){
                    Ok(user_ok) => *user.borrow_mut() = Some(user_ok),
                    Err(err) => {
                        match err {
                            spectre::FileMarshalReadError::File(io_err) => panic!(io_err),
                            spectre::FileMarshalReadError::Marshal(marshal_err) => {
                                match marshal_err.type_{
                                    spectre::SpectreMarshalSuccess => MessageDialog::new(Some(&log_win),DialogFlags::empty(),MessageType::Error, ButtonsType::Ok, "success").show(),
                                    spectre::SpectreMarshalErrorStructure => MessageDialog::new(Some(&log_win),DialogFlags::empty(),MessageType::Error, ButtonsType::Ok, "An error in the structure of the marshall file interrupted marshalling.").show(),
                                    spectre::SpectreMarshalErrorFormat => MessageDialog::new(Some(&log_win),DialogFlags::empty(),MessageType::Error, ButtonsType::Ok, "The marshall file uses an unsupported format version.").show(),
                                    spectre::SpectreMarshalErrorMissing => MessageDialog::new(Some(&log_win),DialogFlags::empty(),MessageType::Error, ButtonsType::Ok, "A required value is missing or not specified.").show(),
                                    spectre::SpectreMarshalErrorUserSecret => MessageDialog::new(Some(&log_win),DialogFlags::empty(),MessageType::Error, ButtonsType::Ok, "The given user secret is not valid.").show(),
                                    spectre::SpectreMarshalErrorIllegal => MessageDialog::new(Some(&log_win),DialogFlags::empty(),MessageType::Error, ButtonsType::Ok, "An illegal value was specified.").show(),
                                    spectre::SpectreMarshalErrorInternal => MessageDialog::new(Some(&log_win),DialogFlags::empty(),MessageType::Error, ButtonsType::Ok, "An internal system error interrupted marshalling.").show(),
                                    _ => panic!("unknown error type while reading marshaling from file"),
                                }
                                // panic!(spectre::c_char_to_string(marshal_err.message))
                            },
                        }
                    }
                }
            }else{
                // TODO only create new user when yes is chosen
                let dialog = MessageDialog::new(Some(&log_win),DialogFlags::empty(),MessageType::Error, ButtonsType::YesNo, "Are u suuure? \n (About creating a new user.)").show();
                *user.borrow_mut() = Some(spectre::User::create(
                    name_entry.text().as_str(),
                    spectre_entry.text().as_str(),
                    spectre::AlgorithmVersionDefault,
                ));
            }
            
            if let Some(_) = *user.borrow() {
                log_win.hide();
                // pwd_win.hide_on_delete();
                // let user_rc = Rc::new(RefCell::new(user));
                // let user_key_rc = Rc::new(RefCell::new(user_key));
                let user_key = Some(spectre::user_key(
                    name_entry.text().as_str(),
                    spectre_entry.text().as_str(),
                    spectre::AlgorithmVersionDefault,
                ));
                let pwd_window = ui::password_window::PasswordWindow::new(user.clone(), Rc::new(RefCell::new(user_key)));
                application.add_window(&pwd_window);
                pwd_window.show();
                // pwd_win.fullscreen();
                // pwd_win.set_default_size(500,500);
                // pwd_win.set_resizable(false);
                windows_clone
                    .borrow_mut()
                    .insert("pwd_window".to_owned(), pwd_window.clone().upcast::<gtk::Window>());
                pwd_window.fill_site_list();
                println!("{:?}", user.borrow().unwrap().userName);
            };
        }});
    }
    login_window.show();

    // PASSWORD UI

    /*-------------------------------------------------------------------------------------------------------------------

    let pwd_entry_big: Entry = Entry::new();
    pwd_window
        .child()
        .unwrap()
        .downcast::<gtk::Box>()
        .ok()
        .unwrap()
        .append(&pwd_entry_big);
    pwd_entry_big.hide();
    // PASSWORD UI connections
    {
        let pwd_entry_big_clone = pwd_entry_big.clone();
        pwd_window.connect_show(move |window| {
            pwd_entry_big_clone.set_text("");
        });

        let pwd_entry_big_clone = pwd_entry_big.clone();
        let search_name = pwd_entry_big_clone.text().to_string();
        // let filter = gtk::StringFilter::new(None);
        // filter.set_search(Some(&search_name));
        // filter_model.set_filter(filter);
        // let filter_model = gtk::FilterListModel::new(Some(&site_list_store), Some(&filter));
        // filter.set_visible_func(move |model: &gtk::TreeModel, iter: &gtk::TreeIter| {
        //     let search_name = pwd_entry_big_clone.text().to_string();
        //     if pwd_entry_big_clone.text_length() < 1 || search_name.is_empty() {
        //         return true;
        //     }

        //     let site_name = (*model)
        //         .get(iter, 0)
        //         .get::<String>()
        //         .unwrap()
        //         .expect("Tree value has wrong type (expected String)")
        //         .to_lowercase();
        //     site_name.contains(&search_name)
        // });

        //TODO-OldList
        /*
        site_list.set_model(Some(&filter));
        pwd_entry_big.connect_changed(move |entry| {
            filter.refilter();
        });
        */

        pwd_entry_big.connect_activate(glib::clone!( @weak user, @weak user_key, @weak pwd_window, @weak pwd_list_store => move |entry| {
            // log_win.hide();
            let user_key = user_key.borrow().expect("NO MASTER KEY GOT DAMMIT");
            let site_name = entry.text();
            let pwd = spectre::site_result(
                site_name.as_str(),
                user_key,
                password_type,
                spectre::AlgorithmVersionDefault,
            );
            entry.clipboard().set_text(&pwd);

            println!(
                "pwd for site {} ({:}) saved to clipboard",
                site_name.as_str(),
                spectre::c_char_to_string(user.borrow().unwrap().userName)
            );
            pwd_window.hide();
            let mut exists = false;
            for s in user.borrow().unwrap().get_sites() {
                unsafe {
                    if (*s).get_name() == site_name.as_str().to_owned() {
                        exists = true;
                    }
                }
            }

            if !exists {
                println!("The site does not exist!!! -> gets created");
                //TODO: show popup
                user.borrow_mut().as_mut().unwrap().add_site(
                    site_name.as_str(),
                    spectre::ResultType::TemplateLong,
                    1,
                    spectre::AlgorithmVersionDefault,
                );

                match spectre::marshal_write_to_file(
                    spectre::MarshalFormat::flat,
                    user.borrow().unwrap(),
                ) {
                    Ok(a) => println!("succsesfully wrote to file"),
                    Err(r) => println!("err {}", r),
                }

                // reload site list:
                fill_site_list(
                    &pwd_list_store,
                    &user.borrow().expect("no User while filling site list"),
                )
            }
        }));
    }
    ------------------------------------*/
}

mod spectre;

#[cfg(test)]
mod testsWithPrint;
