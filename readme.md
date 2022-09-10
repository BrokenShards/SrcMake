## Srcmake
A templated source code generator written in Rust.

### About
Srcmake is a simple application that generates source files from template files. It
currently provides no GUI and supports only C, C++, C# and Rust.

Please note, Srcmake is a personal project that I work on in my spare time and is only
intended for my own use.

### Usage
```
>srcmake [language] [filetype] [name] ([arguments])
```

#### Language Flags
The first required parameter is the language flag; this tells Srcmake which language to
generate files for.

```
c         - Generate C files.
cpp       - Generate C++ files.
cplusplus - Generate C++ files.
c#        - Generate C# files.
csharp    - Generate C# files.
rust      - Generate Rust files.
```

#### Filetype Flags
The second required parameter is the filetype flag; this, in combination with the
language flag, tells Srcmake what kind of file(s) to generate.

```
main   - Generate a main source file.
struct - Generate a struct.

header - Generate a C/C++ header file.
source - Generate a C/C++ source file.

class     - Generate a C++/C# class.
singleton - Generate a C++/C# singleton.

singleton03 - Generate a C++ singleton with C++03 threadsafety.
template    - Generate C++ template class.

interface        - Generate a C# interface.
monobehaviour    - Generate a C# Unity MonoBehaviour class.
scriptableobject - Generate a C# Unity ScriptableObject class.

lib   - Generate a Rust lib file.
trait - Generate a Rust trait.
```

#### Name
The last required parameter is the name; this must contain characters only valid in
a file path. The name is not just used to name the generated file but also classes
and structures where any invalid characters will be replaced by underscores.

#### Arguments
Arguments are optional parameters that can further customise generated code.

List of possible arguments:
```
C Language Arguments
--include   - Interprets the next arguments as files to include until another language argument is met.
--i         - Same as --include.
--namespace - Interprets the next argument as the namespace name.
--ns        - Same as --namespace.

C++ Language Arguments
--include   - Interprets the next arguments as files to include until another language argument is met.
--i         - Same as --include.
--namespace - Interprets the next argument as the namespace name.
--ns        - Same as --namespace.
--virtual   - Make the generated class/structure virtual.
--v         - Same as -virtual.

C# Language Arguments
--namespace - Interprets the next argument as the namespace name.
--ns        - Same as --namespace.
--use       - Interprets the next arguments as C# 'using' statements until another language argument is met.
--u         - Same as --use.
--virtual   - Make the generated class/structure virtual.
--v         - Same as -virtual.

--public    - Makes the generated class/structure public.
--pub       - Same as --public.
--protected - Makes the generated class/structure protected.
--prot      - Same as --protected.
--private   - Makes the generated class/structure private.
--priv      - Same as --private.

--abstract  - Makes generated class/structure abstract.
--ab        - Same as -abstract.
--partial   - Makes generated class/structure partial.
--pt        - Same as -partial.
--static    - Makes generated class/structure static.
--st        - Same as -static.
--sealed    - Makes generated class/structure sealed.
--sl        - Same as -sealed.

Rust Language Arguments
--use       - Interprets the next arguments as Rust 'use' statements until another language argument is met.
--u         - Same as --use.
```

## To Do
- Allow the name flag to contain directories relative to the working directory.
- Do more tests.

## Changelog

### Version 0.0.1
- Initial Release
