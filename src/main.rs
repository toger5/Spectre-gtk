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
use gtk::prelude::*;
use std::fs::File;
// use glib;
use gtk::{
    Application, ApplicationWindow, Builder, Button, ButtonsType, DialogFlags, Entry, Label,
    MessageDialog, MessageType, TreeModelFilterExt, Window, glib, ListItem,
};
use pango;
use std::time::SystemTime;

mod ui;

use ui::password_list_item::PasswordListBox;
use ui::spectre_app::SpectreApp;

mod paths;


fn main() {
    // Current App
    // let windows: Rc<RefCell<HashMap<String, Window>>> = Rc::new(RefCell::new(HashMap::new()));
    // let application = Application::new(Some("timo.gtk.spectre"), Default::default())
    //     .expect("failed to initialize GTK application");
    // {
    //     let windows_clone = windows.clone();
    //     application.connect_activate(move |app| {
    //         let w = windows_clone.borrow();
    //         if let Some(pwd_win) = w.get(&"pwd_window".to_owned()) {
    //             println!("pwd_window exists. So we just show it");
    //             pwd_win.show();
    //         } else if let Some(login_win) = w.get(&"login_window".to_owned()) {
    //             println!("login_window exists. So we just show it");
    //             login_win.show();
    //         } else if w.is_empty() {
    //             println!("window does not exist so one gets created");
    //             drop(w);
    //             build_ui(app, windows_clone.clone());
    //         }
    //     });
    // }
    // application.run(&[]);

    // Test with coustom application
    // let app = SpectreApp::with_username("jung junge jugne\n jetzt wird gegessen");
    // let argv = std::env::args().collect::<Vec<_>>();
    // std::process::exit(app.run(&argv));

    // Test With coustom widget
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.widget_subclass"),
        Default::default(),
    );

    application.connect_activate(|app| {
        // Load custom styling
        let mut path = paths::Paths::new().prefix;
        path.push("data/style.css");
        let provider = gtk::CssProvider::new();
        provider.load_from_file(&gdk::gio::File::new_for_path(&path));
        
        gtk::StyleContext::add_provider_for_display(&gdk::Display::default().unwrap(), &provider, 500);

        let window = gtk::ApplicationWindow::new(app);
        // let store = gtk::gio::ListStore::new( glib::GString::static_type());
        let string_store = gtk::StringList::new(&["hallo", "das sind Meine","hallo", "das sind Meine","hallo", "das sind Meine","hallo", "das sind Meine","hallo", "das sind Meine","hallo", "das sind Meine","hallo", "das sind Meine","hallo", "das sind Meine","hallo", "das sind Meine","hallo", "das sind Meine","hallo", "das sind Meine","hallo", "das sind Meine", "www.google.de"]);
        let model = gtk::NoSelection::new(Some(&string_store));

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(|fact, item|{
            let pwd_box = PasswordListBox::new();
            pwd_box.set_site_name("helloWorld.com");
            item.set_child(Some(&pwd_box));
        });
        factory.connect_bind(|fact, item|{
            println!("{}",item);
            let pwd_box = PasswordListBox::new();
            pwd_box.set_site_name("helloWorld.com");
            item.set_child(Some(&pwd_box));
            item.child();
            // widget.downcast::<gtk::Button>().is_ok();
            // let pwd_box = item.child().expect("no child found in password list item").downcast::<PasswordListBox>().ok();
            // pwd_box.set_site_name(item.item().downcast::<glib::GString>().ok().as_str())
        });

        let list = gtk::ListView::new(Some(&model),Some(&factory));
        let sw = gtk::ScrolledWindow::new();
        sw.set_child(Some(&list));
        sw.set_min_content_height(300);
        sw.set_min_content_width(500);
        sw.set_propagate_natural_width(true);
        window.set_child(Some(&sw));
        window.show();
    });

    application.run();
}

fn build_ui(application: &gtk::Application, mut windows: Rc<RefCell<HashMap<String, Window>>>) {
    // const version: spectre::AlgorithmVersion = AlgorithmVersionDefault;
    const password_type: spectre::ResultType = spectre::ResultType::TemplateLong;
    let glade_src = include_str!("gtk_ui_files/windows.ui");
    let builder = Builder::from_string(glade_src);

    let user: Rc<RefCell<Option<spectre::User>>> = Rc::new(RefCell::new(None));
    let master_key: Rc<RefCell<Option<spectre::SpectreUserKey>>> = Rc::new(RefCell::new(None));
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
    let pwd_window: gtk::Window = builder.object("password_window").unwrap();
    application.add_window(&pwd_window);

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

    // PASSWORD UI
    let pwd_entry_big: Entry = builder
        .object("spectre_entry_big")
        .expect("Couldn't get spectre_entry_big Entry");
    let site_list: gtk::ListView = builder
        .object("site_list")
        .expect("Couldn't get site_list TreeView");
    // site_list.set_headers_visible(false);
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();
    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", 0);
    // site_list.append_column(&column);
    let site_list_store = gtk::StringList::new(&[]);

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
                // pwd_win.hide_on_delete();
                pwd_win.show();
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
    login_window.show();

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
            entry.clipboard().set_text(&pwd);

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

fn fill_site_list(store: &gtk::StringList, user: &spectre::User) {
    // print!("sites:\n");
    let entriesfile = user.get_sites();
    // store.clear();
    for site in entriesfile {
        //user.get_sites() {

        unsafe {
            let site_name: String = (*site).get_name(); //(*site).get_name();c
                                                        // print!("{}",site_name);
            store.append(&site_name);
        }
    }
}

mod spectre;

#[cfg(test)]
mod testsWithPrint;
