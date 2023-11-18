use std::cell::{RefCell, RefMut};

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::mem;
use std::rc::Rc;

use gio::ApplicationCommandLine;
extern crate gtk;
extern crate libc;

use gio::prelude::*;
use gtk::{gio, prelude::*};

use super::spectre;
use gtk::{Application, ApplicationWindow, Builder, Button, ButtonsType, DialogFlags, Entry, Label, MessageDialog, MessageType, Window};
use pango;
use std::time::SystemTime;
#[test]
fn format_converter() {
    //test format converter
    // println!("Format for 2: {}",);
    assert_eq!(spectre::name_for_format(2), "JSON".to_owned());
}

fn test() {
    //test MASTER KEY
    // const version: spectre::AlgorithmVersion = spectre::AlgorithmVersion::V3;
    let m_key: spectre::MasterKey = spectre::master_key("aa", "aa", spectre::ALGORITHM_VERSION_DEFAULT);
    println!("key: {:#?}", m_key);
    //test PASSWORD
    let pwd = spectre::site_result("matrix", m_key, spectre::ResultType::TemplateLong, spectre::AlgorithmVersion::V3);

    //test Marshal
    let mut user = spectre::User::create("name", "abc", spectre::ALGORITHM_VERSION_DEFAULT);
    println!("A");
    user.add_site("abc", spectre::ResultType::TemplateLong, 0, spectre::ALGORITHM_VERSION_DEFAULT);
    user.add_site("der shit", spectre::ResultType::TemplateLong, 0, spectre::ALGORITHM_VERSION_DEFAULT);
    user.add_site("altabox", spectre::ResultType::TemplateLong, 0, spectre::ALGORITHM_VERSION_DEFAULT);
    user.add_site("spectrestgug", spectre::ResultType::TemplateLong, 0, spectre::ALGORITHM_VERSION_DEFAULT);
    user.add_site("sething.org", spectre::ResultType::TemplateLong, 0, spectre::ALGORITHM_VERSION_DEFAULT);
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
    spectre::site_result("abc", m_key, spectre::ResultType::TemplateLong, spectre::ALGORITHM_VERSION_DEFAULT);
    println!("C");
    // println!(
    //     "some marshal file: \n {}",
    //     spectre::marshal_write(spectre::MarshalFormat::flat, user).expect("marshal_write")
    // );
    match spectre::marshal_write_to_file("TESTmpsites.txt", spectre::MarshalFormat::flat, user) {
        Ok(a) => println!("succsesfully wrote to file"),
        Err(r) => println!("err {}", r),
    }
    println!("B");
    match spectre::marshal_read_from_file("TESTmpsites.txt", spectre::MarshalFormat::flat, "abc".to_string()) {
        Ok(usr) => {
            unsafe {
                println!("user site 1) {}", CStr::from_ptr(usr.fullName).to_string_lossy().into_owned());
                for s in usr.get_sites() {
                    println!("We have a site: {}, and it has uses {}", (*s).get_name(), (*s).get_uses());
                }
            };
            //here we have the loaded usr
        }
        Err(r) => {
            println!("Fail {}", r);
        }
    }
}
