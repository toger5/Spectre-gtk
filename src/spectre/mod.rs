// use super::spectrebind;
use std;
use std::ffi::{CStr, CString};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
extern crate num;

pub type SpectreUserKey = spectrebind::SpectreUserKey;

#[repr(u32)]
#[derive(FromPrimitive)]
pub enum AlgorithmVersion {
    /** V0 did math with chars whose signedness was platform-dependent. */
    V0 = spectrebind::SpectreAlgorithmV0,
    /** V1 miscounted the byte-length of multi-byte site names. */
    V1 = spectrebind::SpectreAlgorithmV1,
    /** V2 miscounted the byte-length of multi-byte user names. */
    V2 = spectrebind::SpectreAlgorithmV2,
    /** V3 is the current version. */
    V3 = spectrebind::SpectreAlgorithmV3,
}
pub const AlgorithmVersionDefault: AlgorithmVersion = AlgorithmVersion::V3;
pub const AlgorithmVersionLatest: AlgorithmVersion = AlgorithmVersion::V3;

#[repr(u32)]
#[derive(FromPrimitive)]
pub enum ResultType {
    /** 16: pg^VMAUBk5x3p%HP%i4= */
    TemplateMaximum = spectrebind::SpectreResultTemplateMaximum,
    /** 17: BiroYena8:Kixa */
    TemplateLong = spectrebind::SpectreResultTemplateLong,
    /** 18: BirSuj0- */
    TemplateMedium = spectrebind::SpectreResultTemplateMedium,
    /** 19: Bir8 */
    TemplateShort = spectrebind::SpectreResultTemplateShort,
    /** 20: pO98MoD0 */
    TemplateBasic = spectrebind::SpectreResultTemplateBasic,
    /** 21: 2798 */
    TemplatePIN = spectrebind::SpectreResultTemplatePIN,
    /** 30: birsujano */
    TemplateName = spectrebind::SpectreResultTemplateName,
    /** 31: bir yennoquce fefi */
    TemplatePhrase = spectrebind::SpectreResultTemplatePhrase,

    /** 1056: Custom saved password. */
    StatefulPersonal = spectrebind::SpectreResultStatePersonal,
    /** 2081: Custom saved password that should not be exported from the device. */
    StatefulDevice = spectrebind::SpectreResultStateDevice,
    /** 4160: Derive a unique binary key. */
    DeriveKey = spectrebind::SpectreResultDeriveKey,
}
pub const ResultTypeDefault: ResultType = ResultType::TemplateLong;

pub fn name_for_format(format: u32) -> String {
    let format_name = unsafe { CStr::from_ptr(spectrebind::spectre_format_name(format)) };
    format_name.to_string_lossy().into_owned()
}

pub fn site_result(
    site_name: &str,
    master_key: SpectreUserKey,
    result_type: ResultType,
    algorithm_version: AlgorithmVersion,
) -> String {
    let site_res = unsafe {
        CStr::from_ptr(spectrebind::spectre_site_result(
            &master_key,
            CString::new(site_name).expect("ugh").as_ptr(),
            result_type as u32,
            CString::new("").expect("ugh").as_ptr(),
            1,
            spectrebind::SpectreKeyPurposeAuthentication as u8,
            CString::new("").expect("ugh").as_ptr(),
        ))
    };
    site_res.to_string_lossy().into_owned()
}

pub fn master_key(
    full_name: &str,
    master_password: &str,
    algorithm_version: AlgorithmVersion,
) -> SpectreUserKey {
    let m_key = unsafe {
        spectrebind::spectre_user_key(
            CString::new(full_name).unwrap().as_ptr(),
            CString::new(master_password).unwrap().as_ptr(),
            algorithm_version as u32,
        )
    };
    unsafe { m_key.as_ref().unwrap().clone() }
}
// type Identicon = spectrebind::SpectreIdenticon;
#[repr(u32)]
#[derive(FromPrimitive)]

