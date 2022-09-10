// name.rs //

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

	for c in result.chars()
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

pub fn file_name(path: &str, ext: bool) -> String
{
	if path.is_empty()
	{
		return String::new();
	}

	let mut result = String::from(path.replace("\\", "/"));

	loop
	{
		let slash = result.find("/");

		match slash
		{
			Some(i) =>
			{
				if i + 1 == result.len()
				{
					result = String::new();
					break;
				}

				result = String::from(&result[i + 1..]);
			}
			None => break,
		};
	}

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

	as_valid_name(&file_name(path, false), '_')
}
