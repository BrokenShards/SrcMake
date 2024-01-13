// error.rs
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
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct SMError
{
	message: String,
}
impl SMError
{
	pub fn new(msg: &str) -> Self
	{
		Self {
			message: String::from(msg),
		}
	}
}
impl fmt::Display for SMError
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", &self.message) }
}
impl Error for SMError {}

pub fn make_error(msg: &str) -> SMError { SMError::new(msg) }
pub fn box_error(msg: &str) -> Box<SMError> { Box::new(make_error(msg)) }

pub type SMResult<T> = Result<T, Box<dyn Error>>;
