// version.rs
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
use std::fmt::Display;

#[derive(Default, Eq, PartialEq)]
pub struct Version
{
	pub major: u32,
	pub minor: u32,
	pub patch: u32,
	pub build: u32,
}
impl Display for Version
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!(
			f,
			"{}.{}.{}.{}",
			self.major, self.minor, self.patch, self.build
		)
	}
}
impl Version
{
	pub const fn new(major: u32, minor: u32, patch: u32, build: u32) -> Self
	{
		Self {
			major,
			minor,
			patch,
			build,
		}
	}
}
