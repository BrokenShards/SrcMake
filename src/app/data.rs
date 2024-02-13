// data.rs
//
// Srcmake - A templated source code generator written in Rust.
// Copyright(C) 2024 Michael Furlong.
//
// This program is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation, either version 3
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
// the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with this program.
// If not, see <https://www.gnu.org/licenses/>.
//
use std::env;

use crate::{
	language::{load_languages, Language},
	name::*,
	SMResult,
};

#[derive(Clone, Debug)]
pub struct AppData
{
	pub languages: Vec<Language>,
	pub language: usize,
	pub filetype: String,
	pub name: String,
	pub overwrite: Option<bool>,
	pub author: String,

	pub directory: String,
	pub args: Vec<String>,
}

impl AppData
{
	pub fn new() -> SMResult<Self>
	{
		let langs = load_languages(false)?;

		Ok(Self {
			languages: langs,
			language: Default::default(),
			filetype: Default::default(),
			name: Default::default(),
			overwrite: None,
			author: Default::default(),
			args: Vec::new(),
			directory: format!("{}", env::current_dir().unwrap().display()),
		})
	}

	pub fn get_language(&self) -> Option<&Language>
	{
		if self.language >= self.languages.len()
		{
			None
		}
		else
		{
			Some(&self.languages[self.language])
		}
	}

	pub fn set_language(&mut self, alias: &str) -> bool
	{
		if self.languages.is_empty() || alias.is_empty()
		{
			return false;
		}

		let lo = alias.to_lowercase();

		for (i, lang) in self.languages.iter().enumerate()
		{
			for alias in &lang.aliases
			{
				if alias.to_lowercase() == lo
				{
					self.language = i;
					return true;
				}
			}
		}

		false
	}

	pub fn set_args(&mut self, args: Vec<String>)
	{
		self.args = args.to_vec();

		let alen = self.args.len();
		let mut i = 0;

		while i < alen
		{
			let a = self.args[i].to_lowercase();

			if a == "--o" || a == "--overwrite"
			{
				self.overwrite = Some(true);
			}
			else if a == "--no" || a == "--no_overwrite"
			{
				self.overwrite = Some(false);
			}
			else if (a == "--au" || a == "--author") && i + 1 < alen
			{
				self.author = self.args[i + 1].clone();
			}

			i += 1;
		}
	}

	pub fn valid(&self) -> bool
	{
		is_valid_file_path(&self.name) && self.language < self.languages.len()
	}
}
