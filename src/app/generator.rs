// generator.rs
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
use chrono::{Datelike, Month, Timelike, Utc};
use rlua::Lua;
use std::{
	fs,
	io::Write,
	path::PathBuf,
	thread::{self, JoinHandle},
};

use crate::{app::*, box_error, make_error, name::*, paths::*, SMError, SMResult};

fn replace_universal_macros(data: &AppData, content: String, ext: &str) -> String
{
	let now = Utc::now();
	let hour = now.hour();
	let year = now.year();
	let mon = {
		let mut m = Month::January;
		let mut i = 1;

		while i < now.month()
		{
			m = m.succ();
			i += 1;
		}

		m
	};

	let safename = path_to_name(&data.name, '_');
	let author = &data.author;

	content
		.replace(
			"$FILE_NAME$",
			&format!("{}", get_file_name(&data.name, true)),
		)
		.replace("$FILE_EXT$", &ext)
		.replace("$NAME$", &safename)
		.replace("$AUTHOR$", &author)
		.replace(
			"$DATETIME$",
			&format!(
				"{}-{:02}-{:02}: {:02}:{:02}",
				year,
				now.month(),
				now.day(),
				hour,
				now.minute()
			),
		)
		.replace(
			"$DATE$",
			&format!("{}-{:02}-{:02}", year, now.month(), now.day()),
		)
		.replace("$TIME$", &format!("{:02}:{:02}", hour, now.minute()))
		.replace("$YEAR$", &format!("{}", year))
		.replace("$MONTH_NUM$", &format!("{}", now.month()))
		.replace("$MONTH$", &format!("{}", mon.name()))
		.replace("$DAY$", &format!("{}", now.weekday() as u32))
		.replace("$WEEKDAY$", &format!("{:?}", now.weekday()))
}

fn generate_file(appdata: AppData, tf: &str) -> Result<(), SMError>
{
	// Open template file and read in to string.
	let mut content = match fs::read_to_string(&tf)
	{
		Ok(con) => con,
		Err(e) =>
		{
			return Err(make_error(&format!(
				"Unable to read template from file {tf} because: {e}"
			)))
		}
	};

	let fileext = get_extention(&tf);

	if !appdata.languages[appdata.language]
		.script_path
		.trim()
		.is_empty()
	{
		// Get language script and ensure it exists.
		let scriptdata = {
			let mut scriptpath = scripts_dir();

			scriptpath += &appdata.languages[appdata.language].script_path;

			match fs::read_to_string(&scriptpath)
			{
				Ok(con) => con,
				Err(e) =>
				{
					return Err(make_error(&format!(
						"Unable to read language script {scriptpath} because: {e}"
					)))
				}
			}
		};

		// Start lua instance.
		let lua = Lua::new();

		if let Err(e) = lua.context(|lua_ctx| {
			let luaargs = {
				let la = match lua_ctx.create_table()
				{
					Ok(t) => t,
					Err(e) =>
					{
						return Err(format!("Failed creating lua table for SMArguments: {e}"))
					}
				};

				let mut ai = 1;

				for a in &appdata.args
				{
					if la.set(ai, a.clone()).is_err()
					{
						return Err(format!("Failed setting lua language script argument."));
					}

					ai += 1;
				}

				la
			};

			let globals = lua_ctx.globals();

			// Define variables in lua that the scripts use.
			if globals
				.set("SMFileName", get_file_name(&tf, false))
				.is_err()
			{
				return Err(format!("Failed initialising lua variable SMFileName."));
			}
			if globals
				.set("SMSafeName", path_to_name(&appdata.name, '_'))
				.is_err()
			{
				return Err(format!("Failed initialising lua variable SMSafeName."));
			}
			if globals.set("SMArguments", luaargs).is_err()
			{
				return Err(format!("Failed setting lua language script arguments."));
			}

			// Load the script into lua.
			match lua_ctx.load(&scriptdata).exec()
			{
				Ok(_) =>
				{}
				Err(e) => return Err(format!("Failed parsing language script: {e}.")),
			}
			// Ensure script has required functionsand call ProcessArguments function from lua.
			match lua_ctx.load("ReplaceMacro ~= nil").eval::<bool>()
			{
				Ok(f) =>
				{
					if !f
					{
						return Err(format!("Language script missing ReplaceMacro function."));
					}
				}
				Err(e) =>
				{
					return Err(format!("Failed ReplaceMacro check in language script: {e}"));
				}
			}
			match lua_ctx.load("ProcessArguments ~= nil").eval::<bool>()
			{
				Ok(f) =>
				{
					if f && lua_ctx.load("ProcessArguments()").exec().is_err()
					{
						return Err(format!(
							"Failed running ProcessArguments() in language script."
						));
					}
				}
				Err(e) =>
				{
					return Err(format!(
						"Failed ProcessArguments check in language script: {e}"
					))
				}
			}

			// Parse string, calling the language scripts' ReplaceMacro function from lua to replace macros.
			loop
			{
				let mut mac = content.find('$');

				if mac.is_none()
				{
					break;
				}

				let mut replaced = false;

				while mac.is_some() && mac.unwrap() < content.len()
				{
					let begin = mac.unwrap();
					let end = match content[begin + 1..].find('$')
					{
						Some(e) => e + begin + 1,
						_ => break,
					};

					let macstr = String::from(&content[begin..end + 1].to_uppercase());

					if !is_valid_name(&macstr[1..macstr.len() - 1])
					{
						continue;
					}

					let repl = match lua_ctx
						.load(&format!("ReplaceMacro(\"{}\")", &macstr))
						.eval::<String>()
					{
						Ok(f) => f,
						Err(e) =>
						{
							return Err(format!(
								"Failed running ReplaceMacro in language script: {e}"
							))
						}
					};

					if macstr != repl.to_uppercase()
					{
						content = content.replace(&macstr, &repl);
						replaced = true;
					}

					if repl.is_empty()
					{
						if begin == 0
						{
							if &content[..1] == " "
							{
								content.remove(begin);
							}
						}
						else if &content[begin - 1..begin + 1] == "  "
						{
							content.remove(begin);
						}
					}

					mac = content[begin + repl.len()..].find('$');

					if mac.is_some()
					{
						mac = Some(mac.unwrap() + begin + repl.len());
					}
				}

				if !replaced
				{
					break;
				}
			}

			return Ok(());
		})
		{
			return Err(make_error(&format!("Lua context fail: {e}")));
		}
	}

	// Replace built-in macros (do this after the language to allow its script to override default behaviour).
	content = replace_universal_macros(&appdata, content, fileext);
	// Cleanup whitespace.
	let mut nlns = content.find("\n\n\n").is_some() || content.find("\r\n\r\n\r\n").is_some();

	while nlns
	{
		content = content
			.replace("\r\n\r\n\r\n", "\r\n\r\n")
			.replace("\n\n\n", "\n\n");
		nlns = content.find("\n\n\n").is_some() || content.find("\r\n\r\n\r\n").is_some();
	}

	let mut targetpath = PathBuf::new();
	targetpath.push(&appdata.directory);

	let dataname = appdata.name.replace("\\", "/");
	let split: Vec<&str> = dataname.split("/").collect();

	if !split.len() > 1
	{
		for s in split
		{
			targetpath.push(s);
		}

		targetpath.pop();
	}

	_ = fs::create_dir_all(&targetpath);

	let fname = get_file_name(&appdata.name, true);
	targetpath.push(fname + "." + fileext);

	let exists = match fs::try_exists(&targetpath)
	{
		Ok(e) => e,
		_ => false,
	};

	if exists && appdata.overwrite.is_none()
	{
		println!(
			"A file already exists at {}. Would you like to overwrite it? (Y/N)",
			targetpath.display()
		);

		let mut line = String::new();

		if std::io::stdin().read_line(&mut line).is_err()
		{
			return Ok(());
		}

		let answer = line.trim().to_lowercase();

		if answer != "y" && answer != "yes"
		{
			return Ok(());
		}
	}
	else if exists && appdata.overwrite.is_some()
	{
		let ov = appdata.overwrite.unwrap();

		if !ov
		{
			println!(
				"A file already exists at {}. It will not be overwritten.",
				targetpath.display()
			);

			return Ok(());
		}
		else
		{
			println!(
				"A file already exists at {}. It will be overwritten.",
				targetpath.display()
			);
		}
	}

	match fs::File::create(&targetpath)
	{
		Ok(mut file) => match file.write_all(content.as_bytes())
		{
			Ok(()) =>
			{}
			Err(e) =>
			{
				return Err(make_error(&format!(
					"Unable to write file {}: {e}",
					targetpath.display()
				)))
			}
		},
		Err(e) =>
		{
			return Err(make_error(&format!(
				"Unable to create file {}: {e}",
				targetpath.display()
			)))
		}
	};

	println!(
		"Srcmake created the file: {} successfully.",
		targetpath.display()
	);

	Ok(())
}

