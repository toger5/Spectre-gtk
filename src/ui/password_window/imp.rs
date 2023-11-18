use super::*;

#[derive(Debug)]
pub struct PasswordWindow {
    pub user: Rc<RefCell<Option<spectre::User>>>,
    pub user_key: Rc<RefCell<Option<spectre::UserKey>>>,
    // pub string_store: gtk::FilterListModel,
    pub filter_store: gtk::FilterListModel,
    pub list_view: gtk::ListView,
    pub entry_site: GSite,
    // pub signal_search_changed: Rc<RefCell<Option<glib::signal::SignalHandlerId>>>,
    // pub signal_copy_create_activated: Rc<RefCell<Option<glib::signal::SignalHandlerId>>>,
}

#[glib::object_subclass]
impl ObjectSubclass for PasswordWindow {
    const NAME: &'static str = "PasswordWindow";
    type Type = super::PasswordWindow;
    type ParentType = gtk::Window;

    fn new() -> Self {
        Self {
            filter_store: {
                let custom_sorter = gtk::CustomSorter::new(|a, b| {
                    let a_site = a.clone().downcast::<GSite>().ok().unwrap();
                    let b_site = b.clone().downcast::<GSite>().ok().unwrap();
                    if a_site.is_search() {
                        return gtk::Ordering::Smaller;
                    }
                    if b_site.is_search() {
                        return gtk::Ordering::Larger;
                    }
                    match a_site.site().unwrap().last_used() > b_site.site().unwrap().last_used() {
                        true => gtk::Ordering::Smaller,
                        false => gtk::Ordering::Larger,
                    }
                });
                use crate::model::g_site::GSite;
                use gtk::gio;
                let custom_filter = gtk::CustomFilter::new(|_| true);

                let site_store = gio::ListStore::new(GSite::static_type());
                let sort_site_store = gtk::SortListModel::new(Some(&site_store), Some(&custom_sorter));
                gtk::FilterListModel::new(Some(&sort_site_store), Some(&custom_filter))
            },
            list_view: gtk::ListView::new(Option::<&gtk::NoSelection>::None, Option::<&gtk::SignalListItemFactory>::None),
            entry_site: GSite::new_search(),
            user: Rc::new(RefCell::new(None)),
            user_key: Rc::new(RefCell::new(None)),
            // signal_search_changed: Rc::new(RefCell::new(None)),
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
        println!("close-but-no-logout");
        // let app = self.get_child().unwrap().root().unwrap().downcast::<gtk::Window>().ok().unwrap().application().unwrap();
        // self.parent_close_request(window);
        // app.quit();
        glib::signal::Inhibit(false)
    }
}
