use super::*;

#[derive(Debug)]
pub struct PasswordWindow {
    pub user: Rc<RefCell<Option<spectre::User>>>,
    pub user_key: Rc<RefCell<Option<spectre::UserKey>>>,
    pub string_store: gtk::StringList,
    pub list_view: gtk::ListView,
    pub entry_site_name: Option<String>,
}

#[glib::object_subclass]
impl ObjectSubclass for PasswordWindow {
    const NAME: &'static str = "PasswordWindow";
    type Type = super::PasswordWindow;
    type ParentType = gtk::Window;

    fn new() -> Self {
        Self {
            string_store: gtk::StringList::new(&[]),
            list_view: gtk::ListView::new(
                Option::<&gtk::NoSelection>::None,
                Option::<&gtk::SignalListItemFactory>::None,
            ),
            entry_site_name: Option::<String>::None,
            user: Rc::new(RefCell::new(None)),
            user_key: Rc::new(RefCell::new(None)),
        }
    }
}
impl ObjectImpl for PasswordWindow {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        obj.set_default_size(550, 800);

        let sw = gtk::ScrolledWindow::new();
        sw.set_child(Some(&self.list_view));
        sw.set_min_content_height(300);
        sw.set_min_content_width(500);
        sw.set_propagate_natural_width(true);
        sw.set_propagate_natural_height(true);
        let b = gtk::Box::new(gtk::Orientation::Vertical, 10);
        b.append(&sw);
        obj.set_child(Some(&b));
    }
    fn dispose(&self, obj: &Self::Type) {
        //TODO unparent childs
    }
}
impl WidgetImpl for PasswordWindow {
    // fn show(&self, obj: &Self::Type){
    //     self.parent_show(obj);
    //     // obj.entry_site_name.set = "";
    // }
}
impl WindowImpl for PasswordWindow {
    fn close_request(&self, window: &Self::Type) -> glib::signal::Inhibit {
        self.parent_close_request(window)
    }
}
