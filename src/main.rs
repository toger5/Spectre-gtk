#![allow(warnings)]
#![warn(dead_code)]

#[macro_use]
extern crate num_derive;

use std::cell::{RefCell, RefMut};

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::mem;
use std::rc::Rc;

use gio::ApplicationCommandLineExt;

extern crate gio;
extern crate gtk;
extern crate libc;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{
    Application, ApplicationWindow, Builder, Button, ButtonsType, DialogFlags, Entry, Label,
    MessageDialog, MessageType, Window, TreeModelFilterExt,
};
use pango;
use std::time::SystemTime;

fn main() {
    let windows: Rc<RefCell<HashMap<String, Window>>> = Rc::new(RefCell::new(HashMap::new()));
    let application = Application::new(Some("timo.gtk.mpw"), Default::default())
        .expect("failed to initialize GTK application");
    {
        let windows_clone = windows.clone();
        application.connect_activate(move |app| {
            let w = windows_clone.borrow();
            if let Some(pwd_win) = w.get(&"pwd_window".to_owned()) {
                println!("pwd_window exists. So we just show it");
                pwd_win.show_all();
			}
			else if let Some(login_win) = w.get(&"login_window".to_owned()) {
                println!("login_window exists. So we just show it");
                login_win.show_all();
			}
			else if w.is_empty() {
                println!("window does not exist so one gets created");
                drop(w);
                build_ui(app, windows_clone.clone());
            }
        });
    }
    application.run(&[]);
}

fn build_ui(application: &gtk::Application, mut windows: Rc<RefCell<HashMap<String, Window>>>) {
    // const version: mpw::AlgorithmVersion = AlgorithmVersionDefault;
    const password_type: mpw::ResultType = mpw::ResultType::TemplateLong;
    let glade_src = include_str!("gtk_ui_files/testwindow.ui");
    let builder = Builder::new_from_string(glade_src);

    let user: Rc<RefCell<Option<mpw::User>>> = Rc::new(RefCell::new(None));
    let master_key: Rc<RefCell<Option<mpw::MasterKey>>> = Rc::new(RefCell::new(None));
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
    let mpw_entry: Entry = builder
        .get_object("masterpassword")
        .expect("Couldn't get masterpassword Entry");
    let identicon_label: Label = builder
        .get_object("identicon")
        .expect("Couldn't get identicon Label");

    // PASSWORD UI
    let pwd_entry_big: Entry = builder
        .get_object("mpw_entry_big")
        .expect("Couldn't get mpw_entry_big Entry");
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
    gtk::WidgetExt::override_font(&pwd_entry_big, Some(&big_font));

    // LOGIN UI connections
    {
        //let mpw_ent if i need the entry afterwards... now it gets moved. so it cannot get accessed afterwards
        let name = name_entry.clone();
        let pwd = mpw_entry.clone();
        mpw_entry.connect_changed(move |mpw_entry| {
            identicon_label.set_markup(
                &mpw::identicon(
                    name.get_text().expect("no string in name_entry").as_str(),
                    pwd.get_text().expect("no string in mpw_entry").as_str(),
                )
                .to_string(),
            );
        });
    }
    {
        let log_win = login_window.clone();
        let pwd_win = pwd_window.clone();
        let name = name_entry.clone();
        let pwd = mpw_entry.clone();
        let mut m_k = master_key.clone();
        let mut usr = user.clone();
        let s_store = site_list_store.clone();
        let windows_clone = windows.clone();
        mpw_entry.connect_activate(move |_| {
			// change to load file instead of create user
            *usr.borrow_mut() = Some(mpw::User::create(
                name.get_text().expect("No name string").as_str(),
                pwd.get_text().expect("No password string").as_str(),
                mpw::AlgorithmVersionDefault,
			));
			usr.borrow_mut().as_mut().unwrap().load_sites_from_file();

            *m_k.borrow_mut() = Some(mpw::master_key(
                name.get_text().expect("No name string").as_str(),
                pwd.get_text().expect("No password string").as_str(),
                mpw::AlgorithmVersionDefault,
            ));
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
                &usr.borrow().expect("No user while filling site list"),
            );
            println!("{:?}", usr.borrow());
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
			let search_name = pwd_entry_big_clone.get_text().expect("No search name").to_string();
			if pwd_entry_big_clone.get_text_length() < 1 || search_name.is_empty() {
				return true;
			}
			let site_name = model.get_value(iter, 0).get::<String>().unwrap().to_lowercase();
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
            pwd_win.hide();
            let m_k = m_k.borrow().expect("NO MASTER KEY GOT DAMMIT");
            let site_name = entry.get_text().expect("No site name");
            let pwd = mpw::site_result(
                site_name.as_str(),
                m_k,
                password_type,
                mpw::AlgorithmVersionDefault,
            );
            let mut exists = false;
            for s in usr.borrow().unwrap().get_sites() {
                unsafe {
                    if (*s).get_name() == site_name.as_str().to_owned() {
                        exists = true;
                    }
                }
            }

            if !exists {
                print!("The site does not exist!!!");
				//TODO: show popup
				usr.borrow_mut().as_mut().unwrap().add_site(
					site_name.as_str(),
					mpw::ResultType::TemplateLong,
					1,
					mpw::AlgorithmVersionDefault,
				);

				match mpw::marshal_write_to_file(mpw::MarshalFormat::flat, usr.borrow().unwrap()) {
					Ok(a) => println!("succsesfully wrote to file"),
					Err(r) => println!("err {}", r),
				}

				// reload site list:
				fill_site_list(&s_store, &usr.borrow().expect("no User while filling site list"))
            }
            gtk::Clipboard::get_default(
				&entry
				.get_display()
				.expect("cannot get display on pwd_enty_big"),
            )
            .expect("There is no default Clipboard")
            .set_text(&pwd);
			println!("pwd for site {} ({:}) saved to clipboard",site_name.as_str(), mpw::c_char_to_string(usr.borrow().unwrap().fullName));
        });
    }
}

fn fill_site_list(store: &gtk::ListStore, user: &mpw::User) {
	// print!("sites:\n");
	let entriesfile = user.get_sites();
	store.clear();
    for site in entriesfile {//user.get_sites() {
		
		unsafe {
			let site_name : String = (*site).get_name(); //(*site).get_name();c
			// print!("{}",site_name);
            store.insert_with_values(None, &[0], &[&site_name]);
        }
    }
}

mod mpw;

#[cfg(test)]
mod testsWithPrint;
