// paths.rs
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
use std::{env, path::PathBuf};

pub fn executable_dir() -> String
{
	let rootbuf = {
		let mut path = match env::current_exe()
		{
			Ok(buf) => buf,
			Err(_) => PathBuf::new(),
		};

		path.pop();
		path
	};

	let root = unify_separators(&format!("{}", rootbuf.display()));

	if !root.ends_with('/')
	{
		root + "/"
	}
	else
	{
		root
	}
}
pub fn languages_dir() -> String { executable_dir() + "languages/" }
pub fn templates_dir() -> String { executable_dir() + "templates/" }
pub fn scripts_dir() -> String { languages_dir() + "scripts/" }

pub fn has_extention(path: &str) -> bool { path.rfind('.').is_some() }
pub fn get_extention(path: &str) -> &str
{
	match path.rfind('.')
	{
		Some(i) => &path[i + 1..],
		_ => "",
	}
}
pub fn set_extention(path: &str, ext: &str) -> String
{
	let index = match path.rfind('.')
	{
		Some(i) => i,
		_ => return String::from(path),
	};

	String::from(&path[..index + 1]) + ext
}

pub fn get_file_name(path: &str, ext: bool) -> String
{
	if path.is_empty()
	{
		return String::new();
	}

	let mut result = String::from(unify_separators(path));
	let slash = result.rfind("/");

	match slash
	{
		Some(i) =>
		{
			if i + 1 == result.len()
			{
				result = String::new();
			}
			else
			{
				result = String::from(&result[i + 1..]);
			}
		}
		None =>
		{}
	};

	if !ext
	{
		match result.rfind(".")
		{
			Some(i) => result = String::from(&result[..i]),
			None =>
			{}
		};
	}

	result
}
pub fn get_directory(path: &str) -> String
{
	if path.is_empty()
	{
		return String::new();
	}

	let result = unify_separators(path);
	let slash = result.rfind("/");

	match slash
	{
		Some(i) => String::from(&result[..i + 1]),
		None => result,
	}
}

pub fn unify_separators(path: &str) -> String { path.replace('\\', "/") }
