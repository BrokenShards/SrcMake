// envpath.rs //

use crate::box_error;
use crate::paths;
use crate::SMResult;

#[cfg(target_os = "linux")]
const FILENAME: &str = "/etc/profile.d/500_srcmake_to_path.sh";
#[cfg(target_os = "macos")]
const FILENAME: &str = "/etc/paths.d/500_srcmake_to_path";

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
					"Failed creating the PATH system environment variable: {e}.",
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
			"Failed setting the system PATH: {e}."
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
		Err(_) => return Ok(()),
	};

	let index = path.to_lowercase().find(&exedir.to_lowercase());

	if let Some(i) = index
	{
		let path = (path[..i].to_owned() + &path[i + path.len()..]).replace(";;", ";");

		if let Err(e) = reg_key.set_value("PATH", &path)
		{
			return Err(box_error(&format!(
				"Failed setting the system PATH: {e}."
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

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn add_to_path() -> SMResult<()>
{
	use std::fs;

	let exedir = {
		let mut e = paths::executable_dir();
		e.pop();
		e
	};

	#[cfg(target_os = "linux")]
	let exedir = "export PATH=\"$PATH:".to_owned() + &exedir + "\"";

	if let Err(e) = fs::write(&FILENAME, exedir)
	{
		return Err(box_error(&format!(
			"Cannot add Srcmake to the system PATH, unable to write file {FILENAME}: {e}."
		)));
	}

	println!("Srcmake was successfully added to the system PATH in {FILENAME}.");
	Ok(())
}
#[cfg(any(target_os = "linux", target_os = "macos"))]
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
						"Cannot remove Srcmake from the system PATH, unable to remove file {FILENAME}: {e}."
					)));
				}
			}
		}
		Err(e) =>
		{
			return Err(box_error(&format!(
				"Cannot remove Srcmake from the system PATH, unable to get filesystem access to {FILENAME}: {e}."
			)))
		}
	}

	println!("Srcmake was successfully removed from the system PATH in {FILENAME}.");
	Ok(())
}
