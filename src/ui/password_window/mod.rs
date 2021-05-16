use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use super::password_search_box::CopyButtonMode;
use crate::model::g_site::GSite;
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
pub mod helper {
    use gtk::prelude::*;
    pub fn copy_to_clipboard_with_notification<T>(widget: &T, text: &str)
    where
        T: IsA<gtk::Widget>,
    {
        widget.clipboard().set_text(text);
        let app = widget.root().unwrap().downcast::<gtk::Window>().ok().unwrap().application().unwrap();
        let noti = gtk::gio::Notification::new("Password copied!");
        noti.set_body(Some("It can be pasted anywhere using ctrl+v."));
        app.send_notification(Some("copy-notification"), &noti);
    }
}
impl PasswordWindow {
    pub fn new(user: Rc<RefCell<Option<spectre::User>>>, user_key: Rc<RefCell<Option<spectre::UserKey>>>) -> Self {
        let self_: PasswordWindow = glib::Object::new(&[]).expect("Failed to create PasswordWindow");
        let self_priv_ = &imp::PasswordWindow::from_instance(&self_);
        *self_priv_.user.borrow_mut() = *user.borrow();
        *self_priv_.user_key.borrow_mut() = *user_key.borrow();
        self_.setup();
        self_
    }

    pub fn setup(&self) {
        let self_ = &imp::PasswordWindow::from_instance(self);

        let model = gtk::NoSelection::new(Some(&self_.filter_store));

        let factory = gtk::SignalListItemFactory::new();
        self_.list_view.set_factory(Some(&factory));
        self_.list_view.set_model(Some(&model));

        let (user, user_key) = (self_.user.clone(), self_.user_key.clone());
        factory.connect_setup(glib::clone! {@weak self as self_clone @weak user, @weak user_key=>move |fact, item| {
            let stack = gtk::StackBuilder::new().vhomogeneous(false).build();
            let pwd_box = PasswordListBox::new();
            pwd_box.setup_user(user.clone(), user_key.clone());
            stack.add_named(&pwd_box, Some("pwd"));

            let search_box = PasswordSearchBox::new();
            search_box.setup_user(user.clone(), user_key.clone());
            stack.add_named(&search_box, Some("search"));
            item.set_child(Some(&stack));
        }});
        factory.connect_bind(glib::clone!(@weak self as self_clone => move |fact, item| {
            let (prop, search_box, list_box, stack) = PasswordWindow::parse_list_item(item);
            let visible_child = if (prop.is_search()) {
                "search"
            } else {
                list_box.set_site_name(&prop.site().unwrap().get_name());
                "pwd"
            };
            stack.set_visible_child_name(visible_child);
            let self_c = self_clone.clone();
            search_box.connect_local("copy-create-activated", false, move |args|{
                let site_name = args[1].get::<String>().unwrap();
                println!("copy-create-activated: {}",site_name);
                self_c.activate_copy_or_create(&site_name);
                None
            });
            let self_c = self_clone.clone();
            search_box.connect_local("search-changed", false, move |args|{
                let site_name = args[1].get::<String>().unwrap();
                self_c.filter_site_list(&site_name);
                println!("search-changed: {}",&site_name);
                None
            });
            // let itemx = gtk::PropertyExpression::new(gtk::ListItem::static_type(), gtk::NONE_EXPRESSION, "item");
            // let stringx = gtk::PropertyExpression::new(gtk::StringObject::static_type(), Some(&itemx), "string");
            // println!("evaluation of stirngx: {}", stringx.evaluate(Some(item)).unwrap().get::<String>().unwrap());
        }));
        // factory.connect_unbind(|fact, item| {
        // let (prop, search_box, _, _) = PasswordWindow::parse_list_item(item);
        // if prop == "___search" {
        //     search_box.set_site_name(&prop);
        // }
        // });
    }

    pub fn parse_list_item(item: &gtk::ListItem) -> (GSite, PasswordSearchBox, PasswordListBox, gtk::Stack) {
        let stack = item.child().unwrap().downcast::<gtk::Stack>().ok().unwrap();
        (
            
            item.item().unwrap().downcast::<GSite>().ok().unwrap(),
            // item.item().unwrap().property("string").ok().unwrap().get::<String>().ok().unwrap(),
            stack.child_by_name("search").unwrap().downcast::<PasswordSearchBox>().ok().unwrap(),
            stack.child_by_name("pwd").unwrap().downcast::<PasswordListBox>().ok().unwrap(),
            stack,
        )
    }

