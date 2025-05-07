use crate::model::g_site::*;
use crate::spectre;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::Label;
use std::cell::{RefCell, RefMut};
use std::env;
use std::rc::Rc;
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
        pub copy_button: RefCell<Option<gtk::Button>>,
        pub password_label: RefCell<Option<gtk::Entry>>,
        password_show_button: RefCell<Option<gtk::Button>>,
        hbox_top: RefCell<Option<gtk::Box>>,
        hbox_bottom: RefCell<Option<gtk::Box>>,
        pub site: RefCell<Option<GSite>>,
        pub user: Rc<RefCell<Option<spectre::User>>>,
        pub user_key: Rc<RefCell<Option<spectre::UserKey>>>,
        pub password_version_button: RefCell<Option<gtk::Button>>,
        pub password_type_button: RefCell<Option<gtk::Button>>,
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
        fn constructed(&self) {
            let obj = self.obj();
            self.parent_constructed();
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

            let site_label = Label::new(Some("github.com"));
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

            let hbox_linked = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            hbox_linked.add_css_class("linked");

            let password_type_button = gtk::Button::with_label("type");
            password_type_button.add_css_class("tiny");
            password_type_button.set_sensitive(false);
            hbox_linked.append(&password_type_button);

            let password_version_button = gtk::Button::with_label("version");
            password_version_button.add_css_class("tiny");
            password_version_button.set_sensitive(false);
            hbox_linked.append(&password_version_button);

            hbox_bottom_left_bottom.append(&hbox_linked);

            let password_show_button = gtk::Button::with_label("Hidden");
            password_show_button.set_halign(gtk::Align::End);
            password_show_button.add_css_class("tiny");
            password_show_button.set_has_frame(false);
            password_show_button.connect_clicked(glib::clone!(@weak password_label, @weak password_show_button => move |_| {
                let is_visible = gtk::prelude::EntryExt::is_visible(&password_label);
                password_label.set_visibility(!is_visible);
                if is_visible{
                    password_show_button.set_label("Hidden");
                }else{
                    password_show_button.set_label("Shown");
                }
            }));
            hbox_bottom_left_bottom.append(&password_show_button);

            let copy_button = gtk::Button::with_label("Copy");
            copy_button.set_valign(gtk::Align::End);
            copy_button.set_size_request(120, -1);
            copy_button.add_css_class("suggested-action");
            copy_button.connect_clicked(glib::clone!(@weak obj, @strong copy_button => move |_| {
                let self_ = PasswordListBox::from_instance(&obj);
                crate::ui::password_window::helper::copy_to_clipboard_with_notification(&copy_button, &self_.site.borrow().as_ref().unwrap().get_password(*self_.user_key.borrow().as_ref().unwrap()));
            }));
            hbox_bottom.append(&copy_button);
            *self.password_version_button.borrow_mut() = Some(password_version_button);
            *self.password_type_button.borrow_mut() = Some(password_type_button);
            *self.hbox_top.borrow_mut() = Some(hbox_top);
            *self.hbox_bottom.borrow_mut() = Some(hbox_bottom);
            *self.site_label.borrow_mut() = Some(site_label);
            *self.copy_button.borrow_mut() = Some(copy_button);
            *self.password_label.borrow_mut() = Some(password_label);
            *self.password_show_button.borrow_mut() = Some(password_show_button);
        }

        fn dispose(&self) {
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
        glib::Object::builder().build()
    }

    pub fn setup_user(&self, usr: Rc<RefCell<Option<spectre::User>>>, usr_key: Rc<RefCell<Option<spectre::UserKey>>>) {
        let mut self_ = imp::PasswordListBox::from_instance(self);
        self_.user.replace(*usr.borrow());
        self_.user_key.replace(*usr_key.borrow());
    }

    pub fn set_site(&self, site: &GSite) {
        let self_ = imp::PasswordListBox::from_instance(&self);
        self_.site_label.borrow().as_ref().unwrap().set_text(&site.name());
        let s_pwd = site.get_password(*self_.user_key.borrow().as_ref().unwrap());
        let s_version = site.descriptor().algorithm_version;
        let s_type = site.descriptor().result_type;
        self_.password_label.borrow().as_ref().unwrap().set_text(&s_pwd);
        self_.password_type_button.borrow().as_ref().unwrap().set_label(&s_type.to_string());
        self_.password_version_button.borrow().as_ref().unwrap().set_label(&s_version.to_string());
        *self_.site.borrow_mut() = Some(site.clone());
    }
}
