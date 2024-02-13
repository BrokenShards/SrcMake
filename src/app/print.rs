// print.rs
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
use crate::app::VERSION;

fn print_commands()
{
	println!("[language] - The Language Flag");
	println!(
		"The first required parameter is the language flag; this takes a language alias, telling \
		 Srcmake which language to generate files for. Language aliases are defined in the config \
		 file for each language.\n"
	);

	println!("[filetype] - The Filetype Flag");
	println!(
		"The second required parameter is the filetype flag; this is the name of the template \
		 file that resides within the template directory of the selected language. If multiple \
		 files exist with the same name, multiple files will be generated.\n"
	);

	println!("[name] - The Name Flag");
	println!(
		"The last required parameter is the name; this must contain only valid file path \
		 characters. The name flag is used to name the generated file and must contain valid \
		 characters for a file path; it is also often used by templates to name types, so any \
		 characters that would be invalid in a type name will be replaced by underscores.\n"
	);

	println!("[arguments] - Arguments");
	println!("Arguments are optional parameters that can further customise generated code.");
}
fn print_arguments()
{
	println!("Universal Arguments:");
	println!(
		"\t--au|--author - Sets the author flag to the following argument unless it starts with a \
		 '-'."
	);
	println!(
		"\t--o|--overwrite - If this argument is given, Srcmake will overwrite destination files \
		 without prompting."
	);
	println!(
		"\t--no|--no-overwrite - If this argument is given, Srcmake will skip generating \
		 destination files without prompting."
	);
}

pub fn print_usage()
{
	println!("Srcmake usage:");
	println!(">srcmake -h|-help ([language])");
	println!(
		"Prints Srcmake help. If a language is specified, help for that language will be \
		 printed.\n"
	);
	println!(">srcmake -v|-version");
	println!("Prints Srcmake version.\n");
	println!(">srcmake -p|-path");
	println!("Adds Srcmake to the system environment PATH.\n");
	println!(">srcmake -rp|-remove-path");
	println!("Removes Srcmake from the system environment PATH.\n");
	println!(">srcmake [language] [filetype] [name] ([arguments])");
	println!("Generates file(s) with the given language, filetype, name, and optional arguments.");
}
pub fn print_help()
{
	println!("Srcmake generates source files from templates.\n");
	print_usage();
	println!();
	print_commands();
	println!();
	print_arguments();
	println!("\nFor example:");
	println!(">srcmake csharp class Penguin --au Bob");
	println!(
		"Would use the template file called class in the C# templates directory to generate a C# \
		 class in the current directory called Penguin and set the author flag to Bob."
	);
}
pub fn print_version()
{
	println!(
		"Srcmake version {}.\nCopyright (c) Michael Furlong 2024.",
		VERSION
	);
}