    pub fn fill_site_list(&self) {
        let self_ = &imp::PasswordWindow::from_instance(self);
        let site_list = self_.user.borrow().unwrap().get_sites();
        self.get_store().remove_all();
        // let store = self_.string_store.model().unwrap().downcast::<gtk::StringList>().ok().unwrap();
        let store = self.get_store();
        // store.append("___search");
        store.append(&GSite::new_search());
        for site in site_list.iter().rev() {
            unsafe {
                let site_name: String = (**site).get_name();
                // store.append(&site_name);
                store.append(&GSite::new_with_site(&**site));
            }
        }
    }

    fn get_string_list(&self) -> gtk::StringList {
        let self_ = &imp::PasswordWindow::from_instance(self);
        let store = &self_.filter_store;
        store.model().unwrap().downcast::<gtk::StringList>().unwrap()
    }
    fn get_store(&self) -> gtk::gio::ListStore {
        let self_ = &imp::PasswordWindow::from_instance(self);
        let store = &self_.filter_store;
        store.model().unwrap().downcast::<gtk::gio::ListStore>().unwrap()
    }

    pub fn filter_site_list(&self, filter_str: &str) {
        let self_ = &imp::PasswordWindow::from_instance(self);
        let filter = self_.filter_store.filter().unwrap().downcast::<gtk::CustomFilter>().ok().unwrap();
        let f_str = String::from(filter_str.clone());
        filter.set_filter_func(move |obj| {
            let g_site = obj.downcast_ref::<GSite>().unwrap();
            if g_site.is_search() {return true}
            let s = g_site.site().unwrap().get_name();
            s.contains(&f_str)
        });
        println!("filtering with: {}", filter);
    }
    pub fn activate_copy_or_create(&self, site_name: &String) {
        let self_ = &imp::PasswordWindow::from_instance(self);
        let usr = self_.user.clone();
        let key = self_.user_key.clone();
        // TODO remove hardcoded password_type
        let password_type: spectre::ResultType = spectre::ResultType::TemplateLong;

        if usr.borrow().as_ref().unwrap().has_site(site_name) {
            let pwd = spectre::site_result(&site_name, *key.borrow().as_ref().unwrap(), password_type, spectre::AlgorithmVersionDefault);
            helper::copy_to_clipboard_with_notification(&self_.list_view, &pwd);
            self.hide();
        } else {
            let user_clone = self_.user.clone();
            let self_clone = self.clone();
            let name_clone = site_name.clone();
            let window = self_.list_view.root().unwrap().downcast::<gtk::Window>().ok().unwrap();
            PasswordWindow::show_accept_new_site_dialog(&window, &site_name, move || {
                user_clone.borrow_mut().as_mut().unwrap().add_site(
                    name_clone.clone().as_str(),
                    spectre::ResultType::TemplateLong,
                    1,
                    spectre::AlgorithmVersionDefault,
                );

                match spectre::marshal_write_to_file(spectre::MarshalFormat::flat, *usr.borrow().as_ref().unwrap()) {
                    Ok(a) => println!("succsesfully wrote to file"),
                    Err(r) => println!("err {}", r),
                }
                // reload site list:
                self_clone.fill_site_list();
            });
        }
    }
    // fn show_accept_new_site_dialog(){}
    fn show_accept_new_site_dialog<F: Fn() + 'static>(win: &gtk::Window, site_name: &String, accepted: F) {
        // let self_ = &imp::PasswordWindow::from_instance(self);
        let dialog = gtk::MessageDialog::new(
            Some(win),
            gtk::DialogFlags::MODAL,
            gtk::MessageType::Question,
            gtk::ButtonsType::YesNo,
            "Do you want to add:",
        );
        dialog.set_default_response(gtk::ResponseType::Yes);
        dialog.set_secondary_text(Some(&format!(" {} (Press Enter to add)", site_name)));
        dialog.connect_response(move |dialog, response| {
            println!("{}", response);
            match response {
                gtk::ResponseType::Yes => {
                    dialog.close();
                    accepted();
                }
                gtk::ResponseType::No => dialog.close(),
                default => println!("Message Dialog dismissed"),
            };
        });
        dialog.show();
    }
}

// FILTER
/*
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

 site_list.set_model(Some(&filter));
*/
