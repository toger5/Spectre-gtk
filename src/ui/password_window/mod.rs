use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use crate::spectre;
use crate::ui::{password_list_box::PasswordListBox, password_search_box::PasswordSearchBox};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
mod imp;

glib::wrapper! {
    pub struct PasswordWindow(ObjectSubclass<imp::PasswordWindow>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow;
}
impl PasswordWindow {
    pub fn new(
        user: Rc<RefCell<Option<spectre::User>>>,
        user_key: Rc<RefCell<Option<spectre::UserKey>>>,
    ) -> Self {
        let self_: PasswordWindow =
            glib::Object::new(&[]).expect("Failed to create PasswordWindow");
        let self_priv_ = &imp::PasswordWindow::from_instance(&self_);
        *self_priv_.user.borrow_mut() = *user.borrow();
        *self_priv_.user_key.borrow_mut() = *user_key.borrow();
        self_.setup();
        self_
    }

    pub fn setup(&self) {
        let self_ = &imp::PasswordWindow::from_instance(self);

        let model = gtk::NoSelection::new(Some(&self_.string_store));

        let factory = gtk::SignalListItemFactory::new();
        self_.list_view.set_factory(Some(&factory));
        self_.list_view.set_model(Some(&model));

        let (user, user_key) = (self_.user.clone(), self_.user_key.clone());
        factory.connect_setup(
            glib::clone! { @weak user, @weak user_key=>move |fact, item| {
                let stack = gtk::StackBuilder::new().vhomogeneous(false).build();
                let pwd_box = PasswordListBox::new();
                pwd_box.setup_user(user.clone(), user_key.clone());
                stack.add_named(&pwd_box, Some("pwd"));

                let search_box = PasswordSearchBox::new();
                search_box.setup_user(user.clone(), user_key.clone());
                stack.add_named(&search_box, Some("search"));
                item.set_child(Some(&stack));
            }},
        );
        factory.connect_bind(|fact, item| {
            let (prop, stack) = (
                item.item()
                    .unwrap()
                    .property("string")
                    .ok()
                    .unwrap()
                    .get::<String>()
                    .ok()
                    .unwrap()
                    .unwrap(),
                item.child().unwrap().downcast::<gtk::Stack>().ok().unwrap(),
            );
            println!("{}", prop);
            let visible_child = if prop == "___search" {
                "search"
            } else {
                stack
                    .child_by_name("pwd")
                    .unwrap()
                    .downcast::<PasswordListBox>()
                    .ok()
                    .unwrap()
                    .set_site_name(&prop);
                "pwd"
            };
            stack.set_visible_child_name(visible_child);
        });
        self_.string_store.append("___search");
    }

    pub fn parse_string_list_item(item: gtk::ListItem) -> (String, gtk::Stack) {
        (
            item.item()
                .unwrap()
                .property("string")
                .ok()
                .unwrap()
                .get::<String>()
                .ok()
                .unwrap()
                .unwrap(),
            item.child().unwrap().downcast::<gtk::Stack>().ok().unwrap(),
        )
    }

    pub fn fill_site_list(&self) {
        let self_ = &imp::PasswordWindow::from_instance(self);
        let site_list = self_.user.borrow().unwrap().get_sites();
        // self_.string_store.clear();
        for site in site_list {
            unsafe {
                let site_name: String = (*site).get_name();
                self_.string_store.append(&site_name);
            }
        }
    }
}




// PASSWORD UI
/*
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
*/