pub enum IdenticonColor {
    Unset = spectrebind::SpectreIdenticonColorUnset,
    Red = spectrebind::SpectreIdenticonColorRed,
    Green = spectrebind::SpectreIdenticonColorGreen,
    Yellow = spectrebind::SpectreIdenticonColorYellow,
    Blue = spectrebind::SpectreIdenticonColorBlue,
    Magenta = spectrebind::SpectreIdenticonColorMagenta,
    Cyan = spectrebind::SpectreIdenticonColorCyan,
    White = spectrebind::SpectreIdenticonColorMono,
}

pub const IdenticonColorFirst: u32 = spectrebind::SpectreIdenticonColorFirst;
pub const IdenticonColorLast: u32 = spectrebind::SpectreIdenticonColorLast;

impl IdenticonColor {
    pub fn to_color_code(&self) -> &str {
        match self {
            IdenticonColor::Red => return "#bb1111",
            IdenticonColor::Green => return "#11bb11",
            IdenticonColor::Yellow => return "#11bbbb",
            IdenticonColor::Blue => return "#1111bb",
            IdenticonColor::Magenta => return "#bb11bb",
            IdenticonColor::Cyan => return "#11bbbb",
            IdenticonColor::White => return "#bbbbbb",
            IdenticonColor::Unset => return "#000000",
        }
    }
}
pub struct Identicon {
    pub leftArm: char,
    pub body: char,
    pub rightArm: char,
    pub accessory: char,
    pub color: IdenticonColor,
}
impl Identicon {
    pub fn to_string(&self) -> String {
        format!(
            "<span foreground='{}'>{}{}{}{}</span>",
            self.color.to_color_code(),
            self.leftArm,
            self.body,
            self.rightArm,
            self.accessory
        )
    }
}
impl From<spectrebind::SpectreIdenticon> for Identicon {
    fn from(mpidenticon: spectrebind::SpectreIdenticon) -> Self {
        Identicon {
            leftArm: c_char_to_char(mpidenticon.leftArm),
            body: c_char_to_char(mpidenticon.body),
            rightArm: c_char_to_char(mpidenticon.rightArm),
            accessory: c_char_to_char(mpidenticon.accessory),
            color: num::FromPrimitive::from_u32(mpidenticon.color as u32)
                .unwrap_or(IdenticonColor::White),
        }
    }
}
pub fn identicon(full_name: &str, master_password: &str) -> Identicon {
    unsafe {
        Identicon::from(spectrebind::spectre_identicon(
            CString::new(full_name).unwrap().as_ptr(),
            CString::new(master_password).unwrap().as_ptr(),
        ))
    }
}

// Marshalling
// pub struct Site {
//     pub name: String,
//     pub content: String,
//     pub type_: ResultType,
//     pub counter: u32,
//     pub algorithm: AlgorithmVersion,
//     pub loginContent: String,
//     pub loginType: ResultType,
//     pub url: String,
//     pub uses: u32,
//     pub lastUsed: i64,
//     // pub questions_count: usize,
//     // pub questions: *mut SpectreMarshalledQuestion,
// }
// impl Site {
//     pub fn new(from: spectrebind::SpectreMarshalledSite) -> Site {
//         Site {
//             name: c_char_to_string(from.name),
//             content: c_char_to_string(from.content),
//             loginContent: c_char_to_string(from.loginContent),
//             url: c_char_to_string(from.url),
//             type_: num::FromPrimitive::from_u32(from.type_ as u32).unwrap(),
//             counter: from.counter,
//             algorithm: num::FromPrimitive::from_u32(from.algorithm as u32).unwrap(),
//             loginType: num::FromPrimitive::from_u32(from.loginType as u32).unwrap(),
//             uses: from.uses as u32,
//             lastUsed: from.lastUsed,
//         }
//     }
// }
// pub struct SpectreMarshalledSite {
//     #[doc = " Unique name for this site."]
//     pub siteName: *const ::std::os::raw::c_char,
//     #[doc = " Algorithm version to use for all site operations (eg. result, login, question operations)."]
//     pub algorithm: SpectreAlgorithm,
//     #[doc = " The counter value of the site result to generate."]
//     pub counter: SpectreCounter,
//     #[doc = " The result type to use for generating a site result."]
//     pub resultType: SpectreResultType,
//     #[doc = " State data (base64), if any, necessary for generating the site result."]
//     pub resultState: *const ::std::os::raw::c_char,
//     #[doc = " The result type to use for generating a site login."]
//     pub loginType: SpectreResultType,
//     #[doc = " State data (base64), if any, necessary for generating the site login."]
//     pub loginState: *const ::std::os::raw::c_char,
//     #[doc = " Site metadata: URL location where the site can be accessed."]
//     pub url: *const ::std::os::raw::c_char,
//     #[doc = " Site metadata: Amount of times an action has been taken for this site."]
//     pub uses: ::std::os::raw::c_uint,
//     #[doc = " Site metadata: Date of the most recent action taken on this site."]
//     pub lastUsed: time_t,
//     #[doc = " Amount of security questions associated with this site."]
//     pub questions_count: size_t,
//     #[doc = " Array of security questions associated with this site."]
//     pub questions: *mut SpectreMarshalledQuestion,
// }

