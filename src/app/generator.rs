// generator.rs //

use chrono::{Datelike, Month, Timelike, Utc};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use num_traits::FromPrimitive;

use crate::app::*;
use crate::name::*;
use crate::paths::get_template_dir;

pub fn generate_file(appdata: &AppData) -> Result<(), String>
{
	let mut data: AppData = appdata.clone();

	if !is_valid_file_path(&data.name)
	{
		return Err(String::from(
			"Unable to generate file(s): The given name is not a valid file name or path.",
		));
	}
	if !is_compatible(data.language, data.filetype)
	{
		return Err(String::from(
			"Unable to generate file(s): The given language and filetype flags are incompatible.",
		));
	}
	if !data.valid()
	{
		return Err(String::from(
			"Unable to generate file(s): AppData is invalid.",
		));
	}

	if data.language == Language::Cpp
	{
		match data.filetype
		{
			FileType::Class =>
			{
				data.filetype = FileType::ClassHeader;

				match generate_file(&data)
				{
					Err(e) =>
					{
						return Err(format!(
							"Srcmake was unable to generate class header file {}: {e}.",
							data.name
						));
					}
					_ =>
					{}
				};

				data.filetype = FileType::ClassSource;

				match generate_file(&data)
				{
					Err(e) =>
					{
						return Err(format!(
							"Srcmake was unable to generate class source file {}: {e}.",
							data.name
						));
					}
					_ =>
					{}
				};

				data.filetype = FileType::Class;
				return Ok(());
			}
			FileType::Singleton =>
			{
				data.filetype = FileType::SingletonHeader;

				match generate_file(&data)
				{
					Err(e) =>
					{
						return Err(format!(
							"Srcmake was unable to generate singleton header file {}: {e}.",
							data.name
						));
					}
					_ =>
					{}
				};

				data.filetype = FileType::SingletonSource;

				match generate_file(&data)
				{
					Err(e) =>
					{
						return Err(format!(
							"Srcmake was unable to generate singleton source file {}: {e}.",
							data.name
						));
					}
					_ =>
					{}
				};

				data.filetype = FileType::Singleton;
				return Ok(());
			}
			FileType::Singleton03 =>
			{
				data.filetype = FileType::Singleton03Header;

				match generate_file(&data)
				{
					Err(e) =>
					{
						return Err(format!(
							"Srcmake was unable to generate singleton03 header file {}: {e}.",
							data.name
						));
					}
					_ =>
					{}
				};

				data.filetype = FileType::Singleton03Source;

				match generate_file(&data)
				{
					Err(e) =>
					{
						return Err(format!(
							"Srcmake was unable to generate singleton03 source file {}: {e}.",
							data.name
						));
					}
					_ =>
					{}
				};

				data.filetype = FileType::Singleton03;
				return Ok(());
			}
			FileType::Template =>
			{
				data.filetype = FileType::TemplateHeader;

				match generate_file(&data)
				{
					Err(e) =>
					{
						return Err(format!(
							"Srcmake was unable to generate template header file {}: {e}.",
							data.name
						));
					}
					_ =>
					{}
				};

				data.filetype = FileType::TemplateSource;

				match generate_file(&data)
				{
					Err(e) =>
					{
						return Err(format!(
							"Srcmake was unable to generate template source file {}: {e}.",
							data.name
						));
					}
					_ =>
					{}
				};

				data.filetype = FileType::Template;
				return Ok(());
			}
			_ =>
			{}
		}
	}

	let mut templatepath = match get_template_dir()
	{
		Ok(s) => s,
		Err(e) => return Err(format!("Unable to get template directory: {e}")),
	};

	templatepath.push(format!("{}_{}.src", data.language, data.filetype));

	// Open template file and read in to string.
	let mut content = match fs::read_to_string(&templatepath)
	{
		Ok(con) => con,
		Err(e) =>
		{
			return Err(format!(
				"Unable to read template from file {}: {e}",
				templatepath.display()
			))
		}
	};

	// Replace applicable macros for the language.
	content = match data.language
	{
		Language::C => replace_c_macros(&data, &mut content),
		Language::Cpp => replace_cpp_macros(&data, &mut content),
		Language::CSharp => replace_csharp_macros(&data, &mut content),
		Language::Rust => replace_rust_macros(&data, &mut content),
	};

	let mut nlns = content.find("\n\n\n").is_some() || content.find("\r\n\r\n\r\n").is_some();

	while nlns
	{
		content = content
			.replace("\r\n\r\n\r\n", "\r\n\r\n")
			.replace("\n\n\n", "\n\n");
		nlns = content.find("\n\n\n").is_some() || content.find("\r\n\r\n\r\n").is_some();
	}

	// Save to new file.
	let mut targetpath = PathBuf::new();
	targetpath.push(data.directory);

	let dataname = data.name.replace("\\", "/");
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

	let ext = match data.language
	{
		Language::C =>
		{
			if data.filetype.is_header()
			{
				"h"
			}
			else
			{
				"c"
			}
		}
		Language::Cpp =>
		{
			if data.filetype == FileType::TemplateSource
			{
				"inl"
			}
			else if data.filetype.is_header()
			{
				"hpp"
			}
			else
			{
				"cpp"
			}
		}
		Language::CSharp => "cs",
		Language::Rust => "rs",
	};

	let fname = file_name(&data.name, true);
	targetpath.push(fname + "." + &ext);

	match fs::File::create(&targetpath)
	{
		Ok(mut file) => match file.write_all(content.as_bytes())
		{
			Ok(()) =>
			{}
			Err(e) =>
			{
				return Err(format!(
					"Unable to write file {}: {e}",
					targetpath.display()
				))
			}
		},
		Err(e) =>
		{
			return Err(format!(
				"Unable to create file {}: {e}",
				targetpath.display()
			))
		}
	};

	println!(
		"Srcmake created the file: {} successfully.",
		targetpath.display()
	);

	Ok(())
}

