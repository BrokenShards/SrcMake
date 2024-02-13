// envpath.rs //

use crate::box_error;
use crate::paths;
use crate::SMResult;

#[cfg(target_os = "linux")]
const FILENAME: &str = "/etc/profile.d/500_srcmake_to_path.sh";
#[cfg(target_os = "macos")]
const FILENAME: &str = "/etc/paths";

#[cfg(target_os = "windows")]
pub fn add_to_path() -> SMResult<()>
{
	use winreg::{
		enums::{HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS},
		RegKey,
	};

	let reg_key = {
		match RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(
			"SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
			KEY_ALL_ACCESS,
		)
		{
			Ok(k) => k,
			Err(e) =>
			{
				return Err(box_error(&format!(
					"Unable to get read/write access the Windows registry: {e}. Try running the \
					 command again with administrative privileges.",
				)));
			}
		}
	};

	let exedir = paths::executable_dir();

	let path: String = match reg_key.get_value("PATH")
	{
		Ok(p) => p,
		Err(e) =>
		{
			if reg_key.set_value("PATH", &exedir).is_err()
			{
				return Err(box_error(&format!(
					"Failed adding the PATH system environment variable: {e}.",
				)));
			}
			return Ok(());
		}
	};

	if path.to_lowercase().find(&exedir.to_lowercase()).is_some()
	{
		println!("Srcmake is already in the system PATH.");
		return Ok(());
	}

	let path = (if !path.ends_with(';')
	{
		path + ";"
	}
	else
	{
		path
	} + "\"" + &exedir.replace("/", "\\")
		+ "\";");

	if let Err(e) = reg_key.set_value("PATH", &path)
	{
		return Err(box_error(&format!(
			"Failed setting the environment path: {e}."
		)));
	}

	println!("Srcmake was successfully added to the system PATH.");
	Ok(())
}
#[cfg(target_os = "windows")]
pub fn remove_from_path() -> SMResult<()>
{
	use winreg::{
		enums::{HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS},
		RegKey,
	};

	let reg_key = {
		match RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(
			"SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment",
			KEY_ALL_ACCESS,
		)
		{
			Ok(k) => k,
			Err(e) =>
			{
				return Err(box_error(&format!(
					"Unable to get read/write access the Windows registry: {e}. Try running the \
					 command again with administrative privileges.",
				)));
			}
		}
	};

	let exedir = paths::executable_dir();

	let path: String = match reg_key.get_value("PATH")
	{
		Ok(p) => p,
		Err(_) => return Ok(())
	};

	let index = path.to_lowercase().find(&exedir.to_lowercase());

	if let Some(i) = index
	{
		let path = ( path[..i].to_owned() + &path[i + path.len()..] ).replace(";;", ";");

		if let Err(e) = reg_key.set_value("PATH", &path)
		{
			return Err(box_error(&format!(
				"Failed setting the environment path: {e}."
			)));
		}
	}
	else
	{
		return Ok(());
	}

	println!("Srcmake was successfully removed from the system PATH.");
	Ok(())
}

#[cfg(target_os = "linux")]
pub fn add_to_path() -> SMResult<()>
{
	use std::env;
	use std::fs;

	let exedir = {
		let mut e = paths::executable_dir();
		e.pop();
		e
	};

	match env::var("PATH")
	{
		Ok(p) =>
		{
			let index = p.to_lowercase().find(&exedir.to_lowercase());

			if index.is_some()
			{
				println!("Srcmake is already in the user PATH.");
				return Ok(());
			}
		}
		Err(e) =>
		{
			return Err(box_error(&format!(
				"Cannot add Srcmake to PATH, unable to access the PATH environment variable: {e}."
			)))
		}
	};

	let pathstr = "export PATH=\"$PATH:".to_owned() + &exedir + "\"";
	
	if let Err(e) = fs::write(&FILENAME, pathstr)
	{
		return Err(box_error(&format!(
			"Cannot add Srcmake to PATH, unable to write file {FILENAME}: {e}."
		)))
	}

	println!("Srcmake was successfully added to the system PATH in {FILENAME}.");
	Ok(())
}
#[cfg(target_os = "linux")]
pub fn remove_from_path() -> SMResult<()>
{
	use std::fs;

	match fs::try_exists(&FILENAME)
	{
		Ok(e) =>
		{
			if e
			{
				if let Err(e) = fs::remove_file(FILENAME)
				{
					return Err(box_error(&format!(
						"Unable to remove file {FILENAME}: {e}."
					)))
				}
			}
		},
		Err(e) => return Err(box_error(&format!(
			"Cannot get filesystem access to {FILENAME}: {e}."
		)))
	}

	println!("Srcmake was successfully removed from the system PATH in {FILENAME}.");
	Ok(())
}

