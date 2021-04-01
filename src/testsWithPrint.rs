use std::cell::{RefCell, RefMut};

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::mem;
use std::rc::Rc;

use gio::ApplicationCommandLineExt;

extern crate gio;
extern crate gtk;
extern crate libc;

use gio::prelude::*;
use gtk::prelude::*;

use super::mpw;
use gtk::{
    Application, ApplicationWindow, Builder, Button, ButtonsType, DialogFlags, Entry, Label,
    MessageDialog, MessageType, Window,
};
use pango;
use std::time::SystemTime;
#[test]
fn format_converter() {
    //test format converter
    println!("Format for 2: {}",);
    assert_eq!(mpw::name_for_format(2), "JSON".to_owned());
}

fn test() {
    //test MASTER KEY
    // const version: mpw::AlgorithmVersion = mpw::AlgorithmVersion::V3;
    let m_key: mpw::MasterKey = mpw::master_key("aa", "aa", mpw::AlgorithmVersionDefault);
    println!("key: {:#?}", m_key);
    //test PASSWORD
    let pwd = mpw::site_result(
        "matrix",
        m_key,
        mpw::ResultType::TemplateLong,
        mpw::AlgorithmVersion::V3,
    );

    //test Marshal
    let mut user = mpw::User::create("name", "abc", mpw::AlgorithmVersionDefault);
    println!("A");
    user.add_site(
        "abc",
        mpw::ResultType::TemplateLong,
        0,
        mpw::AlgorithmVersionDefault,
    );
    user.add_site(
        "der shit",
        mpw::ResultType::TemplateLong,
        0,
        mpw::AlgorithmVersionDefault,
    );
    user.add_site(
        "altabox",
        mpw::ResultType::TemplateLong,
        0,
        mpw::AlgorithmVersionDefault,
    );
    user.add_site(
        "mpwstgug",
        mpw::ResultType::TemplateLong,
        0,
        mpw::AlgorithmVersionDefault,
    );
    user.add_site(
        "sething.org",
        mpw::ResultType::TemplateLong,
        0,
        mpw::AlgorithmVersionDefault,
    );
    // for s in user.get_sites().iter_mut() {
    //     (*s).lastUsed = SystemTime::now()
    //         .duration_since(SystemTime::UNIX_EPOCH)
    //         .expect("time unwrap error")
    //         .as_secs() as i64;
    //     println!("We have a site: {}, and it has uses {}", s.name, s.uses);
    // }
    // for s in user.get_sites() {
    //     println!("We have a site: {}, and it has uses {}", s.name, s.uses);
    // }

    println!("A");
    mpw::site_result(
        "abc",
        m_key,
        mpw::ResultType::TemplateLong,
        mpw::AlgorithmVersionDefault,
    );
    println!("C");
    // println!(
    //     "some marshal file: \n {}",
    //     mpw::marshal_write(mpw::MarshalFormat::flat, user).expect("marshal_write")
    // );
    match mpw::marshal_write_to_file("TESTmpsites.txt", mpw::MarshalFormat::flat, user) {
        Ok(a) => println!("succsesfully wrote to file"),
        Err(r) => println!("err {}", r),
    }
    println!("B");
    match mpw::marshal_read_from_file(
        "TESTmpsites.txt",
        mpw::MarshalFormat::flat,
        "abc".to_string(),
    ) {
        Ok(usr) => {
            unsafe {
                println!(
                    "user site 1) {}",
                    CStr::from_ptr(usr.fullName).to_string_lossy().into_owned()
                );
                for s in usr.get_sites() {
                    println!(
                        "We have a site: {}, and it has uses {}",
                        (*s).get_name(),
                        (*s).get_uses()
                    );
                }
            };
            //here we have the loaded usr
        }
        Err(r) => {
            println!("Fail {}", r);
        }
    }
}
