use crate::spectre;
use gtk::glib;
use gtk::subclass::prelude::*;
use std::cell::{RefCell, Ref};

#[derive(Clone)]
pub struct SiteDescriptor{
    pub siteName: RefCell<String>,
    pub resultType: spectre::ResultType,
    pub algorithmVersion: spectre::AlgorithmVersion,
}
impl Default for SiteDescriptor {
    fn default() -> Self {
        Self{
            siteName: RefCell::new("".to_owned()),
            resultType: spectre::ResultTypeDefault,
            algorithmVersion: spectre::AlgorithmVersionDefault
        }
    }
}
impl std::fmt::Debug for SiteDescriptor{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        f.debug_tuple("")
        .field(&self.siteName)
        // .field(&self.latitude)
        .finish()
    }
}
mod imp{
    use super::*;
    use once_cell::sync::{Lazy, OnceCell};
    use std::cell::RefCell;
    #[derive(Debug, Default)]
    pub struct GSite {
       pub site: RefCell<Option<spectre::Site>>,
       pub isSearch: RefCell<bool>,
       pub siteDescriptor: RefCell<SiteDescriptor>
    }
    #[glib::object_subclass]
    impl ObjectSubclass for GSite {
        const NAME: &'static str = "GSite";
        type Type = super::GSite;
        type ParentType = glib::Object;
        fn new() -> Self {
            Self{site: RefCell::new(None), isSearch: RefCell::new(false), siteDescriptor: RefCell::new(SiteDescriptor::default())}
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

        fn set_property(
            &self,
            _obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            match pspec.name() {
                "site-name" => {
                    // let user_id = value.get().unwrap();
                    // self.user_id.set(user_id).unwrap();
                    println!("Need to add site name")
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
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
    pub fn descriptor(&self) -> SiteDescriptor{
        let self_ = imp::GSite::from_instance(&self);
        self_.siteDescriptor.borrow().clone()
    }
    pub fn set_descriptor_name(&self, name: &str){
        let self_ = imp::GSite::from_instance(&self);
        *self_.siteDescriptor.borrow().siteName.borrow_mut() = name.to_owned();
    }
    pub fn descriptor_name(&self) -> String {
        self.descriptor().siteName.borrow().clone()
    }
    pub fn update_descriptor(&self, descriptor: SiteDescriptor){
        let self_ = imp::GSite::from_instance(&self);
        *self_.siteDescriptor.borrow_mut() = descriptor;
    }
    pub fn site(&self)-> Option<spectre::Site> {
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
    pub fn name(&self)-> String {
        let s = self.site();
        s.unwrap().get_name()
    }
    pub fn new_search() -> Self {
        let s = GSite::new();
        let self_ = imp::GSite::from_instance(&s);
        *self_.isSearch.borrow_mut() = true;
        s
    }
    pub fn is_search(&self) -> bool {
        let is_search = *imp::GSite::from_instance(&self).isSearch.borrow();
        is_search
        // *self_.
    }
    pub fn set_site(&self, new_site : &spectre::Site){
        let self_ = imp::GSite::from_instance(&self);
        self_.site.replace(Some(*new_site));
    }
}


























/*
use gtk::{gio, glib, prelude::*, subclass::prelude::*};

use matrix_sdk::{
    events::{room::member::MemberEventContent, StateEvent},
    identifiers::UserId,
    RoomMember,
};

mod imp {
    use super::*;
    use once_cell::sync::{Lazy, OnceCell};
    use std::cell::RefCell;

    #[derive(Debug, Default)]
    pub struct User {
        pub user_id: OnceCell<String>,
        pub display_name: RefCell<Option<String>>,
        pub avatar: RefCell<Option<gio::LoadableIcon>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for User {
        const NAME: &'static str = "User";
        type Type = super::User;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for User {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpec::new_string(
                        "user-id",
                        "User id",
                        "The user id of this user",
                        None,
                        glib::ParamFlags::READWRITE | glib::ParamFlags::CONSTRUCT_ONLY,
                    ),
                    glib::ParamSpec::new_string(
                        "display-name",
                        "Display Name",
                        "The display name of the user",
                        None,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpec::new_object(
                        "avatar",
                        "Avatar",
                        "The avatar of this user",
                        gio::LoadableIcon::static_type(),
                        glib::ParamFlags::READABLE,
                    ),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            _obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            match pspec.name() {
                "user-id" => {
                    let user_id = value.get().unwrap();
                    self.user_id.set(user_id).unwrap();
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "display-name" => obj.display_name().to_value(),
                "user-id" => self.user_id.get().to_value(),
                "avatar" => self.avatar.borrow().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

glib::wrapper! {
    pub struct User(ObjectSubclass<imp::User>);
}

/// This is a `glib::Object` rapresentation of matrix users.
impl User {
    pub fn new(user_id: &UserId) -> Self {
        glib::Object::new(&[("user-id", &user_id.to_string())]).expect("Failed to create User")
    }

    pub fn user_id(&self) -> UserId {
        use std::convert::TryFrom;
        let priv_ = imp::User::from_instance(&self);
        UserId::try_from(priv_.user_id.get().unwrap().as_str()).unwrap()
    }

    pub fn display_name(&self) -> String {
        let priv_ = imp::User::from_instance(&self);

        if let Some(display_name) = priv_.display_name.borrow().to_owned() {
            display_name
        } else {
            priv_
                .user_id
                .get()
                .unwrap()
                .trim_start_matches("@")
                .to_owned()
        }
    }

    /// Update the user based on the the room member state event
    //TODO: create the GLoadableIcon and set `avatar`
    pub fn update_from_room_member(&self, member: &RoomMember) {
        let changed = {
            let priv_ = imp::User::from_instance(&self);
            let user_id = priv_.user_id.get().unwrap();
            if member.user_id().as_str() != user_id {
                return;
            };

            //let content = event.content;
            let display_name = member.display_name().map(|name| name.to_owned());

            let mut current_display_name = priv_.display_name.borrow_mut();
            if *current_display_name != display_name {
                *current_display_name = display_name;
                true
            } else {
                false
            }
        };

        if changed {
            self.notify("display-name");
        }
    }

    /// Update the user based on the the room member state event
    //TODO: create the GLoadableIcon and set `avatar`
    pub fn update_from_member_event(&self, event: &StateEvent<MemberEventContent>) {
        let changed = {
            let priv_ = imp::User::from_instance(&self);
            let user_id = priv_.user_id.get().unwrap();
            if event.sender.as_str() != user_id {
                return;
            };

            let display_name = if let Some(display_name) = &event.content.displayname {
                Some(display_name.to_owned())
            } else {
                event
                    .content
                    .third_party_invite
                    .as_ref()
                    .map(|i| i.display_name.to_owned())
            };

            let mut current_display_name = priv_.display_name.borrow_mut();
            if *current_display_name != display_name {
                *current_display_name = display_name;
                true
            } else {
                false
            }
        };

        if changed {
            self.notify("display-name");
        }
    }
}
*/