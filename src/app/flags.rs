// filetype.rs //

use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Clone, Hash, Copy)]
#[repr(u8)]
pub enum Language
{
	C = 0,
	Cpp = 1,
	CSharp = 2,
	Rust = 3,
}

impl Display for Language
{
	fn fmt(&self, f: &mut Formatter) -> Result { write!(f, "{:?}", self) }
}

impl Language
{
	pub fn from_string(s: &str) -> Option<Self>
	{
		let s = s.trim().to_lowercase();

		if s == "c"
		{
			Some(Self::C)
		}
		else if s == "cpp" || s == "cplusplus" || s == "c++"
		{
			Some(Self::Cpp)
		}
		else if s == "csharp" || s == "cs" || s == "c#"
		{
			Some(Self::CSharp)
		}
		else if s == "rust"
		{
			Some(Self::Rust)
		}
		else
		{
			None
		}
	}
}

#[derive(Debug, PartialEq, Clone, Hash, Copy)]
#[repr(u8)]
pub enum FileType
{
	// All
	Main = 0,
	Struct = 1,

	// C/C++
	Header = 2,
	Source = 3,

	// C++
	Class = 4,
	ClassHeader = 5,
	ClassSource = 6,
	Singleton = 7,
	SingletonHeader = 8,
	SingletonSource = 9,
	Singleton03 = 10,
	Singleton03Header = 11,
	Singleton03Source = 12,
	Template = 13,
	TemplateHeader = 14,
	TemplateSource = 15,

	// C#
	Interface = 16,
	MonoBehaviour = 17,
	ScriptableObject = 18,

	// Rust
	Lib = 19,
	Trait = 20,
}

impl Display for FileType
{
	fn fmt(&self, f: &mut Formatter) -> Result { write!(f, "{:?}", self) }
}
impl FileType
{
	pub fn from_string(s: &str) -> Option<Self>
	{
		let s = s.trim().to_lowercase();

		if s == "main"
		{
			Some(Self::Main)
		}
		else if s == "header"
		{
			Some(Self::Header)
		}
		else if s == "source"
		{
			Some(Self::Source)
		}
		else if s == "struct"
		{
			Some(Self::Struct)
		}
		else if s == "class"
		{
			Some(Self::Class)
		}
		else if s == "classheader"
		{
			Some(Self::ClassHeader)
		}
		else if s == "classsource"
		{
			Some(Self::ClassSource)
		}
		else if s == "singleton"
		{
			Some(Self::Singleton)
		}
		else if s == "singletonheader"
		{
			Some(Self::SingletonHeader)
		}
		else if s == "singletonsource"
		{
			Some(Self::SingletonSource)
		}
		else if s == "singleton03"
		{
			Some(Self::Singleton03)
		}
		else if s == "singleton03header"
		{
			Some(Self::Singleton03Header)
		}
		else if s == "singleton03source"
		{
			Some(Self::Singleton03Source)
		}
		else if s == "template"
		{
			Some(Self::Template)
		}
		else if s == "templateheader"
		{
			Some(Self::TemplateHeader)
		}
		else if s == "templatesource"
		{
			Some(Self::TemplateSource)
		}
		else if s == "interface"
		{
			Some(Self::Interface)
		}
		else if s == "monobehaviour"
		{
			Some(Self::MonoBehaviour)
		}
		else if s == "scriptableobject"
		{
			Some(Self::ScriptableObject)
		}
		else if s == "lib"
		{
			Some(Self::Lib)
		}
		else if s == "trait"
		{
			Some(Self::Trait)
		}
		else
		{
			None
		}
	}

	pub fn is_header(&self) -> bool
	{
		match &self
		{
			FileType::Header
			| FileType::Struct
			| FileType::ClassHeader
			| FileType::SingletonHeader
			| FileType::Singleton03Header
			| FileType::TemplateHeader => true,
			_ => false,
		}
	}
}

pub fn is_compatible(language: Language, filetype: FileType) -> bool
{
	match language
	{
		Language::C => match filetype
		{
			FileType::Main | FileType::Header | FileType::Source | FileType::Struct => true,
			_ => false,
		},
		Language::Cpp => match filetype
		{
			FileType::Main
			| FileType::Header
			| FileType::Source
			| FileType::Struct
			| FileType::Class
			| FileType::ClassHeader
			| FileType::ClassSource
			| FileType::Singleton
			| FileType::SingletonHeader
			| FileType::SingletonSource
			| FileType::Singleton03
			| FileType::Singleton03Header
			| FileType::Singleton03Source
			| FileType::Template
			| FileType::TemplateHeader
			| FileType::TemplateSource => true,
			_ => false,
		},
		Language::CSharp => match filetype
		{
			FileType::Main
			| FileType::Class
			| FileType::Struct
			| FileType::Interface
			| FileType::Singleton
			| FileType::MonoBehaviour
			| FileType::ScriptableObject => true,
			_ => false,
		},
		Language::Rust => match filetype
		{
			FileType::Main | FileType::Lib | FileType::Struct | FileType::Trait => true,
			_ => false,
		},
	}
}
