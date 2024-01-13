// name.rs
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
use crate::paths::get_file_name;

pub fn is_valid_file_name(name: &str) -> bool
{
	if name.is_empty()
	{
		return false;
	}

	for c in name.chars()
	{
		if (c as i32) < 32
		{
			return false;
		}

		match c
		{
			'<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' | '$' | '\'' | '&' =>
			{
				return false
			}
			_ => continue,
		};
	}

	true
}
pub fn is_valid_file_path(path: &str) -> bool
{
	if path.is_empty()
	{
		return false;
	}

	let mut p = path.replace("\\", "/");
	p = if p.len() >= 4 && &p[..4] == "//?/"
	{
		String::from(&p[..4])
	}
	else
	{
		path.to_string()
	};

	while !p.is_empty() && &p[0..1] == "/"
	{
		p.remove(0);
	}

	// if name is empty here, it must have been a root path ("/" | "\")
	if p.is_empty()
	{
		return true;
	}

	match p.find(':')
	{
		Some(i) =>
		{
			let fc = p.chars().next().unwrap();

			if i != 1 || ((fc < 'a' || fc > 'z') && (fc < 'A' || fc > 'Z'))
			{
				return false;
			}

			p = if p.len() > 2 && (&p[2..3] == "\\" || &p[2..3] == "/")
			{
				String::from(&p[3..])
			}
			else
			{
				String::from(&p[2..])
			};
		}
		_ =>
		{}
	};

	// if name is empty here, it must have been a root path ("C:" | "D:\")
	if p.is_empty()
	{
		return true;
	}

	let split = p.split("/");
	let vec: Vec<&str> = split.collect();

	for f in vec
	{
		if !is_valid_file_name(f)
		{
			return false;
		}
	}

	true
}

pub fn is_valid_name(name: &str) -> bool
{
	if name.is_empty()
	{
		return false;
	}

	let mut first = true;

	let name = String::from(name).to_lowercase();

	for c in name.chars()
	{
		if first
		{
			if (c < 'a' || c > 'z') && c != '_'
			{
				return false;
			}

			first = false;
		}
		else
		{
			if (c < 'a' || c > 'z') && (c < '0' || c > '9') && c != '_'
			{
				return false;
			}
		}
	}

	true
}

pub fn as_valid_name(name: &str, repl: char) -> String
{
	if name.is_empty()
	{
		return repl.to_string();
	}

	let mut result = String::from(name);

	let mut first = true;
	let mut i: usize = 0;
	let mut indicies: Vec<usize> = Vec::new();

	let lo = result.to_lowercase();

	for c in lo.chars()
	{
		if first
		{
			if (c < 'a' || c > 'z') && c != '_'
			{
				indicies.push(i);
			}

			first = false;
		}
		else
		{
			if (c < 'a' || c > 'z') && (c < '0' || c > '9') && c != '_'
			{
				indicies.push(i);
			}
		}

		i += 1;
	}

	for ind in indicies
	{
		result.remove(ind);
		result.insert(ind, repl);
	}

	result
}

pub fn path_to_name(path: &str, repl: char) -> String
{
	if path.is_empty()
	{
		return repl.to_string();
	}
	if is_valid_name(path)
	{
		return String::from(path);
	}

	as_valid_name(&get_file_name(path, false), '_')
}