fn include_string(args: &Vec<String>) -> String
{
	let mut includes: Vec<String> = Vec::new();
	let mut ininclude = false;

	for arg in args
	{
		if !ininclude
		{
			let arg = arg.to_lowercase();

			if arg == "--i" || arg == "--include"
			{
				ininclude = true;
			}

			continue;
		}
		else
		{
			if &arg[0..2] == "--"
			{
				ininclude = false;
			}
			else
			{
				includes.push(arg.to_string());
			}
		}
	}

	let mut includestr = String::new();

	if !includes.is_empty()
	{
		for i in includes
		{
			includestr.push_str(&format!("#include <{i}>\n"));
		}

		includestr.pop();
	}

	includestr
}
fn using_string(args: &Vec<String>) -> String
{
	let mut includes: Vec<String> = Vec::new();
	let mut ininclude = false;

	for arg in args
	{
		if !ininclude
		{
			let arg = arg.to_lowercase();

			if arg == "--u" || arg == "--use"
			{
				ininclude = true;
			}

			continue;
		}

		includes.push(arg.to_string());
		ininclude = false;
	}

	let mut includestr = String::new();

	if !includes.is_empty()
	{
		for i in includes
		{
			includestr.push_str(&format!("using {i};\n"));
		}

		includestr.pop();
	}

	includestr
}
fn use_string(args: &Vec<String>) -> String
{
	let mut includes: Vec<String> = Vec::new();
	let mut ininclude = false;

	for arg in args
	{
		if !ininclude
		{
			let arg = arg.to_lowercase();

			if arg == "--u" || arg == "--use"
			{
				ininclude = true;
			}

			continue;
		}

		includes.push(arg.to_string());
		ininclude = false;
	}

	let mut includestr = String::new();

	if !includes.is_empty()
	{
		for i in includes
		{
			includestr.push_str(&format!("use {i};\n"));
		}

		includestr.pop();
	}

	includestr
}
fn namespace_string(args: &Vec<String>) -> String
{
	let mut namespace = String::new();
	let mut innamespace = false;

	for arg in args
	{
		if !innamespace
		{
			let arg = arg.to_lowercase();

			if arg == "--ns" || arg == "--namespace"
			{
				innamespace = true;
			}

			continue;
		}

		namespace = arg.clone();
		break;
	}

	namespace
}
fn header_guard(name: &str, cpp: bool) -> String
{
	let mut result = String::new();

	if !is_valid_name(name)
	{
		return result;
	}

	let mut waslow = false;

	for c in name.chars()
	{
		if waslow && (c.is_uppercase() || (c >= '0' && c <= '9'))
		{
			result.push('_');
		}

		result.push(c.to_ascii_uppercase());
		waslow = c.is_uppercase() || (c >= '0' && c <= '9');
	}

	result.push_str(if cpp { "_HPP" } else { "_H" });
	result
}
fn author(args: &Vec<String>) -> String
{
	let mut author = String::new();
	let mut inauthor = false;

	for arg in args
	{
		if !inauthor
		{
			let arg = arg.to_lowercase();

			if arg == "--au" || arg == "--author"
			{
				inauthor = true;
			}

			continue;
		}

		author = arg.clone();
		inauthor = false;
	}

	author
}

