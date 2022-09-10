// print.rs //

pub const VERSION_MAJOR: u32 = 0;
pub const VERSION_MINOR: u32 = 1;
pub const VERSION_PATCH: u32 = 0;

pub fn print_welcome()
{
	println!(
		"Srcmake version {}.{}.{}.\nCopyright (c) Michael Furlong 2022.\n",
		VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH
	);
}
pub fn print_usage()
{
	println!("Srcmake usage:");
	println!("\t>srcmake [language] [filetype] [name] ([arguments])\n");
	println!(
		"So `>srcmake cpp header Object -i vector map` would create a C++ header file called \
		 Object in the current working directory that #include <vector> and <map>."
	);
}
pub fn print_language()
{
	println!("Language Flags:");
	println!("\tc         - Generate C files.");
	println!("\tcpp       - Generate C++ files.");
	println!("\tcplusplus - Generate C++ files.");
	println!("\tc#        - Generate C# files.");
	println!("\tcsharp    - Generate C# files.");
	println!("\trust      - Generate Rust files.");
}
pub fn print_filetypes()
{
	println!("\nFiletype Flags:");
	println!("All Languages");
	println!("\tmain   - Generate a main source file.");
	println!("\tstruct - Generate a struct.");

	println!("C/C++");
	println!("\theader - Generate a C/C++ header file.");
	println!("\tsource - Generate a C/C++ source file.");

	println!("C++/C#");
	println!("\tclass     - Generate a C++/C# class.");
	println!("\tsingleton - Generate a C++/C# singleton.");

	println!("C++");
	println!("\tsingleton03 - Generate a C++ singleton with C++03 threadsafety.");
	println!("\ttemplate    - Generate C++ template class.");

	println!("C#");
	println!("\tinterface        - Generate a C# interface.");
	println!("\tmonobehaviour    - Generate a C# Unity MonoBehaviour class.");
	println!("\tscriptableobject - Generate a C# Unity ScriptableObject class.");

	println!("Rust");
	println!("\tlib   - Generate a rust lib file.");
	println!("\ttrait - Generate a rust trait.");
}
pub fn print_arguments()
{
	println!("C Language Arguments:");
	println!(
		"\t--include   - Interprets the next arguments as files to include until another language \
		 argument is met."
	);
	println!("\t--i         - Same as --include.");
	println!("\t--namespace - Interprets the next argument as the namespace name.");
	println!("\t--ns        - Same as --namespace.");

	println!("C++ Language Arguments");
	println!(
		"\t--include   - Interprets the next arguments as files to include until another language \
		 argument is met."
	);
	println!("\t--i         - Same as --include.");
	println!("\t--namespace - Interprets the next argument as the namespace name.");
	println!("\t--ns        - Same as --namespace.");
	println!("\t--virtual   - Make the generated class/structure virtual.");
	println!("\t--v         - Same as -virtual.");

	println!("C# Language Arguments");
	println!("\t--namespace - Interprets the next argument as the namespace name.");
	println!("\t--ns        - Same as --namespace.");
	println!(
		"\t--use       - Interprets the next arguments as C# 'using' statements until another \
		 language argument is met."
	);
	println!("\t--u         - Same as --use.");
	println!("\t--virtual   - Make the generated class/structure virtual.");
	println!("\t--v         - Same as -virtual.");

	println!("\t--public    - Makes the generated class/structure public.");
	println!("\t--pub       - Same as --public.");
	println!("\t--protected - Makes the generated class/structure protected.");
	println!("\t--prot      - Same as --protected.");
	println!("\t--private   - Makes the generated class/structure private.");
	println!("\t--priv      - Same as --private.");

	println!("\t--abstract  - Makes generated class/structure abstract.");
	println!("\t--ab        - Same as -abstract.");
	println!("\t--partial   - Makes generated class/structure partial.");
	println!("\t--pt        - Same as -partial.");
	println!("\t--static    - Makes generated class/structure static.");
	println!("\t--st        - Same as -static.");
	println!("\t--sealed    - Makes generated class/structure sealed.");
	println!("\t--sl        - Same as -sealed.");

	println!("Rust Language Arguments");
	println!(
		"\t--use       - Interprets the next arguments as Rust 'use' statements until another \
		 language argument is met."
	);
	println!("\t--u         - Same as --use.");
}
pub fn print_help()
{
	print_usage();
	print_language();
	print_filetypes();
	print_arguments();

	println!("The name flag must be a valid relative file path to be valid.\n");
}
