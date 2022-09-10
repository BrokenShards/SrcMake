// data.rs //

use std::env;

use crate::app::*;
use crate::name::*;

#[derive(Clone, Debug)]
pub struct AppData
{
	pub language: Language,
	pub filetype: FileType,
	pub name: String,
	pub directory: String,
	pub args: Vec<String>,
}
impl Default for AppData
{
	fn default() -> Self
	{
		Self {
			language: Language::C,
			filetype: FileType::Header,
			name: "NewFile".to_string(),
			args: Vec::new(),
			directory: format!("{}", env::current_dir().unwrap().display()),
		}
	}
}
impl AppData
{
	pub fn new(
		language: Language,
		filetype: FileType,
		name: &str,
		args: &[String],
		dir: Option<&str>,
	) -> Self
	{
		Self {
			language,
			filetype,
			name: name.to_string(),
			args: args.to_vec(),
			directory: match dir
			{
				Some(s) => String::from(s),
				None => format!("{}", env::current_dir().unwrap().display()),
			},
		}
	}

	pub fn valid(&self) -> bool
	{
		is_valid_file_path(&self.name) && is_compatible(self.language, self.filetype)
	}
}