fn replace_universal_macros(data: &AppData, content: &String) -> String
{
	let ext = match data.language
	{
		Language::C =>
		{
			if data.filetype.is_header()
			{
				"h"
			}
			else
			{
				"c"
			}
		}
		Language::Cpp =>
		{
			if data.filetype == FileType::TemplateSource
			{
				"inl"
			}
			else if data.filetype.is_header()
			{
				"hpp"
			}
			else
			{
				"cpp"
			}
		}
		Language::CSharp => "cs",
		Language::Rust => "rs",
	};

	let now = Utc::now();
	let hour = now.hour();
	let year = now.year();
	let mon = Month::from_u32(now.month()).unwrap();

	let safename = path_to_name(&data.name, '_');
	let author = author(&data.args);
	let authorkey = if author.is_empty()
	{
		"$AUTHOR$ "
	}
	else
	{
		"$AUTHOR$"
	};

	content
		.replace("$FILE_NAME$", &format!("{}", file_name(&data.name, true)))
		.replace("$FILE_EXT$", &ext)
		.replace("$NAME$", &safename)
		.replace(&authorkey, &author)
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
fn replace_c_macros(data: &AppData, content: &String) -> String
{
	let cpp = data.language == Language::Cpp;

	let mut result = replace_universal_macros(data, content)
		.replace("$HEADER_EXT$", &(if cpp { "hpp" } else { "h" }))
		.replace("$SOURCE_EXT$", &(if cpp { "cpp" } else { "c" }))
		.replace("$INLINE_EXT$", "inl")
		.replace(
			"$HEADER_GUARD$",
			&header_guard(&data.name, data.language == Language::Cpp),
		);

	let includestr = include_string(&data.args);

	result = if !includestr.is_empty()
	{
		result.replace("$INCLUDES$", &includestr)
	}
	else
	{
		result.replace("$INCLUDES$\n", "").replace("$INCLUDES$", "")
	};

	let namespace = namespace_string(&data.args);

	result = if !namespace.is_empty()
	{
		result
			.replace("$NAMESPACE_BEGIN$", &format!("namespace {namespace}\n{{"))
			.replace("$NAMESPACE_END$", "}")
	}
	else
	{
		result
			.replace("$NAMESPACE_BEGIN$", "")
			.replace("$NAMESPACE_END$", "")
	};

	result
}
fn replace_cpp_macros(data: &AppData, content: &String) -> String
{
	let mut result = replace_c_macros(data, content);

	let mut virt = false;

	for a in data.args.iter()
	{
		if a.to_lowercase() == "--v" || a.to_lowercase() == "--virtual"
		{
			virt = true;
		}
	}

	result = if virt
	{
		result.replace("$VIRTUAL$", "virtual")
	}
	else
	{
		result.replace("$VIRTUAL$ ", "").replace("$VIRTUAL$", "")
	};

	result = result.replace("$HEADER_EXT$", "hpp");
	result
}
fn replace_csharp_macros(data: &AppData, content: &String) -> String
{
	let mut result = replace_universal_macros(data, content);

	let mut virt = false;
	let mut publ = false;
	let mut prv = false;
	let mut prot = false;
	let mut abst = false;
	let mut part = false;
	let mut stat = false;
	let mut seal = false;

	for a in data.args.iter()
	{
		let al = a.to_lowercase();

		if al == "--v" || al == "--virtual"
		{
			virt = true;
		}
		else if al == "--pub" || al == "--public"
		{
			publ = true;
		}
		else if al == "--priv" || al == "--private"
		{
			prv = true;
		}
		else if al == "--prot" || al == "--protected"
		{
			prot = true;
		}
		else if al == "--ab" || al == "--abstract"
		{
			abst = true;
		}
		else if al == "--pt" || al == "--partial"
		{
			part = true;
		}
		else if al == "--st" || al == "--static"
		{
			stat = true;
		}
		else if al == "--sl" || al == "--sealed"
		{
			seal = true;
		}
	}

	if (abst && part)
		|| (abst && stat)
		|| (abst && seal)
		|| (part && stat)
		|| (part && seal)
		|| (stat && seal)
	{
		println!(
			"Only one C# class modifier can be defined at one time. Class modifiers will be \
			 ignored."
		);
		abst = false;
		part = false;
		stat = false;
		seal = false;
	}

	result = if virt
	{
		result.replace("$VIRTUAL$", "virtual")
	}
	else
	{
		result.replace("$VIRTUAL$ ", "").replace("$VIRTUAL$", "")
	};

	result = if abst
	{
		result.replace("$CLASS_MODIFIER$", "abstract")
	}
	else if part
	{
		result.replace("$CLASS_MODIFIER$", "partial")
	}
	else if stat
	{
		result.replace("$CLASS_MODIFIER$", "static")
	}
	else if seal
	{
		result.replace("$CLASS_MODIFIER$", "sealed")
	}
	else
	{
		result
			.replace("$CLASS_MODIFIER$ ", "")
			.replace("$CLASS_MODIFIER$", "")
	};

	result = if publ
	{
		result.replace("$ACCESS$", "public")
	}
	else if prot
	{
		result.replace("$ACCESS$", "protected")
	}
	else if prv
	{
		result.replace("$ACCESS$", "private")
	}
	else
	{
		result.replace("$ACCESS$ ", "").replace("$ACCESS$", "")
	};

	let usingstr = using_string(&data.args);

	result = if !usingstr.is_empty()
	{
		result.replace("$USINGS$", &usingstr)
	}
	else
	{
		result.replace("$USINGS$", "")
	};

	let namespace = namespace_string(&data.args);

	result = if !namespace.is_empty()
	{
		result
			.replace("$NAMESPACE_BEGIN$", &format!("namespace {namespace}\n{{"))
			.replace("$NAMESPACE_END$", "}")
	}
	else
	{
		result
			.replace("$NAMESPACE_BEGIN$", "")
			.replace("$NAMESPACE_END$", "")
	};

	result
}
fn replace_rust_macros(data: &AppData, content: &String) -> String
{
	let mut result = replace_universal_macros(data, content);

	let usestr = use_string(&data.args);

	result = if !usestr.is_empty()
	{
		result.replace("$USES$", &usestr)
	}
	else
	{
		result.replace("$USES$", "")
	};

	result
}
