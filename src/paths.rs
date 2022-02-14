use std::path::PathBuf;
use std::env;

pub struct Paths {
	pub prefix: PathBuf
}

impl Paths {
	pub fn new() -> Paths {
		let mut tmp = match env::current_exe() {
			Ok(p) => p,
			Err(e) => panic!("Can't find exe path: {}", e)
		};
		tmp.pop();
		tmp.pop();
		tmp.pop();
		let prefix = tmp.clone();
		println!("Prefix path: {}", prefix.into_os_string().into_string().unwrap());
		return Paths{ prefix: tmp };
	}
}
