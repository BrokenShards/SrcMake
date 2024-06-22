## Srcmake
A templated source code generator written in Rust.

### About
Srcmake is an application that generates source files from templates. Srcmake supports C, C++, C#
and Rust out of the box but languages can be added and modified.

Please note, Srcmake is just a personal project that I work on in my spare time. It is only intended
for my own use and is developed as such. If someone else finds Srcmake useful or wants to submit an
issue or pull request, be my guest but the repo may not be checked regularly.

### Usage
```
>srcmake -h|-help ([language])
```
Prints Srcmake usage help. If a language is specified, help for that language will be printed. If
`--all` is specified instead of a language, help will be printed for all supported languages.

```
>srcmake -v|-version
```
Prints Srcmake version.

```
>srcmake -p|-path
```
Adds the Srcmake directory to the PATH system environment variable. Requires admin privileges.

```
>srcmake -rp|-remove-path
```
Removes the Srcmake directory from the PATH system environment variable. Requires admin privileges.

```
>srcmake [language] [filetype] [name] ([arguments])
```

Generates file(s) using the given language, filetype, name, and optional arguments.

#### Language Flags
The first required parameter is the language flag; this takes a language alias, telling Srcmake
which language to generate files for. Language aliases are defined in the languages' config files.

#### Filetype Flags
The second required parameter is the filetype flag; this is the name of the template file(s) that 
resides within the template directory of the selected language. If multiple files exist with the
same name, multiple files will be generated.

#### Name Flag
The last required parameter is the name; this must contain only valid file path characters. The name
flag is used to name the generated file and is used by templates to name types, so any characters
that would be invalid in a type name will be replaced by underscores.

#### Arguments Flag
Arguments are optional parameters that can further customise generated code. Srcmake handles these
arguments itself:
- `--au`|`--author`   - Sets the author flag to the next argument provided unless it starts with a '-'.
- `--o`|`--overwrite` - If this argument is given, Srcmake will overwrite destination files without prompting.
- `--no`|`--no-overwrite` - If this argument is given, Srcmake will skip generating destination files without prompting.

Languages can define their own arguments and are handled by their respective lua scripts.

### Languages and Templates
#### Languages
Languages are described in config (`.cfg`) files residing in the `languages/` directory. If the file
does not have the `.cfg` file extension, it will be ignored by Srcmake. Languages use lua scripts
located in the `languages/scripts/` directory in order to replace language specific macros found in
their templates. Lua scripts can have any file extension.

For more information on Srcmake languages, see `languages/Example.txt`.

##### The Lua Script
Srcmake will create these global lua variables before loading the language script:

- `SMFileName`  - The name as given in the name flag (no directory or file extension).
- `SMSafeName`  - The name flag with all characters that are invalid in a type name replaced with '_'.
- `SMArguments` - The extra arguments passed to srcmake (after the name flag) if any.

Do not declare these global variables in your own scripts, or their values will overwrite those set
by Srcmake. The lua script `SrcmakeDefines.lua` exists for the sole purpose of preventing editor
errors/warnings about undeclared variables when writing language scripts. Do not actually include,
reference or `require` this file in your language script or it will also overwrite the values of
these variables that were set by Srcmake.

When using a language script, Srcmake will first call the `ProcessArguments()` function (if it 
exists) before searching the template file, calling `ReplaceMacro(macro)` to replace each macro
that is found.

For more information on Srcmake language scripts, see `languages/scripts/Example.lua`.

#### Templates
Templates reside in the `templates/` directory. Templates for a particular language must reside in
that languages' subdirectory. For example, C# templates should be placed in `templates/CSharp/`.
Template files are used as a base to create a type of file, its text will be copied to the generated
file with its macros replaced. The file extension of the template file will be used for the
generated file. If multiple files exist with the same name, multiple files will be generated.

##### Macros
Macros are replaced by Srcmake and the language script when generating file(s). Srcmake will
consider any string surrounded by `$` a macro, unless it would not be a valid type name (must start
with a letter or an underscore and can only contain letters, digits and underscores). When
generating a file, first the language script will recursively replace macros, then the built-in
macros will be replaced.

#### Built in Macros
Srcmake provides these built in macros to be used in template files:
- `$FILE_NAME$`  - The name of the file without the directory or file extension.
- `$FILE_EXT$`   - The file extension.
- `$NAME$`       - The name flag with all characters that are invalid in a variable/class name replaced with '_'.
- `$AUTHOR$`     - The file author.
- `$DATETIME$`   - The current date and time.
- `$DATE$`       - The current date.
- `$TIME$`       - The current time.
- `$YEAR$`       - The current year.
- `$MONTH_NUM$`  - The current month number (1-12).
- `$MONTH$`      - The current month name.
- `$DAY$`        - The current day number.
- `$WEEKDAY$`    - The current weekday name.

## To Do

## Changelog

### Version 0.3.0 (WIP)
- Multithreaded language loading and file generation.
- Added arguments to add and remove Srcmake from the system environment PATH.

### Version 0.2.0
- Languages are no longer hard coded and are now defined externally in config files, using lua 
  scripts to handle argument and macro processing. This has added `parsecfg` as a dependency.
- Templates are no longer hard coded. Srcmake searches the languages' template directory, using the
  filetype flag as the file name for the template, if there are multiple files with the same name,
  multiple files will be generated. A generated file will have the same extension as the template.
- Language specific arguments are now handled by the languages' lua script, for argument help, use
  `>srcmake -h [language]` where `[language]` is one of the languages' aliases.
- Added `--o`|`--overwrite` argument to overwrite existing files without prompting the user.
- Added `--no`|`--no-overwrite` argument to skip generating files if they already exist without
  prompting the user.

### Version 0.1.0
- Initial Release
