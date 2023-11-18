use crate::spectre;
use gtk::glib;
use gtk::subclass::prelude::*;
use std::cell::{Ref, RefCell};

#[derive(Clone)]
pub struct SiteDescriptor {
    pub site_name: RefCell<String>,
    pub result_type: spectre::ResultType,
    pub algorithm_version: spectre::AlgorithmVersion,
}
impl Default for SiteDescriptor {
    fn default() -> Self {
        Self {
            site_name: RefCell::new("".to_owned()),
            result_type: spectre::ResultTypeDefault,
            algorithm_version: spectre::ALGORITHM_VERSION_DEFAULT,
        }
    }
}
impl std::fmt::Debug for SiteDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.site_name)
            .field(&self.result_type.to_string())
            .field(&self.algorithm_version.to_string())
            .finish()
    }
}
mod imp {
    use super::*;
    use once_cell::sync::{Lazy, OnceCell};
    use std::cell::RefCell;
    #[derive(Debug, Default)]
    pub struct GSite {
        pub site: RefCell<Option<spectre::Site>>,
        pub is_search: RefCell<bool>,
        pub site_descriptor: RefCell<SiteDescriptor>,
    }
    #[glib::object_subclass]
    impl ObjectSubclass for GSite {
        const NAME: &'static str = "GSite";
        type Type = super::GSite;
        type ParentType = glib::Object;
        fn new() -> Self {
            Self {
                site: RefCell::new(None),
                is_search: RefCell::new(false),
                site_descriptor: RefCell::new(SiteDescriptor::default()),
            }
        }
    }
    impl ObjectImpl for GSite {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    // glib::ParamSpec::new_string(
                    //     "user-id",
                    //     "User id",
                    //     "The user id of this user",
                    //     None,
                    //     glib::ParamFlags::READWRITE | glib::ParamFlags::CONSTRUCT_ONLY,
                    // ),
                    // glib::ParamSpec::new_string(
                    //     "display-name",
                    //     "Display Name",
                    //     "The display name of the user",
                    //     None,
                    //     glib::ParamFlags::READWRITE,
                    // ),
                    // glib::ParamSpec::new_object(
                    //     "avatar",
                    //     "Avatar",
                    //     "The avatar of this user",
                    //     gio::LoadableIcon::static_type(),
                    //     glib::ParamFlags::READABLE,
                    // ),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &glib::Value, p_spec: &glib::ParamSpec) {
            match p_spec.name() {
                "site-name" => {
                    // let user_id = value.get().unwrap();
                    // self.user_id.set(user_id).unwrap();
                    println!("Need to add site name")
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, obj: &Self::Type, _id: usize, p_spec: &glib::ParamSpec) -> glib::Value {
            match p_spec.name() {
                // "display-name" => obj.display_name().to_value(),
                // "user-id" => self.user_id.get().to_value(),
                // "avatar" => self.avatar.borrow().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

glib::wrapper! {
    pub struct GSite(ObjectSubclass<imp::GSite>);
}

impl GSite {
    pub fn descriptor(&self) -> SiteDescriptor {
        let self_ = imp::GSite::from_instance(&self);
        self_.site_descriptor.borrow().clone()
    }
    pub fn set_descriptor_name(&self, name: &str) {
        let self_ = imp::GSite::from_instance(&self);
        *self_.site_descriptor.borrow().site_name.borrow_mut() = name.to_owned();
    }
    pub fn set_descriptor_version(&self, version: spectre::AlgorithmVersion) {
        self.set_descriptor(SiteDescriptor {
            site_name: RefCell::new(self.descriptor().site_name.borrow().to_owned()),
            result_type: self.descriptor().result_type,
            algorithm_version: version,
        });
    }
    pub fn set_descriptor_type(&self, p_type: spectre::ResultType) {
        self.set_descriptor(SiteDescriptor {
            site_name: RefCell::new(self.descriptor().site_name.borrow().to_owned()),
            result_type: p_type,
            algorithm_version: self.descriptor().algorithm_version,
        });
    }
    pub fn descriptor_name(&self) -> String {
        self.descriptor().site_name.borrow().clone()
    }
    pub fn set_descriptor(&self, descriptor: SiteDescriptor) {
        let self_ = imp::GSite::from_instance(&self);
        *self_.site_descriptor.borrow_mut() = descriptor;
    }
    pub fn site(&self) -> Option<spectre::Site> {
        let self_ = imp::GSite::from_instance(&self);
        *self_.site.borrow()
    }
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create GSite")
    }
    pub fn new_with_site(site: &spectre::Site) -> Self {
        let s = GSite::new();
        s.set_site(site);
        s
    }
    pub fn name(&self) -> String {
        let s = self.site();
        s.unwrap().get_name()
    }
    pub fn new_search() -> Self {
        let s = GSite::new();
        let self_ = imp::GSite::from_instance(&s);
        *self_.is_search.borrow_mut() = true;
        s
    }
    pub fn is_search(&self) -> bool {
        let is_search = *imp::GSite::from_instance(&self).is_search.borrow();
        is_search
    }
    pub fn set_site(&self, new_site: &spectre::Site) {
        let self_ = imp::GSite::from_instance(&self);
        self_.site.replace(Some(*new_site));
        self.set_descriptor_name(&new_site.get_name());
        self.set_descriptor_version(new_site.get_algorithm());
        self.set_descriptor_type(new_site.get_resultType());
        println!("{:?}", self.descriptor());
    }
    pub fn get_password(&self, key: spectre::UserKey) -> String {
        let d = self.descriptor();
        if d.site_name.borrow().len() > 0 {
            println!("Generated pwd with version: V{:?}", d.algorithm_version as i32);
            let res = spectre::site_result(&d.site_name.borrow(), key, d.result_type, d.algorithm_version);
            res
        } else {
            String::from("")
        }
    }
}
