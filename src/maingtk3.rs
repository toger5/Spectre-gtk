#![allow(warnings)]
#![warn(dead_code)]

#[macro_use]
extern crate num_derive;

use std::cell::{RefCell, RefMut};

use gio::ApplicationCommandLineExt;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::mem;
use std::rc::Rc;
extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate libc;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{
    Application, ApplicationWindow, Builder, Button, ButtonsType, DialogFlags, Entry, Label,
    MessageDialog, MessageType, TreeModelFilterExt, Window,
};
use pango;
use std::time::SystemTime;

fn main() {
    let windows: Rc<RefCell<HashMap<String, Window>>> = Rc::new(RefCell::new(HashMap::new()));
    let application = Application::new(Some("timo.gtk.spectre"), Default::default())
        .expect("failed to initialize GTK application");
    {
        let windows_clone = windows.clone();
        application.connect_activate(move |app| {
            let w = windows_clone.borrow();
            if let Some(pwd_win) = w.get(&"pwd_window".to_owned()) {
                println!("pwd_window exists. So we just show it");
                pwd_win.show_all();
            } else if let Some(login_win) = w.get(&"login_window".to_owned()) {
                println!("login_window exists. So we just show it");
                login_win.show_all();
            } else if w.is_empty() {
                println!("window does not exist so one gets created");
                drop(w);
                build_ui(app, windows_clone.clone());
            }
        });
    }
    application.run(&[]);
}

