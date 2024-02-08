// envpath.rs //

use crate::box_error;
use crate::paths;
use crate::SMResult;

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
					"Unable to access the Windows registry: {e}.",
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

	Ok(())
}

#[cfg(target_os = "linux")]
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

	let homevar = match env::var("HOME")
	{
		Ok(h) =>
		{
			if !h.ends_with('/')
			{
				h + "/"
			}
			else
			{
				h
			}
		}
		Err(e) =>
		{
			return Err(box_error(&format!(
				"Cannot add Srcmake to PATH, unable to access the HOME environment variable: {e}."
			)))
		}
	};
	let profiledir = homevar + ".profile";
	let exists = match fs::try_exists(&profiledir)
	{
		Ok(e) => e,
		Err(e) =>
		{
			return Err(box_error(&format!(
				"Cannot add Srcmake to PATH, unable to check if file $HOME/.profile exists: {e}."
			)))
		}
	};

	let pathstr = "export PATH=\"$PATH:".to_owned() + &exedir + "\"";

	if !exists
	{
		if let Err(e) = fs::write(&profiledir, pathstr)
		{
			return Err(box_error(&format!(
				"Cannot add Srcmake to PATH, $HOME/.profile does not exist and cannot be created: \
				 {e}."
			)));
		}

		return Ok(());
	}

	let mut file = match OpenOptions::new()
		.write(true)
		.append(true)
		.open(&profiledir)
	{
		Ok(f) => f,
		Err(e) =>
		{
			return Err(box_error(&format!(
				"Cannot add Srcmake to PATH, unable to open $HOME/.profile with write access: {e}."
			)))
		}
	};

	if let Err(e) = writeln!(file, "\n{pathstr}")
	{
		return Err(box_error(&format!(
			"Cannot add Srcmake to PATH in $HOME/.profile: {e}."
		)));
	}

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

	let pathsfile = "/etc/paths";
	let exists = match fs::try_exists(&pathsfile)
	{
		Ok(e) => e,
		_ =>
		{
			return Err(box_error(&format!(
				"Cannot add Srcmake to PATH, unable to access file {pathsfile}."
			)))
		}
	};

	if !exists
	{
		if let Err(e) = fs::write(&pathsfile, exedir)
		{
			return Err(box_error(&format!(
				"Cannot add Srcmake to PATH, {pathsfile} does not exist and cannot be created: \
				 {e}."
			)));
		}

		return Ok(());
	}

	let mut file = match OpenOptions::new().write(true).append(true).open(&pathsfile)
	{
		Ok(f) => f,
		Err(e) =>
		{
			return Err(box_error(&format!(
				"Cannot add Srcmake to PATH, unable to open {pathsfile} with write access: {e}."
			)))
		}
	};

	if let Err(e) = writeln!(file, "\n{exedir}")
	{
		return Err(box_error(&format!(
			"Cannot add Srcmake to PATH in {pathsfile}: {e}."
		)));
	}

	Ok(())
}
