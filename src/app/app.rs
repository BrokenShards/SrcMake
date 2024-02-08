// app.rs
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

use crate::app::{version::Version, *};
use crate::error::{box_error, SMResult};
use crate::language::{language_index, load_languages};
use crate::name::*;

pub const VERSION: Version = Version::new(0, 2, 0, 0);

fn process_args(args: Vec<String>) -> SMResult<Option<AppData>>
{
	let args = if args.len() < 2
	{
		vec![]
	}
	else
	{
		args[1..].to_vec()
	};

	if args.is_empty()
	{
		print_usage();
		return Ok(None);
	}

	let help = {
		let a = args[0].to_lowercase();
		a == "-h" || a == "-help"
	};

	let alen = args.len();

	if alen == 1
	{
		let a = args[0].to_lowercase();

		if help
		{
			print_help();
		}
		else if a == "-v" || a == "-version"
		{
			print_version();
		}
<<<<<<< Updated upstream
=======
		else if a == "-p" || a == "-path"
		{
			match add_to_path()
			{
				Err(e) => return Err(box_error(&format!("{e}"))),
				_ => println!("Srcmake was successfully added to the system PATH."),
			}
		}
>>>>>>> Stashed changes
		else
		{
			return Err(box_error(
				"Invalid argument(s). Run `>srcmake -h` for help.",
			));
		}

		return Ok(None);
	}
	if alen == 2
	{
		if !help
		{
			return Err(box_error(
				"Invalid argument(s). Run `>srcmake -h` for help.",
			));
		}

		let langs = load_languages(true);
		let lstr = args[1].to_lowercase();

		if lstr == "--all"
		{
			for lang in &langs
			{
				println!("{} usage:", &lang.name);
				lang.print_help();
				println!();
			}

			return Ok(None);
		}

		let index = language_index(&lstr, &langs);

		if index >= langs.len()
		{
			return Err(box_error(&format!(
				"Unable to get help for {}; no language found with that alias.",
				&args[1]
			)));
		}

		langs[index].print_help();
		return Ok(None);
	}

	let mut data = AppData::default();

	if !data.set_language(&args[0])
	{
		return Err(box_error(&format!(
			"{} is not a valid language alias.",
			&args[0]
		)));
	}

	data.filetype = args[1].to_lowercase();

	data.name = match is_valid_file_path(&args[2])
	{
		true => args[2].clone(),
		false =>
		{
			return Err(box_error("Invalid name given."));
		}
	};

	data.set_args(
		if args.len() > 3
		{
			(&args[3..]).to_vec()
		}
		else
		{
			vec![]
		},
	);

	Ok(Some(data))
}

pub fn run_srcmake() -> SMResult<()>
{
	let data = match process_args(env::args().collect())?
	{
		Some(d) => d,
		_ => return Ok(()),
	};

	generate_files(&data)
}
