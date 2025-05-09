use adw::prelude::*;
use gtk::{gio, glib};
use std::{cell::RefCell, env::args};

use adw::subclass::prelude::*;

mod imp {
    use super::*;
    // use gtk::subclass::prelude::*;

    // By implementing Default we don't have to provide a `new` fn in our ObjectSubclass impl.
    #[derive(Default)]
    pub struct SpectreApp {
        pub username: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SpectreApp {
        const NAME: &'static str = "SpectreApp";
        type Type = super::SpectreApp;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for SpectreApp {}
    impl ApplicationImpl for SpectreApp {
        fn activate(&self) {
            // *self.username.borrow_mut() = String::from("Firstname Lastname");

            // We create our window at activation stage
            let window = adw::ApplicationWindow::new(self.obj().as_ref());
            window.set_default_size(600, 350);
            window.set_title(Some("Spectre"));

            let label = gtk::Label::new(Some(&self.username.borrow()));
            label.add_css_class("title-2");
            window.set_child(Some(&label));
            window.show();
        }
    }
    impl GtkApplicationImpl for SpectreApp {}
}

glib::wrapper! {
    pub struct SpectreApp(ObjectSubclass<imp::SpectreApp>) @extends gio::Application, gtk::Application, @implements gio::ActionGroup, gio::ActionMap;
}

impl SpectreApp {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", "org.gtk_rs.application-subclass")
            .property("flags", gio::ApplicationFlags::empty())
            .build()
    }
    pub fn with_username(username: &str) -> Self {
        let app = SpectreApp::new();
        let app_ = imp::SpectreApp::from_instance(&app);
        *app_.username.borrow_mut() = username.to_owned();
        app
    }
    pub fn update_username(&self, name: &str) {
        let self_ = imp::SpectreApp::from_instance(self);
        *self_.username.borrow_mut() = name.to_owned();
    }
}