pub type Site = spectrebind::SpectreMarshalledSite;
impl Site {
    pub fn get_name(&self) -> String {
        c_char_to_string(self.siteName)
    }
    pub fn set_name(&mut self, name: &str) {
        self.siteName = name.as_ptr() as *const i8;
    }
    pub fn get_url(&self) -> String {
        c_char_to_string(self.url)
    }
    pub fn set_url(&mut self, url: &str) {
        self.url = url.as_ptr() as *const i8;
    }
    pub fn increase_uses(&mut self) {
        self.uses += 1;
    }
    pub fn get_uses(&self) -> u32 {
        self.uses
    }
    pub fn set_used_now(&mut self) {
        match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
            Ok(n) => self.lastUsed = n.as_secs() as i64,
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }
    pub fn get_algorithm(&self) -> AlgorithmVersion {
        num::FromPrimitive::from_u32(self.loginType as u32).unwrap()
    }
    pub fn get_loginType(&self) -> ResultType {
        num::FromPrimitive::from_u32(self.loginType as u32).unwrap()
    }
}

fn c_char_to_char(c: *const ::std::os::raw::c_char) -> char {
    unsafe {
        CStr::from_ptr(c)
            .to_string_lossy()
            .into_owned()
            .pop()
            .unwrap_or(' ')
    }
}

pub fn c_char_to_string(c: *const ::std::os::raw::c_char) -> String {
    unsafe { CStr::from_ptr(c).to_string_lossy().into_owned() }
}

pub type User = spectrebind::SpectreMarshalledUser;

impl User {
    pub fn create(
        full_name: &str,
        master_password: &str,
        algorithm_version: AlgorithmVersion,
    ) -> User {
        unsafe {
            *spectrebind::spectre_marshal_user(
                CString::new(full_name).unwrap().as_ptr(),
                spectrebind::spectre_proxy_provider_set_secret(
                    CString::new(master_password).unwrap().as_ptr(),
                ),
                algorithm_version as u32,
            )
        }
    }

    pub fn load_sites_from_file(&mut self) {
        // let mut file = File::create(path)?;
        // self.masterPassword
        // key provider stuff
        let masterpwd: String = "mpw_placeholder".to_string();//c_char_to_string("mpw_placeholder");
                                                            //"mpw_placeholder".to_string(); // c_char_to_string(&"mpw_placeholder");
                                                               // String::from("123");// unsafe{CStr::from_ptr(self.masterPassword)
                                                               // .to_string_lossy()
                                                               // .into_owned()};
        let mut path = dirs::config_dir().unwrap();
        path.push(format!("{}", c_char_to_string(self.userName)));
        path.set_extension("mpsites");
        match marshal_read_from_file(&path, MarshalFormat::flat, masterpwd) {
            Ok(new_user_with_sites) => {
                std::mem::swap(self, new_user_with_sites);
            }
            Err(err) => println!("error while loading sites from file: {}", err),
        }
        // self = &mut user_with_s;
    }

