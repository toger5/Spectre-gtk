use std::{cell::{RefCell, RefMut}};
use std::rc::Rc;
use std::env;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::spectre;
mod imp {
    use super::*;
    // use gtk::subclass::prelude::*;

    #[derive(Debug, Default)]
    pub struct PasswordListBox {
        /// Reference to the child widget.
        ///
        /// In our case it's a text label for the PasswordListBox. Since this example only uses a
        /// `gtk::Label`, the type could've been `Option<gtk::Label>`. However, a real PasswordListBox might
        /// switch between a label widget and an icon widget, and in general your widget can contain
        /// arbitrary children. Thus we used `Option<gtk::Widget>` to show how to handle any widget
        /// and to make the example easier to tweak and play with.
        ///
        /// Widgets automatically store strong references to their children, added in `set_parent()`
        /// and removed in `unparent()`. Therefore, this field could be a `WeakRef<gtk::Widget>`.
        /// Using a strong reference is just a little clearer.

        pub site_label: RefCell<Option<gtk::Label>>,
        copy_button: RefCell<Option<gtk::Button>>,
        pub password_label: RefCell<Option<gtk::Entry>>,
        password_show_button: RefCell<Option<gtk::Button>>,
        hbox_top: RefCell<Option<gtk::Box>>,
        hbox_bottom: RefCell<Option<gtk::Box>>,
        pub site_name: Rc<RefCell<Option<String>>>,
        pub user: Rc<RefCell<Option<spectre::User>>>,
        pub user_key: Rc<RefCell<Option<spectre::UserKey>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PasswordListBox {
        const NAME: &'static str = "PasswordListBox";
        type Type = super::PasswordListBox;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            // klass.set_layout_manager_type::<gtk::BinLayout>();

            // Make it look like a GTK Entry.
            klass.set_css_name("box");
            // klass.add_css
        }
    }
    // impl
    impl ObjectImpl for PasswordListBox {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.set_css_classes(&["view", "top", "bottom"]);
            obj.set_halign(gtk::Align::Center);
            obj.set_size_request(450, -1);
            obj.set_valign(gtk::Align::Start);
            obj.set_orientation(gtk::Orientation::Vertical);
            obj.set_spacing(10);

            let hbox_top = gtk::Box::new(gtk::Orientation::Horizontal, 30);
            obj.append(&hbox_top);
            
            let hbox_bottom = gtk::Box::new(gtk::Orientation::Horizontal, 30);
            obj.append(&hbox_bottom);

            let site_label = gtk::Label::new(Some("github.com"));
            site_label.set_hexpand(true);
            site_label.add_css_class("site-name-label");
            hbox_top.append(&site_label);
            
            let hbox_bottom_left = gtk::Box::new(gtk::Orientation::Vertical, 0);
            hbox_bottom.append(&hbox_bottom_left);
            
            let password_label = gtk::Entry::new();
            password_label.set_hexpand(true);
            password_label.set_text("Haga0.RenoBetu");
            password_label.add_css_class("monospace");
            password_label.set_visibility(false);
            password_label.set_has_frame(false);
            password_label.set_can_focus(false);
            hbox_bottom_left.append(&password_label);

            let hbox_bottom_left_bottom = gtk::Box::new(gtk::Orientation::Horizontal, 20);
            hbox_bottom_left.append(&hbox_bottom_left_bottom);

            let hbox_linked = gtk::Box::new(gtk::Orientation::Horizontal,0);
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
                    let is_visible = gtk::EntryExt::is_visible(&password_label);
                    password_label.set_visibility(!is_visible);
                    if is_visible{
                        password_show_button.set_label("Hidden");
                    }else{
                        password_show_button.set_label("Shown");
                    }
                }),
            );
            hbox_bottom_left_bottom.append(&password_show_button);


            let copy_button = gtk::Button::with_label("Copy");
            copy_button.set_valign(gtk::Align::End);
            copy_button.set_size_request(120,-1);
            copy_button.add_css_class("suggested-action");
            let usr_clone = self.user.clone();
            let usr_key_clone = self.user_key.clone();
            let site_name_clone = self.site_name.clone();
            let copy_button_clone = copy_button.clone();
            let obj_clone = obj.clone();
            copy_button.connect_clicked( move |_| {
                    println!("password should be copied... but is not... yet {:?}", usr_clone.borrow());
                    // let m_k = usr_key_clone.borrow().expect("NO MASTER KEY GOT DAMMIT");
                    // TODO remove hardcoded password_type
                    
                    copy_button_clone.clipboard().set_text(&obj_clone.get_password());

                    println!(
                        "pwd for site {} ({:}) saved to clipboard",
                        site_name_clone.clone().borrow().as_ref().unwrap().as_str(),
                        spectre::c_char_to_string(usr_clone.borrow().unwrap().userName)
                    );
                },
            );
            hbox_bottom.append(&copy_button);
            
            *self.hbox_top.borrow_mut() = Some(hbox_top);
            *self.hbox_bottom.borrow_mut() = Some(hbox_bottom);
            *self.site_label.borrow_mut() = Some(site_label);
            *self.copy_button.borrow_mut() = Some(copy_button);
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

    impl WidgetImpl for PasswordListBox {}
    impl BoxImpl for PasswordListBox {}
}
glib::wrapper! {
    pub struct PasswordListBox(ObjectSubclass<imp::PasswordListBox>)
    @extends gtk::Box, gtk::Widget, @implements gtk::ConstraintTarget, gtk::Orientable;
}

impl PasswordListBox {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create PasswordListBox")
    }
    pub fn setup_user(&self, usr: Rc<RefCell<Option<spectre::User>>>,usr_key: Rc<RefCell<Option<spectre::UserKey>>>){
        let mut self_ = imp::PasswordListBox::from_instance(&self).clone();
        self_.user.replace(*usr.borrow());
        self_.user_key.replace(*usr_key.borrow());
        //*self_.user.borrow_mut() = *usr.borrow();
    }
    pub fn set_site_name(&self, name: &str) {
        let self_ = imp::PasswordListBox::from_instance(&self);
        // self_.password_label.borrow().as_ref().unwrap().set_text(spectre::site_result(name, user_key: UserKey, result_type: ResultType, algorithm_version: AlgorithmVersion));
        self_.site_label.borrow().as_ref().unwrap().set_text(name);
        *self_.site_name.borrow_mut() = Some(String::from(name));
        self_.password_label.borrow().as_ref().unwrap().set_text(&self.get_password());
    }
    pub fn get_password(&self) -> String {
        let self_ = imp::PasswordListBox::from_instance(&self);
        let password_type: spectre::ResultType = spectre::ResultType::TemplateLong;
        spectre::site_result(
            self_.site_name.borrow().as_ref().unwrap().as_str(),
            *self_.user_key.borrow().as_ref().unwrap(),
            password_type,
            spectre::AlgorithmVersionDefault,
        )
    }
}
