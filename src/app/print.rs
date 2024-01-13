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

pub fn print_usage()
{
	println!("Srcmake usage:");
	println!(">srcmake -h|-help ([language])");
	println!(
		"Prints SrcMake help. If a language is specified, help for that language will be \
		 printed.\n"
	);
	println!(">srcmake -v|-version");
	println!("Prints SrcMake version.\n");
	println!(">srcmake [language] [filetype] [name] ([arguments])");
	println!("Runs SrcMake with the given language, filetype, name, and optional arguments.");
}
pub fn print_arguments()
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
pub fn print_help()
{
	println!("SrcMake generates code files based on templates.\n");
	print_usage();
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
