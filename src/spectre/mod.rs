// use super::spectrebind;
use std;
use std::ffi::{CStr, CString};
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
extern crate num;

pub type UserKey = spectrebind::SpectreUserKey;
impl Debug for UserKey{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("spectre::UserKey")
        .field("bytes", &self.bytes)
        .field("keyID", &self.keyID.hex)
        .field("algorithm", &self.algorithm)
        .finish()
    }

}
impl Default for UserKey{
    fn default() -> UserKey{
        user_key("","",AlgorithmVersionDefault)
    }
}
#[repr(u32)]
#[derive(FromPrimitive, Clone)]
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
    user_key: UserKey,
    result_type: ResultType,
    algorithm_version: AlgorithmVersion,
) -> String {
    let site_res = unsafe {
        CStr::from_ptr(spectrebind::spectre_site_result(
            &user_key,
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

pub fn user_key(
    full_name: &str,
    user_password: &str,
    algorithm_version: AlgorithmVersion,
) -> UserKey {
    let m_key = unsafe {
        spectrebind::spectre_user_key(
            CString::new(full_name).unwrap().as_ptr(),
            CString::new(user_password).unwrap().as_ptr(),
            algorithm_version as u32,
        )
    };
    unsafe { m_key.as_ref().unwrap().clone() }
}

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
    pub fn to_markup_string(&self) -> String {
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
pub fn identicon(full_name: &str, user_password: &str) -> Identicon {
    unsafe {
        Identicon::from(spectrebind::spectre_identicon(
            CString::new(full_name).unwrap().as_ptr(),
            CString::new(user_password).unwrap().as_ptr(),
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
            .unwrap_or('*')
    }
}

pub fn c_char_to_string(c: *const ::std::os::raw::c_char) -> String {
    unsafe { CStr::from_ptr(c).to_string_lossy().into_owned() }
}

pub type User = spectrebind::SpectreMarshalledUser;

impl User {
    pub fn create(
        full_name: &str,
        user_password: &str,
        algorithm_version: AlgorithmVersion,
    ) -> User {
        let mut u: User;
        unsafe {
            u = *spectrebind::spectre_marshal_user(
                CString::new(full_name).unwrap().as_ptr(),
                spectrebind::spectre_proxy_provider_set_secret(
                    CString::new(user_password).unwrap().as_ptr(),
                ),
                algorithm_version.clone() as u32,
            );
        }
        u.keyID = user_key(full_name, user_password, algorithm_version).keyID;
        u
    }

    pub fn authenticate(
        path: &std::path::PathBuf,
        // input_format: MarshalFormat,
        userSecret: String,
    ) -> Result<User, FileMarshalReadError> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(io_err) => return Err(FileMarshalReadError::File(io_err)),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(io_err) => return Err(FileMarshalReadError::File(io_err)),
        };
        // TODO: input format should not be hardcoded
        match marshal_read_from_string(contents, MarshalFormat::flat, userSecret) {
            Ok(user) => Ok(unsafe { *user }),
            Err(marshal_err) => Err(FileMarshalReadError::Marshal(marshal_err)),
        }
    }

    pub fn get_sites(&self) -> Vec<*mut Site> {
        let mut sites: Vec<*mut Site> = Vec::new();
        for i in 0..self.sites_count {
            unsafe {
                sites.push(self.sites.wrapping_add(i as usize));
            }
        }
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
    
    pub fn has_site(&self, site_name: &String) -> bool {
        for s in self.get_sites() {
            unsafe {
                println!("{:?}",s.as_ref().unwrap().get_name());
                if (*s).get_name() == site_name.clone() {
                    return true;
                }
            }
        }
        false
    }
    
}
impl Default for User {
    fn default() -> User {
        User::create("","",AlgorithmVersionDefault)
    }
}
impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("spectre::User")
         .field("x", &self.userName)
         .field("defaultType", &self.defaultType)
         .field("keyID", &self.keyID.hex)
         .field("algorithm", &self.algorithm)
         .field("identicon", &self.identicon)
         .field("defaultType", &self.defaultType)
         .field("defaultType", &self.defaultType)
         .finish()
    }
}
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
        let worked = spectrebind::spectre_marshal_write(f, &mut marshalFile, &mut user);
        // let mut outbuffer: *mut ::std::os::raw::c_char = 0 as *mut ::std::os::raw::c_char;
        if worked != std::ptr::null() {
            unsafe { Ok(CStr::from_ptr(worked).to_string_lossy().into_owned()) }
        } else {
            unsafe { Err(CStr::from_ptr(error.message).to_string_lossy().into_owned()) }
        }
    }
}
pub enum FileMarshalReadError {
    File(std::io::Error),
    Marshal(spectrebind::SpectreMarshalError),
}

fn marshal_read_from_string(
    input_text: String,
    input_format: MarshalFormat,
    userSecret: String,
) -> Result<*mut User, spectrebind::SpectreMarshalError> {
    let mut marshalFile = unsafe {
        spectrebind::spectre_marshal_read(
            std::ptr::null_mut(),
            CString::new(input_text.into_bytes()).unwrap().as_ptr(),
        )
    };
    let mut usr: *mut User;
    unsafe {
        if marshalFile.as_ref().unwrap().error.type_ != spectrebind::SpectreMarshalSuccess {
            return Err(marshalFile.as_ref().unwrap().error);
        }
        usr = spectrebind::spectre_marshal_auth(
            marshalFile,
            spectrebind::spectre_proxy_provider_set_secret(
                CString::new(userSecret.into_bytes()).unwrap().as_ptr(),
            ),
        );
        if usr.is_null() {
            return Err(marshalFile.as_ref().unwrap().error);
        }
    }
    Ok(usr)
}

pub fn marshal_write_to_file(out_format: MarshalFormat, mut user: User) -> std::io::Result<()> {
    let mut path = crate::config::get_save_path();
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

pub const SpectreMarshalSuccess: spectrebind::SpectreMarshalErrorType =
    spectrebind::SpectreMarshalSuccess;
pub const SpectreMarshalErrorStructure: spectrebind::SpectreMarshalErrorType =
    spectrebind::SpectreMarshalErrorStructure;
pub const SpectreMarshalErrorFormat: spectrebind::SpectreMarshalErrorType =
    spectrebind::SpectreMarshalErrorFormat;
pub const SpectreMarshalErrorMissing: spectrebind::SpectreMarshalErrorType =
    spectrebind::SpectreMarshalErrorMissing;
pub const SpectreMarshalErrorUserSecret: spectrebind::SpectreMarshalErrorType =
    spectrebind::SpectreMarshalErrorUserSecret;
pub const SpectreMarshalErrorIllegal: spectrebind::SpectreMarshalErrorType =
    spectrebind::SpectreMarshalErrorIllegal;
pub const SpectreMarshalErrorInternal: spectrebind::SpectreMarshalErrorType =
    spectrebind::SpectreMarshalErrorInternal;

#[allow(warnings)]
mod spectrebind;
