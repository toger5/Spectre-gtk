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
        self_.string_store.append("___search");

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
            let prop = item
                .item()
                .unwrap()
                .property("string")
                .ok()
                .unwrap()
                .get::<String>()
                .ok()
                .unwrap()
                .unwrap();
            let stack = item.child().unwrap().downcast::<gtk::Stack>().ok().unwrap();

            let visible_child = if prop == "___search" {
                "search"
            } else {
                let pwd_box = stack
                    .child_by_name("pwd")
                    .expect("no child found in password list item")
                    .downcast::<PasswordListBox>()
                    .ok()
                    .unwrap();

                pwd_box.set_site_name(&prop);
                "pwd"
            };
            stack.set_visible_child_name(visible_child);
        });
    }

    pub fn fill_site_list(&self) {
        let self_ = &imp::PasswordWindow::from_instance(self);
        let site_list = self_.user.borrow().unwrap().get_sites();
        // store.clear();
        for site in site_list {
            unsafe {
                let site_name: String = (*site).get_name();
                self_.string_store.append(&site_name);
            }
        }
    }
}
