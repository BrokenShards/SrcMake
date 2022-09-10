// paths.rs //

use std::{env, io, path::PathBuf};

fn get_exe_path() -> io::Result<PathBuf> { env::current_exe() }

fn get_exe_dir() -> io::Result<PathBuf>
{
	let mut path = match get_exe_path()
	{
		Ok(buf) => buf,
		Err(e) =>
		{
			println!("Failed getting exe directory path: {e}");
			return Err(e);
		}
	};

	path.pop();
	Ok(path)
}

pub fn get_template_dir() -> io::Result<PathBuf>
{
	let mut path = match get_exe_dir()
	{
		Ok(buf) => buf,
		Err(e) =>
		{
			println!("Failed getting exe directory path: {e}");
			return Err(e);
		}
	};

	path.push("templates");
	Ok(path)
}
