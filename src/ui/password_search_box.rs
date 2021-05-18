use crate::spectre;
use crate::model::g_site::*;
use glib::subclass::Signal;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;
use std::cell::{RefCell, RefMut};
use std::env;
use std::rc::Rc;
mod imp {
    use super::*;
    // use gtk::subclass::prelude::*;

    #[derive(Debug, Default)]
    pub struct PasswordSearchBox {
        pub site: RefCell<Option<GSite>>,
        pub password_label: RefCell<Option<gtk::Label>>,
        pub site_entry: RefCell<Option<gtk::Entry>>,
        pub create_copy_button: RefCell<Option<gtk::Button>>,
        pub user: Rc<RefCell<Option<spectre::User>>>,
        pub user_key: Rc<RefCell<Option<spectre::UserKey>>>,
        //TODO rethink when the pwd is shown...
        pub password_show_button: RefCell<Option<gtk::ToggleButton>>,
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
    impl ObjectImpl for PasswordSearchBox {
        fn constructed(&self, obj: &Self::Type) {
            use gtk::*;
            self.parent_constructed(obj);
            obj.set_css_classes(&["view", "top", "bottom"]);
            obj.set_halign(Align::Center);
            obj.set_size_request(450, -1);
            obj.set_valign(Align::Start);
            obj.set_orientation(Orientation::Vertical);
            obj.set_spacing(20);
            obj.set_margin_bottom(50);
            obj.set_margin_top(50);

            let password_label = gtk::LabelBuilder::new()
                .hexpand_set(true)
                .halign(Align::Fill)
                .label("Password")
                .css_classes(vec![String::from("monospace"), String::from("pwd-preview")])
                .build();
            obj.append(&password_label);

            let hbox_top = Box::new(Orientation::Horizontal, 15);
            obj.append(&hbox_top);

            let hbox_bottom = gtk::Box::new(gtk::Orientation::Horizontal, 30);
            obj.append(&hbox_bottom);

            let site_entry = EntryBuilder::new()
                .halign(Align::Fill)
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
                .valign(Align::Fill)
                .width_request(120)
                .label("Copy")
                .sensitive(false)
                .css_classes(vec![String::from("suggested-action")])
                .build();
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

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![
                    Signal::builder("search-changed", &[GSite::static_type().into()], <()>::static_type().into()).build(),
                    Signal::builder("copy-create-activated", &[GSite::static_type().into()], <()>::static_type().into()).build(),
                ]
            });
            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for PasswordSearchBox {}
    impl BoxImpl for PasswordSearchBox {}
}

glib::wrapper! {
    pub struct PasswordSearchBox(ObjectSubclass<imp::PasswordSearchBox>)
    @extends gtk::Box, gtk::Widget, @implements gtk::ConstraintTarget, gtk::Orientable;
}
pub enum CopyButtonMode {
    Create,
    Disabled,
    Copy,
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
    fn set_copy_button_mode(&self, mode: &CopyButtonMode) {
        let self_ = imp::PasswordSearchBox::from_instance(&self);
        let create_copy_button = self_.create_copy_button.borrow().as_ref().unwrap().clone();
        match mode {
            CopyButtonMode::Copy => {
                create_copy_button.set_label("Copy");
                create_copy_button.set_sensitive(true);
                create_copy_button.set_css_classes(&["suggested-action"]);
            }
            CopyButtonMode::Create => {
                create_copy_button.set_label("Create");
                create_copy_button.set_sensitive(true);
                create_copy_button.set_css_classes(&["create-action"]);
            }
            CopyButtonMode::Disabled => {
                create_copy_button.set_label("Copy");
                create_copy_button.set_sensitive(false);
                create_copy_button.set_css_classes(&[]);
            }
        }
    }
    fn calculate_copy_button_mode(&self, ) -> CopyButtonMode {
        let self_ = imp::PasswordSearchBox::from_instance(&self);
        let user = self_.user.clone();
        let entry = self_.site_entry.clone();
        if user.borrow().as_ref().unwrap().has_site(&entry.borrow().as_ref().unwrap().text().to_string()) {
            CopyButtonMode::Copy
        } else if entry.borrow().as_ref().unwrap().text().len() > 0 {
            CopyButtonMode::Create
        } else {
            CopyButtonMode::Disabled
        }
    }
    fn connect_events(&self) {
        let self_ = imp::PasswordSearchBox::from_instance(&self);
        let self_clone = self.clone();
        let user = self_.user.clone();
        let create_copy_button = self_.create_copy_button.borrow().as_ref().unwrap().clone();
        self_.site_entry.borrow().as_ref().unwrap().connect_changed(move |entry| {
            let self_ = imp::PasswordSearchBox::from_instance(&self_clone);
            self_clone.update_password_label();
            self_clone.set_copy_button_mode(&self_clone.calculate_copy_button_mode());
            self_.site.borrow().as_ref().unwrap().set_descriptor_name(&entry.text().to_string());
            self_clone.emit_by_name("search-changed", &[&self_.site.borrow().as_ref().unwrap()]).unwrap();
        });

        let self_clone = self.clone();
        let password_show_button = self_.password_show_button.borrow().as_ref().unwrap().clone();
        password_show_button.connect_toggled(move |button| {
            self_clone.update_password_label();
        });
        let self_clone = self.clone();
        let create_copy_button = self_.create_copy_button.borrow().as_ref().unwrap().clone();
        let entry = self_.site_entry.borrow().as_ref().unwrap().clone();
        // let self_site = self_.site.borrow().as_ref().unwrap().clone();
        create_copy_button.connect_clicked(glib::clone!(@weak self as self_clone => move |_| {
            // button.clipboard().set_text(&self_clone.get_password_for_label())
            let self_ = imp::PasswordSearchBox::from_instance(&self_clone);

            self_clone.emit_by_name("copy-create-activated",  &[&self_.site.borrow().as_ref().unwrap()]).unwrap();
        }));
        // let self_site = self_.site.borrow().as_ref().unwrap().clone();
        let site_entry = self_.site_entry.borrow().as_ref().unwrap().clone();
        site_entry.connect_activate(glib::clone!(@weak self as self_clone => move |entry|{
            let self_ = imp::PasswordSearchBox::from_instance(&self_clone);
            self_clone.emit_by_name("copy-create-activated",  &[&self_.site.borrow().as_ref().unwrap()]).unwrap();
        }));
        // button.connect_clicked(clone!(@weak self as tag => move |_btn| {
            
        // }));
    }
    pub fn set_site(&self, site: &GSite) {
        let self_ = imp::PasswordSearchBox::from_instance(&self);
        *self_.site.borrow_mut() = Some(site.clone());
        self_.site_entry.borrow().as_ref().unwrap().set_text(&site.descriptor_name());
        self.update_password_label();
    }
    fn update_password_label(&self) {
        let self_ = imp::PasswordSearchBox::from_instance(&self);
        let self_clone = self.clone();
        let password_label = self_.password_label.borrow().as_ref().unwrap().clone();
        password_label.set_text(self_clone.get_password_for_label().as_str());
    }
    pub fn get_password_for_label(&self) -> String {
        let self_ = imp::PasswordSearchBox::from_instance(&self);

        // TODO remove hardcoded password_type
        let password_type: spectre::ResultType = spectre::ResultType::TemplateLong;
        let password_show_button = self_.password_show_button.borrow().as_ref().unwrap().clone();
        if password_show_button.is_active() {
            return String::from("Hidden");
        }
        let site_entry = self_.site_entry.borrow().as_ref().unwrap().clone();
        if site_entry.text().len() > 0 {
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
