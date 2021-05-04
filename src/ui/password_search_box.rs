use std::cell::{RefCell, RefMut};
use std::env;
use std::rc::Rc;

use crate::spectre;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
mod imp {
    use super::*;
    // use gtk::subclass::prelude::*;

    #[derive(Debug, Default)]
    pub struct PasswordSearchBox {
        pub password_label: RefCell<Option<gtk::Label>>,
        pub site_entry: RefCell<Option<gtk::Entry>>,
        create_copy_button: RefCell<Option<gtk::Button>>,
        pub user: Rc<RefCell<Option<spectre::User>>>,
        pub password_show_button: RefCell<Option<gtk::ToggleButton>>,
        pub user_key: Rc<RefCell<Option<spectre::UserKey>>>,
        pub hbox_bottom: RefCell<Option<gtk::Box>>,
        pub hbox_top: RefCell<Option<gtk::Box>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PasswordSearchBox {
        const NAME: &'static str = "PasswordSearchBox";
        type Type = super::PasswordSearchBox;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("box");
        }
    }
    // fn setup_
    // impl
    impl ObjectImpl for PasswordSearchBox {
        fn constructed(&self, obj: &Self::Type) {
            use gtk::*;
            self.parent_constructed(obj);
            obj.set_css_classes(&["view", "top", "bottom"]);
            obj.set_halign(Align::Center);
            obj.set_size_request(450, -1);
            obj.set_valign(Align::Start);
            obj.set_orientation(Orientation::Vertical);
            obj.set_spacing(10);
            obj.set_margin_bottom(50);
            obj.set_margin_top(50);

            // let password_label = gtk::EntryBuilder::new()
            //     .hexpand(false)
            //     .css_name("label")
            //     .text("Haga0.RenoBetu")
            //     .css_classes(vec![String::from("monospace"),String::from("pwd-preview")])
            //     .visibility(false)
            //     .has_frame(false)
            //     .can_focus(false)
            //     .halign(Align::Center)
            //     .build();
            let password_label = gtk::LabelBuilder::new()
                .hexpand_set(true)
                .label("Password")
                .css_classes(vec![String::from("monospace"), String::from("pwd-preview")])
                .build();
            obj.append(&password_label);

            let hbox_top = Box::new(Orientation::Horizontal, 30);
            obj.append(&hbox_top);

            let hbox_bottom = gtk::Box::new(gtk::Orientation::Horizontal, 30);
            obj.append(&hbox_bottom);

            let site_entry = EntryBuilder::new()
                .halign(Align::Start)
                .css_classes(vec![String::from("site-name-entry"), String::from("pwd-preview")])
                .hexpand(true)
                .build();
            hbox_top.append(&site_entry);

            let hbox_bottom_left = gtk::Box::new(gtk::Orientation::Vertical, 0);
            hbox_bottom_left.set_hexpand(true);
            hbox_bottom.append(&hbox_bottom_left);

            let hbox_bottom_left_bottom = gtk::Box::new(gtk::Orientation::Horizontal, 20);
            hbox_bottom_left.append(&hbox_bottom_left_bottom);

            let hbox_linked = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            hbox_linked.add_css_class("linked");
            let password_short_button = gtk::Button::with_label("short");
            password_short_button.add_css_class("tiny");
            let password_normal_button = gtk::Button::with_label("normal");
            password_normal_button.add_css_class("tiny");
            let password_long_button = gtk::Button::with_label("long");
            password_long_button.add_css_class("tiny");
            hbox_linked.append(&password_normal_button);
            hbox_linked.append(&password_short_button);
            hbox_linked.append(&password_long_button);
            hbox_bottom_left_bottom.append(&hbox_linked);
            let password_show_button = gtk::ToggleButton::with_label("Hidden");
            password_show_button.set_halign(gtk::Align::End);
            password_show_button.add_css_class("tiny");
            password_show_button.set_has_frame(false);
            hbox_bottom_left_bottom.append(&password_show_button);
            let create_copy_button = gtk::ButtonBuilder::new()
                .valign(gtk::Align::End)
                .vexpand(true)
                .width_request(120)
                .label("Copy")
                .sensitive(false)
                .css_classes(vec![String::from("suggested-action")])
                .build();
            // let create_copy_button = gtk::Button::with_label("Copy");
            // create_copy_button.set_valign(gtk::Align::End);
            // create_copy_button.set_size_request(120, -1);
            // create_copy_button.add_css_class("suggested-action");
            create_copy_button.connect_clicked(glib::clone!(@weak obj, @weak create_copy_button => move |_| {
                create_copy_button.clipboard().set_text(&obj.get_password())
            }));
            hbox_top.append(&create_copy_button);

            *self.hbox_top.borrow_mut() = Some(hbox_top);
            *self.hbox_bottom.borrow_mut() = Some(hbox_bottom);
            *self.site_entry.borrow_mut() = Some(site_entry);
            *self.create_copy_button.borrow_mut() = Some(create_copy_button);
            *self.password_label.borrow_mut() = Some(password_label);
            *self.password_show_button.borrow_mut() = Some(password_show_button);
        }

