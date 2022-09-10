// app.rs //

use std::env;

use crate::app::*;
use crate::name::*;

fn help_call(args: &Vec<String>) -> bool
{
	if args.len() == 2
	{
		let a = args[1].to_lowercase();

		if a == "--h" || a == "--help"
		{
			return true;
		}
	}

	false
}
fn process_args(args: &Vec<String>) -> Result<AppData, String>
{
	if args.len() < 2
	{
		return Err(String::from("Invalid amount of arguments given."));
	}

	if args.len() < 4
	{
		return Err(String::from("Invalid amount of arguments given."));
	}

	let lang = match Language::from_string(&args[1])
	{
		Some(lang) => lang,
		None =>
		{
			return Err(String::from("Invalid language flag given."));
		}
	};
	let filet = match FileType::from_string(&args[2])
	{
		Some(filet) => filet,
		None =>
		{
			return Err(String::from("Invalid file type flag given."));
		}
	};
	let name = match is_valid_file_path(&args[3])
	{
		true => args[3].clone(),
		false =>
		{
			return Err(String::from("Invalid name given."));
		}
	};

	Ok(AppData::new(
		lang,
		filet,
		&name,
		if args.len() >= 4 { &args[4..] } else { &[] },
		None,
	))
}

pub fn run_srcmake() -> Result<(), String>
{
	print_welcome();

	let args: Vec<String> = env::args().collect();

	if help_call(&args)
	{
		print_help();
		return Ok(());
	}

	let data = match process_args(&args)
	{
		Ok(d) => d,
		Err(e) =>
		{
			return Err(format!("Srcmake failed processing arguments: {e}"));
		}
	};

	generate_file(&data)
}