fn build_ui(application: &gtk::Application, mut windows: Rc<RefCell<HashMap<String, Window>>>) {
    // const version: spectre::AlgorithmVersion = AlgorithmVersionDefault;
    const password_type: spectre::ResultType = spectre::ResultType::TemplateLong;
    let glade_src = include_str!("gtk_ui_files/testwindow.ui");
    let builder = Builder::from_string(glade_src);

    let user: Rc<RefCell<Option<spectre::User>>> = Rc::new(RefCell::new(None));
    let master_key: Rc<RefCell<Option<spectre::UserKey>>> = Rc::new(RefCell::new(None));
    //login Window
    let login_window: gtk::Window = builder
        .get_object("login_window")
        .expect("Couldn't get login_window");
    login_window.set_resizable(false);
    application.add_window(&login_window);
    windows
        .borrow_mut()
        .insert("login_window".to_owned(), login_window.clone());

    //pwd Window
    let pwd_window: gtk::Window = builder.get_object("password_window").unwrap();
    application.add_window(&pwd_window);

    // LOGIN UI
    let name_entry: Entry = builder
        .get_object("username")
        .expect("Couldn't get username Entry");
    let spectre_entry: Entry = builder
        .get_object("masterpassword")
        .expect("Couldn't get masterpassword Entry");
    let identicon_label: Label = builder
        .get_object("identicon")
        .expect("Couldn't get identicon Label");

    // PASSWORD UI
    let pwd_entry_big: Entry = builder
        .get_object("spectre_entry_big")
        .expect("Couldn't get spectre_entry_big Entry");
    let site_list: gtk::TreeView = builder
        .get_object("site_list")
        .expect("Couldn't get site_list TreeView");
    site_list.set_headers_visible(false);
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();
    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", 0);
    site_list.append_column(&column);
    let site_list_store = gtk::ListStore::new(&[String::static_type()]);

    let mut big_font = pango::FontDescription::new();
    big_font.set_size(60000);
    gtk::WidgetExt::override_font(&pwd_entry_big, &big_font);

    // LOGIN UI connections
    {
        //let spectre_ent if i need the entry afterwards... now it gets moved. so it cannot get accessed afterwards
        let name = name_entry.clone();
        let pwd = spectre_entry.clone();
        spectre_entry.connect_changed(move |spectre_entry| {
            identicon_label.set_markup(
                &spectre::identicon(name.text().as_str(), pwd.text().as_str()).to_string(),
            );
        });
    }
    {
        let log_win = login_window.clone();
        let pwd_win = pwd_window.clone();
        let name = name_entry.clone();
        let pwd = spectre_entry.clone();
        let mut m_k = master_key.clone();
        let mut usr = user.clone();
        let s_store = site_list_store.clone();
        let windows_clone = windows.clone();
        spectre_entry.connect_activate(move |_| {

            let mut path = dirs::config_dir().unwrap();
            path.push(format!("{}", name.text().as_str()));
            path.set_extension("mpsites");

            // check if user already exists:
            if path.exists() {
                match spectre::User::authenticate(&path, pwd.text().as_str().to_string()){
                    Ok(user) => *usr.borrow_mut() = Some(user),
                    Err(err) => {
                        match err {
                            spectre::FileMarshalReadError::File(io_err) => panic!(io_err),
                            spectre::FileMarshalReadError::Marshal(marshal_err) => {
                                match marshal_err.type_{
                                    spectre::SpectreMarshalSuccess => MessageDialog::new(Some(&log_win),DialogFlags::empty(),MessageType::Error, ButtonsType::Ok, "success").show_all(),
                                    spectre::SpectreMarshalErrorStructure => MessageDialog::new(Some(&log_win),DialogFlags::empty(),MessageType::Error, ButtonsType::Ok, "An error in the structure of the marshall file interrupted marshalling.").show_all(),
                                    spectre::SpectreMarshalErrorFormat => MessageDialog::new(Some(&log_win),DialogFlags::empty(),MessageType::Error, ButtonsType::Ok, "The marshall file uses an unsupported format version.").show_all(),
                                    spectre::SpectreMarshalErrorMissing => MessageDialog::new(Some(&log_win),DialogFlags::empty(),MessageType::Error, ButtonsType::Ok, "A required value is missing or not specified.").show_all(),
                                    spectre::SpectreMarshalErrorUserSecret => MessageDialog::new(Some(&log_win),DialogFlags::empty(),MessageType::Error, ButtonsType::Ok, "The given user secret is not valid.").show_all(),
                                    spectre::SpectreMarshalErrorIllegal => MessageDialog::new(Some(&log_win),DialogFlags::empty(),MessageType::Error, ButtonsType::Ok, "An illegal value was specified.").show_all(),
                                    spectre::SpectreMarshalErrorInternal => MessageDialog::new(Some(&log_win),DialogFlags::empty(),MessageType::Error, ButtonsType::Ok, "An internal system error interrupted marshalling.").show_all(),
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
                *usr.borrow_mut() = Some(spectre::User::create(
                    name.text().as_str(),
                    pwd.text().as_str(),
                    spectre::AlgorithmVersionDefault,
                ));
            }
            *m_k.borrow_mut() = Some(spectre::user_key(
                name.text().as_str(),
                pwd.text().as_str(),
                spectre::AlgorithmVersionDefault,
            ));
            if let Some(user) = *usr.borrow() {
                log_win.hide();
                pwd_win.hide_on_delete();
                pwd_win.show_all();
                // pwd_win.fullscreen();
                // pwd_win.set_default_size(500,500);
                pwd_win.set_resizable(false);
                windows_clone
                    .borrow_mut()
                    .insert("pwd_window".to_owned(), pwd_win.clone());
                fill_site_list(
                    &s_store,
                    &user,
                );
                println!("{:?}", user.userName);
            }
        });
    }
    login_window.show_all();

    // PASSWORD UI connections
    {
        let pwd_entry_big_clone = pwd_entry_big.clone();
        pwd_window.connect_show(move |window| {
            pwd_entry_big_clone.set_text("");
        });
        let filter = gtk::TreeModelFilter::new(&site_list_store, None);

        let pwd_entry_big_clone = pwd_entry_big.clone();
        filter.set_visible_func(move |model, iter| {
            let search_name = pwd_entry_big_clone.text().to_string();
            if pwd_entry_big_clone.text_length() < 1 || search_name.is_empty() {
                return true;
            }
            let site_name = model
                .get_value(iter, 0)
                .get::<String>()
                .unwrap()
                .expect("a")
                .to_lowercase();
            site_name.contains(&search_name)
        });
        site_list.set_model(Some(&filter));
        pwd_entry_big.connect_changed(move |entry| {
            filter.refilter();
        });

        let pwd_win = pwd_window.clone();
        let m_k = master_key.clone();
        let usr = user.clone();
        let s_store = site_list_store.clone();
        pwd_entry_big.connect_activate(move |entry| {
            // log_win.hide();
            let m_k = m_k.borrow().expect("NO MASTER KEY GOT DAMMIT");
            let site_name = entry.text();
            let pwd = spectre::site_result(
                site_name.as_str(),
                m_k,
                password_type,
                spectre::AlgorithmVersionDefault,
            );
            let atom = gdk::Atom::intern("CLIPBOARD");
            let clipboard = gtk::Clipboard::get(&atom);

            // pwd_win.get_clipboard(gtk::Atom::intern("GDK_SELECTION_CLIPBOARD"))
            // .expect("There is no default Clipboard")
            clipboard.set_text(&pwd);
            // gtk::Clipboard::get_default(
            // 	&pwd_win
            // 	.get_display(),
            // )
            println!(
                "pwd for site {} ({:}) saved to clipboard",
                site_name.as_str(),
                spectre::c_char_to_string(usr.borrow().unwrap().userName)
            );
            pwd_win.hide();
            let mut exists = false;
            for s in usr.borrow().unwrap().get_sites() {
                unsafe {
                    if (*s).get_name() == site_name.as_str().to_owned() {
                        exists = true;
                    }
                }
            }

            if !exists {
                println!("The site does not exist!!! -> gets created");
                //TODO: show popup
                usr.borrow_mut().as_mut().unwrap().add_site(
                    site_name.as_str(),
                    spectre::ResultType::TemplateLong,
                    1,
                    spectre::AlgorithmVersionDefault,
                );

                match spectre::marshal_write_to_file(
                    spectre::MarshalFormat::flat,
                    usr.borrow().unwrap(),
                ) {
                    Ok(a) => println!("succsesfully wrote to file"),
                    Err(r) => println!("err {}", r),
                }

                // reload site list:
                fill_site_list(
                    &s_store,
                    &usr.borrow().expect("no User while filling site list"),
                )
            }
        });
    }
}

fn fill_site_list(store: &gtk::ListStore, user: &spectre::User) {
    // print!("sites:\n");
    let entriesfile = user.get_sites();
    store.clear();
    for site in entriesfile {
        //user.get_sites() {

        unsafe {
            let site_name: String = (*site).get_name(); //(*site).get_name();c
                                                        // print!("{}",site_name);
            store.insert_with_values(None, &[0], &[&site_name]);
        }
    }
}

mod spectre;

#[cfg(test)]
mod testsWithPrint;