        fn dispose(&self, _obj: &Self::Type) {
            // Child widgets need to be manually unparented in `dispose()`.
            if let Some(child) = self.hbox_bottom.borrow_mut().take() {
                child.unparent();
            }
            if let Some(child) = self.hbox_top.borrow_mut().take() {
                child.unparent();
            }
        }
    }

    impl WidgetImpl for PasswordSearchBox {}
    impl BoxImpl for PasswordSearchBox {}
}

glib::wrapper! {
    pub struct PasswordSearchBox(ObjectSubclass<imp::PasswordSearchBox>)
    @extends gtk::Box, gtk::Widget, @implements gtk::ConstraintTarget, gtk::Orientable;
}

impl PasswordSearchBox {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create PasswordSearchBox")
    }

    pub fn setup_user(&self, usr: Rc<RefCell<Option<spectre::User>>>, usr_key: Rc<RefCell<Option<spectre::UserKey>>>) {
        let mut self_ = imp::PasswordSearchBox::from_instance(&self).clone();
        self_.user.replace(*usr.borrow());
        self_.user_key.replace(*usr_key.borrow());
        // *self_.user.borrow_mut() = *usr.borrow();
        self.connect_events();
    }

    fn connect_events(&self) {
        let self_ = imp::PasswordSearchBox::from_instance(&self);
        let self_clone = self.clone();
        let password_label = self_.password_label.borrow().as_ref().unwrap().clone();
        self_.site_entry.borrow().as_ref().unwrap().connect_changed( move |_| {
            password_label.set_text(self_clone.get_password().as_str());
        });

        let self_clone = self.clone();
        let password_show_button = self_.password_show_button.borrow().as_ref().unwrap().clone();
        // let password_label = self_.password_label.borrow().as_ref().unwrap().clone();
        password_show_button.connect_toggled(move |button| {
            self_clone.update_password_label()
        });
    }
    fn update_password_label(&self){
        let self_ = imp::PasswordSearchBox::from_instance(&self);
        let self_clone = self.clone();
        let password_label = self_.password_label.borrow().as_ref().unwrap().clone();
        password_label.set_text(self_clone.get_password().as_str());
    }
    pub fn set_site_name(&self, name: &str) {
        let self_ = imp::PasswordSearchBox::from_instance(&self);
        self_.site_entry.borrow().as_ref().unwrap().set_text(name);
        self_.password_label.borrow().as_ref().unwrap().set_text(&self.get_password());
    }

    pub fn get_password(&self) -> String {
        let self_ = imp::PasswordSearchBox::from_instance(&self);

        // TODO remove hardcoded password_type
        let password_type: spectre::ResultType = spectre::ResultType::TemplateLong;
        let password_show_button = self_.password_show_button.borrow().as_ref().unwrap().clone();
        if password_show_button.is_active() {return String::from("Hidden");}
        let site_entry = self_.site_entry.borrow().as_ref().unwrap().clone();
        if site_entry.text().len()>0 {
            spectre::site_result(
                site_entry.text().as_str(),
                *self_.user_key.borrow().as_ref().unwrap(),
                password_type,
                spectre::AlgorithmVersionDefault,
            )
        } else {
            String::from("")
        }
    }
}