pub fn generate_files(appdata: &AppData) -> SMResult<()>
{
	if !appdata.valid()
	{
		if !is_valid_file_path(&appdata.name)
		{
			return Err(box_error(
				"Unable to generate file(s): The given name is not a valid file name or path.",
			));
		}

		return Err(box_error("Unable to generate file(s): AppData is invalid."));
	}

	let tfiles = {
		let templates = appdata.languages[appdata.language].template_paths();

		if templates.is_empty()
		{
			return Err(box_error(&format!(
				"Unable to generate file(s): There are no templates for the language {}.",
				&appdata.languages[appdata.language].name
			)));
		}

		let mut tfl: Vec<String> = Vec::new();

		for t in &templates
		{
			let ftostr = match t.file_stem()
			{
				Some(f) => f,
				_ => continue,
			};
			let ftstr = match ftostr.to_str()
			{
				Some(f) => f,
				_ => continue,
			};

			if ftstr.to_lowercase() != appdata.filetype
			{
				continue;
			}

			match t.to_str()
			{
				Some(f) => tfl.push(String::from(f)),
				_ => continue,
			}
		}

		if tfl.is_empty()
		{
			return Err(box_error(&format!(
				"The language {} does not have the template {}.",
				&appdata.languages[appdata.language].name, &appdata.filetype
			)));
		}

		tfl
	};

	let mut ft: Vec<JoinHandle<Result<(), SMError>>> = Vec::new();

	for tf in tfiles
	{
		let ad = appdata.clone();

		ft.push(thread::spawn(move || return generate_file(ad, &tf)));
	}

	for t in ft
	{
		match t.join()
		{
			Ok(_) =>
			{}
			Err(_) =>
			{
				return Err(box_error("Failed generating file."));
			}
		};
	}

	Ok(())
}
