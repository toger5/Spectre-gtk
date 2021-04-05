use std::cell::RefCell;
use std::env;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

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
        password_label: RefCell<Option<gtk::Entry>>,
        password_show_button: RefCell<Option<gtk::Button>>,
        hbox_top: RefCell<Option<gtk::Box>>,
        hbox_bottom: RefCell<Option<gtk::Box>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PasswordListBox {
        const NAME: &'static str = "PasswordListBox";
        type Type = super::PasswordListBox;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            // The layout manager determines how child widgets are laid out.
            klass.set_layout_manager_type::<gtk::BinLayout>();

            // Make it look like a GTK PasswordListBox.
            // klass.set_css_name("PasswordListBox");
        }
    }
    // impl
    impl ObjectImpl for PasswordListBox {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            // let vbox_main = gtk::Box::new(gtk::Orientation::Vertical,30);
            // obj.set_orientation(gtk::Orientation::Vertical);
            let hbox_top = gtk::Box::new(gtk::Orientation::Vertical, 30);
            *self.hbox_top.borrow_mut() = Some(hbox_top);
            let hbox_bottom = gtk::Box::new(gtk::Orientation::Vertical, 30);
            *self.hbox_bottom.borrow_mut() = Some(hbox_bottom);
            obj.append(self.hbox_top.borrow().as_ref().unwrap());
            obj.append(self.hbox_bottom.borrow().as_ref().unwrap());
            // Create the child label.
            let site_label = gtk::Label::new(Some("github.com"));
            self.hbox_top.borrow().as_ref().unwrap().append(&site_label);
            *self.site_label.borrow_mut() = Some(site_label);
            let copy_button = gtk::Button::with_label("Copy");
            self.hbox_top.borrow().as_ref().unwrap().append(self.site_label.borrow().as_ref().unwrap());
            *self.copy_button.borrow_mut() = Some(copy_button);
            let password_label = gtk::Entry::new();
            password_label.set_text("Haga0.RenoBetu");
            password_label.set_visibility(false);
            password_label.set_has_frame(false);
            password_label.set_editable(false);
            self.hbox_bottom.borrow().as_ref().unwrap().append(&password_label);
            let password_show_button = gtk::Button::with_label("Show");
            password_show_button.set_has_frame(false);
            self.hbox_bottom.borrow().as_ref().unwrap().append(&password_show_button);
            password_show_button.connect_activate(glib::clone!(@weak password_label => move |_| {
                password_label.set_visibility(!password_label.get_visibility());
            }));
            *self.password_label.borrow_mut() = Some(password_label);
            // button_increase.connect_clicked(clone!(@strong number => move |_| {
            //     *number.borrow_mut() += 1;
            // }));
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
    pub fn set_site_name(&self, name: &str){
        let self_ = imp::PasswordListBox::from_instance(&self);
        self_.site_label.borrow().as_ref().unwrap().set_text(name);
    }
}
