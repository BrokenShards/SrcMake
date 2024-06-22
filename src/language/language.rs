// language.rs
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
use parsecfg::{Document, KeyValue};
use std::{
	fs,
	path::PathBuf,
	thread::{self, JoinHandle},
};

use crate::{
	box_error,
	language::LanguageHelp,
	make_error,
	paths::{self, get_extention},
	SMError, SMResult,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Language
{
	pub name: String,
	pub template_dir: String,
	pub aliases: Vec<String>,
	pub script_path: String,
	pub help: Option<LanguageHelp>,
}
impl Language
{
	pub fn from_document(doc: &Document, read_help: bool) -> SMResult<Self>
	{
		let section = match doc.get("Language")
		{
			Some(s) => s,
			_ =>
			{
				return Err(box_error(
					"Cannot load Language from document: No Language section.",
				))
			}
		};

		let name = match section.get("name")
		{
			Some(s) => match &s.value
			{
				KeyValue::String(st) => st.clone(),
				_ =>
				{
					return Err(box_error(
						"Cannot load Language from document: 'name' key has wrong value type in \
						 Language section.",
					))
				}
			},
			_ =>
			{
				return Err(box_error(
					"Cannot load Language from document: No 'name' key in Language section.",
				))
			}
		};
		let template_dir = match section.get("template_dir")
		{
			Some(s) => match &s.value
			{
				KeyValue::String(st) => st.clone(),
				_ =>
				{
					return Err(box_error(
						"Cannot load Language from document: 'template_dir' key has wrong value \
						 type in Language section.",
					))
				}
			},
			_ =>
			{
				return Err(box_error(
					"Cannot load Language from document: No 'template_dir' key in Language \
					 section.",
				))
			}
		};
		let aliases = match section.get("aliases")
		{
			Some(s) => match &s.value
			{
				KeyValue::StringArray(a) => a.clone(),
				_ =>
				{
					return Err(box_error(
						"Cannot load Language from document: 'template_dir' key has wrong value \
						 type in Language section.",
					))
				}
			},
			_ =>
			{
				return Err(box_error(
					"Cannot load Language from document: No 'aliases' key in Language section.",
				))
			}
		};
		let script_path = match section.get("script")
		{
			Some(s) => match &s.value
			{
				KeyValue::String(st) => st.clone(),
				_ =>
				{
					return Err(box_error(
						"Cannot load Language from document: 'script' key has wrong value type in \
						 Language section.",
					))
				}
			},
			_ =>
			{
				return Err(box_error(
					"Cannot load Language from document: No 'script' key in Language section.",
				))
			}
		};

		let help = if read_help && doc.contains("Help")
		{
			match LanguageHelp::from_section(doc.get("Help").unwrap())
			{
				Ok(lh) => Some(lh),
				_ => None,
			}
		}
		else
		{
			None
		};

		Ok(Self {
			name,
			template_dir,
			aliases,
			script_path,
			help,
		})
	}
}
impl Language
{
	pub fn new(
		name: &str,
		temps: &str,
		aliases: &[String],
		script: &str,
		help: Option<LanguageHelp>,
	) -> Self
	{
		Self {
			name: name.to_string(),
			template_dir: temps.to_string(),
			aliases: aliases.to_vec(),
			script_path: script.to_string(),
			help,
		}
	}

	pub fn template_directory(&self) -> String { paths::templates_dir() + &self.template_dir }

	pub fn template_paths(&self) -> Vec<PathBuf>
	{
		let mut buf: Vec<PathBuf> = vec![];
		let entries = match fs::read_dir(self.template_directory())
		{
			Ok(d) => d,
			_ => return vec![],
		};

		for entry in entries
		{
			let entry = match entry
			{
				Ok(e) => e,
				_ => continue,
			};
			let meta = match entry.metadata()
			{
				Ok(m) => m,
				_ => continue,
			};

			if meta.is_file()
			{
				buf.push(entry.path());
			}
		}

		buf
	}

	pub fn print_help(&self)
	{
		println!("{} usage:", &self.name);

		println!("Language flag aliases:");

		for alias in &self.aliases
		{
			println!("\t{}", alias);
		}

		if let Some(h) = &self.help
		{
			h.print_help();
		};
	}
}

pub fn load_languages(help: bool) -> SMResult<Vec<Language>>
{
	let lang_dir = paths::languages_dir();

	let mut buf: Vec<Language> = vec![];
	let entries = match fs::read_dir(&lang_dir)
	{
		Ok(d) => d,
		Err(e) =>
		{
			return Err(box_error(&format!(
				"Failed reading from directory {lang_dir}: {e}."
			)));
		}
	};

	let mut tasks: Vec<JoinHandle<Result<Language, SMError>>> = Vec::new();

	for entry in entries
	{
		let entry = match entry
		{
			Ok(e) => e,
			_ => continue,
		};

		match entry.metadata()
		{
			Ok(m) =>
			{
				if !m.is_file()
				{
					continue;
				}
			}
			_ => continue,
		};

		let entrypath = format!("{}", entry.path().display());

		if get_extention(&entrypath).to_lowercase() != "cfg"
		{
			continue;
		}

		tasks.push(thread::spawn(move || {
			let doc = match Document::from_file(&entrypath)
			{
				Ok(d) => d,
				Err(e) =>
				{
					return Err(make_error(&format!(
						"Failed loading cfg document from {}: {e}.",
						&entrypath
					)));
				}
			};

			match Language::from_document(&doc, help)
			{
				Ok(l) => Ok(l),
				Err(e) =>
				{
					return Err(make_error(&format!(
						"Failed loading language from {}: {e}.",
						&entrypath
					)));
				}
			}
		}));
	}

	for task in tasks
	{
		let res = match task.join()
		{
			Ok(r) => r,
			Err(_) => continue,
		};

		if let Ok(l) = res
		{
			buf.push(l);
		}
	}

	Ok(buf)
}
pub fn language_index(lang: &str, languages: &Vec<Language>) -> usize
{
	let lcmp = lang.to_lowercase();

	for (i, l) in languages.iter().enumerate()
	{
		for alias in &l.aliases
		{
			if alias.to_lowercase() == lcmp
			{
				return i;
			}
		}
	}

	return languages.len();
}