    pub fn get_sites(&self) -> Vec<*mut Site> {
        let mut sites: Vec<*mut Site> = Vec::new();
        // TODO:
        // for i in 0..self.sites_count {
        //     unsafe {
        //         sites.push(self.sites.wrapping_add(usize::from(i)));
        //     }
        // }
        sites
    }
    pub fn add_site(
        &mut self,
        site_name: &str,
        result_type: ResultType,
        site_counter: u32,
        algorithm_version: AlgorithmVersion,
    ) {
        let s: *mut Site;
        let site_name_ptr = CString::new(site_name).unwrap();
        unsafe {
            s = spectrebind::spectre_marshal_site(
                self,
                site_name_ptr.as_ptr(),
                result_type as u32,
                site_counter,
                algorithm_version as u32,
            );
            (*s).set_used_now();
        }
    }
}
// Create a new user object ready for marshalling.
// type Site = spectrebind::SpectreMarshalledSite;
// Create a new site attached to the given user object, ready for marshalling.

// type MarshalFormat = spectrebind::SpectreMarshalFormat;
#[repr(u32)]
pub enum MarshalFormat {
    flat = spectrebind::SpectreFormatFlat,
    json = spectrebind::SpectreFormatJSON,
}

fn marshal_write(out_format: MarshalFormat, mut user: User) -> Result<String, String> {
    let f = out_format as u32;
    let mut error = spectrebind::SpectreMarshalError {
        type_: spectrebind::SpectreMarshalSuccess,
        message: &(0 as ::std::os::raw::c_char),
    };
    unsafe {
        let mut marshalFile = spectrebind::spectre_marshal_file(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        let worked = spectrebind::spectre_marshal_write(f, &mut marshalFile, &mut user) ;
        let mut outbuffer: *mut ::std::os::raw::c_char = 0 as *mut ::std::os::raw::c_char;
        if worked != std::ptr::null() {
            unsafe { Ok(CStr::from_ptr(outbuffer).to_string_lossy().into_owned()) }
        } else {
            unsafe { Err(CStr::from_ptr(error.message).to_string_lossy().into_owned()) }
        }
    }
}
fn marshal_read(
    input_text: String,
    input_format: MarshalFormat,
    masterPassword: String,
) -> Result<*mut User, String> {
    // let mut error = spectrebind::SpectreMarshalError {
    //     type_: spectrebind::SpectreMarshalSuccess,
    //     message: &(0 as ::std::os::raw::c_char),
    // };
    // let mut spectrebind::SpectreMarshalledInfo {
    //     format : SpectreFormatDefault,
    //     redacted:false,
    //     algorithm: SpectreAlgorithmCurrent,
    //     userName: user
    // }
    // let mut marshalFile = spectrebind::spectre_marshal_file(std::ptr::null_mut(), std::ptr::null_mut(),std::ptr::null_mut());
    let mut marshalFile = unsafe {
        spectrebind::spectre_marshal_read(
            std::ptr::null_mut(),
            CString::new(input_text).unwrap().as_ptr(),
        )
    };
    unsafe {
        if marshalFile.as_ref().unwrap().error.type_ == spectrebind::SpectreMarshalSuccess {
            Ok(spectrebind::spectre_marshal_auth(
                marshalFile,
                spectrebind::spectre_proxy_provider_set_secret(
                    CString::new(masterPassword).unwrap().as_ptr(),
                ),
            ))
        } else {
            Err(CStr::from_ptr(marshalFile.as_ref().unwrap().error.message)
                .to_string_lossy()
                .into_owned())
        }
    }
}
pub fn marshal_read_from_file(
    path: &std::path::PathBuf,
    input_format: MarshalFormat,
    masterPassword: String,
) -> std::io::Result<&mut User> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    match marshal_read(contents, input_format, masterPassword) {
        Ok(user) => Ok(unsafe { &mut *user }),
        Err(msg) => Err(std::io::Error::new(std::io::ErrorKind::Other, msg)),
    }
}
pub fn marshal_write_to_file(out_format: MarshalFormat, mut user: User) -> std::io::Result<()> {
    let mut path = dirs::config_dir().unwrap();
    path.push(format!("{}", c_char_to_string(user.userName)));
    path.set_extension("mpsites");
    let mut file = File::create(path)?;
    match marshal_write(out_format, user) {
        Ok(content) => match file.write(content.as_ref()) {
            Ok(n) => Ok(()),
            Err(e) => Err(e),
        },
        Err(msg) => Err(std::io::Error::new(std::io::ErrorKind::Other, msg)),
    }
}

#[allow(warnings)]
mod spectrebind;
