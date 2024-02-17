// help.rs
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
use parsecfg::{KeyValue, Section};
use std::fmt::Display;

use crate::{box_error, SMResult};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LanguageArg
{
	pub aliases: Vec<String>,
	pub info: String,
}
impl Display for LanguageArg
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "\t{} - {}", self.aliases.join(" | "), &self.info)
	}
}
impl LanguageArg
{
	pub fn new(aliases: Vec<String>, info: &str) -> Self
	{
		Self {
			aliases,
			info: String::from(info),
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MacroArg
{
	pub name: String,
	pub info: String,
}
impl Display for MacroArg
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(f, "\t{} - {}", &self.name, &self.info)
	}
}
impl MacroArg
{
	pub fn new(name: &str, info: &str) -> Self
	{
		Self {
			name: String::from(name),
			info: String::from(info),
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LanguageHelp
{
	pub arguments: Vec<LanguageArg>,
	pub macros: Vec<MacroArg>,
}
impl LanguageHelp
{
	pub fn from_section(sect: &Section) -> SMResult<Self>
	{
		let mut args: Vec<LanguageArg> = Vec::new();
		let mut macros: Vec<MacroArg> = Vec::new();

		let argarr = if sect.contains("arguments")
		{
			if let KeyValue::StringArray(a) = &sect.get("arguments").unwrap().value
			{
				if a.len() == 0
				{
					vec![]
				}
				else if a.len() % 2 == 1
				{
					return Err(box_error(
						"Cannot create LanguageHelp from section: 'arguments' array contains an \
						 odd number of strings.",
					));
				}
				else
				{
					a.clone()
				}
			}
			else
			{
				return Err(box_error(
					"Cannot create LanguageHelp from section: 'arguments' key value type is not \
					 an array.",
				));
			}
		}
		else
		{
			vec![]
		};
		let macarr = if sect.contains("macros")
		{
			if let KeyValue::StringArray(a) = &sect.get("macros").unwrap().value
			{
				if a.len() == 0
				{
					vec![]
				}
				else if a.len() % 2 == 1
				{
					return Err(box_error(
						"Cannot create LanguageHelp from section: 'macros' array contains an odd \
						 number of strings.",
					));
				}
				else
				{
					a.clone()
				}
			}
			else
			{
				return Err(box_error(
					"Cannot create LanguageHelp from section: 'macros' key value type is not an \
					 array.",
				));
			}
		}
		else
		{
			vec![]
		};

		let mut i = 0usize;
		while i < argarr.len()
		{
			args.push(LanguageArg::new(
				argarr[i]
					.split(',')
					.map(|s| String::from(s.trim()))
					.collect(),
				&argarr[i + 1],
			));
			i += 2;
		}

		i = 0;
		while i < macarr.len()
		{
			macros.push(MacroArg::new(&macarr[i], &macarr[i + 1]));
			i += 2;
		}

		Ok(Self::new(&args, &macros))
	}
}
impl LanguageHelp
{
	pub fn new(args: &[LanguageArg], macros: &[MacroArg]) -> Self
	{
		Self {
			arguments: args.to_vec(),
			macros: macros.to_vec(),
		}
	}

	pub fn print_help(&self)
	{
		println!("Language arguments:");

		for arg in &self.arguments
		{
			println!("\t{}", arg);
		}

		println!("\nLanguage macros:");

		for mac in &self.macros
		{
			println!("\t{}", mac);
		}
	}
}