#[cfg(target_os = "macos")]
pub fn add_to_path() -> SMResult<()>
{
	use std::env;
	use std::fs;
	use std::fs::OpenOptions;
	use std::io::Write;

	let exedir = {
		let mut e = paths::executable_dir();
		e.pop();
		e
	};

	match env::var("PATH")
	{
		Ok(p) =>
		{
			let index = p.to_lowercase().find(&exedir.to_lowercase());

			if index.is_some()
			{
				println!("Srcmake is already in the system PATH.");
				return Ok(());
			}
		}
		Err(e) =>
		{
			return Err(box_error(&format!(
				"Cannot add Srcmake to PATH, unable to access the PATH environment variable: {e}."
			)))
		}
	};

	let exists = match fs::try_exists(&FILENAME)
	{
		Ok(e) => e,
		_ =>
		{
			return Err(box_error(&format!(
				"Cannot add Srcmake to PATH, unable to access file {FILENAME}."
			)))
		}
	};

	if !exists
	{
		if let Err(e) = fs::write(&FILENAME, exedir)
		{
			return Err(box_error(&format!(
				"Cannot add Srcmake to PATH, {FILENAME} does not exist and cannot be created: \
				 {e}."
			)));
		}

		return Ok(());
	}

	let mut file = match OpenOptions::new().write(true).append(true).open(&FILENAME)
	{
		Ok(f) => f,
		Err(e) =>
		{
			return Err(box_error(&format!(
				"Cannot add Srcmake to PATH, unable to open {FILENAME} with write access: {e}."
			)))
		}
	};

	if let Err(e) = writeln!(file, "\n{exedir}")
	{
		return Err(box_error(&format!(
			"Cannot add Srcmake to PATH in {FILENAME}: {e}."
		)));
	}

	println!("Srcmake was successfully added to the system PATH in {FILENAME}.");
	Ok(())
}
#[cfg(target_os = "macos")]
pub fn remove_from_path() -> SMResult<()>
{
	use std::fs;

	let exists = match fs::try_exists(&FILENAME)
	{
		Ok(e) => e,
		_ =>
		{
			return Err(box_error(&format!(
				"Cannot remove Srcmake from PATH, unable to access file {FILENAME}."
			)))
		}
	};

	if !exists
	{
		return Ok(());
	}

	let filedata = match fs::read_to_string(&FILENAME)
	{
		Ok(f) => f,
		Err(e) =>
		{
			return Err(box_error(&format!(
				"Cannot remove Srcmake from PATH, unable to read {FILENAME}: {e}."
			)))
		}
	};

	let exedir = {
		let mut e = paths::executable_dir();
		e.pop();
		e
	};
	let index = filedata.to_lowercase().find(&exedir.to_lowercase());

	if let Some(i) = index
	{
		let filedata = filedata[..i].to_owned() + &filedata[i + filedata.len()..];

		if let Err(e) = fs::write(&FILENAME, filedata)
		{
			return Err(box_error(&format!(
				"Cannot remove Srcmake from PATH, failed writing to {FILENAME}: {e}."
			)));
		}
	}
	else
	{
		return Ok(());
	}

	println!("Srcmake was successfully removed from the system PATH in {FILENAME}.");
	Ok(())
}
