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
        create_copy_button: RefCell<Option<gtk::Button>>,
        pub site_name: Rc<RefCell<Option<String>>>,
        pub user: Rc<RefCell<Option<spectre::User>>>,
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
                .label("Haga0.RenoBetu")
                .css_classes(vec![String::from("monospace"),String::from("pwd-preview")]).build();
            
            obj.append(&password_label);


            let hbox_top = Box::new(Orientation::Horizontal, 30);
            obj.append(&hbox_top);

            let hbox_bottom = gtk::Box::new(gtk::Orientation::Horizontal, 30);
            obj.append(&hbox_bottom);

            let site_entry = EntryBuilder::new()
                .halign(Align::Start)
                .css_classes(vec![
                    String::from("site-name-entry"),
                    String::from("pwd-preview"),
                ])
                .build();
            hbox_top.append(&site_entry);

            let hbox_bottom_left = gtk::Box::new(gtk::Orientation::Vertical, 0);
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
            let password_show_button = gtk::Button::with_label("Hidden");
            password_show_button.set_halign(gtk::Align::End);
            password_show_button.add_css_class("tiny");
            password_show_button.set_has_frame(false);
            password_show_button.connect_clicked(
                glib::clone!(@weak password_label, @weak password_show_button => move |_| {
                    // let is_visible = gtk::EntryExt::is_visible(&password_label);
                    // password_label.set_visibility(!is_visible);
                    // if is_visible{
                    //     password_show_button.set_label("Hidden");
                    // }else{
                    //     password_show_button.set_label("Shown");
                    // }
                    //TODO make visiblility fake...
                    password_label.set_text("HIDDEN");
                }),
            );
            hbox_bottom_left_bottom.append(&password_show_button);

            let create_copy_button = gtk::Button::with_label("Copy");
            create_copy_button.set_valign(gtk::Align::End);
            create_copy_button.set_size_request(120, -1);
            create_copy_button.add_css_class("suggested-action");
            create_copy_button.connect_clicked(
                glib::clone!(@weak obj, @weak create_copy_button => move |_| {
                    create_copy_button.clipboard().set_text(&obj.get_password())
                }),
            );
            hbox_top.append(&create_copy_button);

            *self.hbox_top.borrow_mut() = Some(hbox_top);
            *self.hbox_bottom.borrow_mut() = Some(hbox_bottom);
            // *self.site_label.borrow_mut() = Some(site_label);
            *self.create_copy_button.borrow_mut() = Some(create_copy_button);
            // *self.password_label.borrow_mut() = Some(password_label);
            // *self.password_show_button.borrow_mut() = Some(password_show_button);
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

    pub fn setup_user(
        &self,
        usr: Rc<RefCell<Option<spectre::User>>>,
        usr_key: Rc<RefCell<Option<spectre::UserKey>>>,
    ) {
        let mut self_ = imp::PasswordSearchBox::from_instance(&self).clone();
        self_.user.replace(*usr.borrow());
        self_.user_key.replace(*usr_key.borrow());
        //*self_.user.borrow_mut() = *usr.borrow();
    }

    pub fn set_site_name(&self, name: &str) {
        let self_ = imp::PasswordSearchBox::from_instance(&self);
        // self_.password_label.borrow().as_ref().unwrap().set_text(spectre::site_result(name, user_key: UserKey, result_type: ResultType, algorithm_version: AlgorithmVersion));
        // self_.site_label.borrow().as_ref().unwrap().set_text(name);
        *self_.site_name.borrow_mut() = Some(String::from(name));
        self_
            .password_label
            .borrow()
            .as_ref()
            .unwrap()
            .set_text(&self.get_password());
    }

    pub fn get_password(&self) -> String {
        let self_ = imp::PasswordSearchBox::from_instance(&self);

        // TODO remove hardcoded password_type
        let password_type: spectre::ResultType = spectre::ResultType::TemplateLong;
        spectre::site_result(
            self_.site_name.borrow().as_ref().unwrap().as_str(),
            *self_.user_key.borrow().as_ref().unwrap(),
            password_type,
            spectre::AlgorithmVersionDefault,
        )
    }
}
