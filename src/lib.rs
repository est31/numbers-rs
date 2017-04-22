// Cross language i64 to String conversion utility
//
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors. All rights reserved.
// Licensed under MIT license, or Apache 2 license,
// at your option. Please see the LICENSE file
// attached to this source distribution for details.

//! Convert i64 to String in various languages

pub extern crate french_numbers as french;
pub extern crate english_numbers as english;
pub extern crate chinese_numbers as chinese;
#[cfg(feature = "isolang")]
pub extern crate isolang;

#[cfg(test)]
mod tests;

/// A language to convert into
#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub enum Language {
	French,
	English,
	Chinese,
}

#[cfg(feature = "isolang")]
impl Language {
	pub fn from_iso_language(l :isolang::Language) -> Option<Self> {
		use isolang::Language::*;
		use Language::*;
		match l {
			Fra => Some(French),
			Eng => Some(English),
			Zho => Some(Chinese),
			_ => None,
		}
	}
	pub fn to_iso_language(self) -> isolang::Language {
		use isolang::Language::*;
		use Language::*;
		match self {
			French => Fra,
			English => Eng,
			Chinese => Zho,
		}
	}
}

/// Creates a string representation of the `i64` in the specified language
pub fn convert(lang :Language, val :i64) -> String {
	use Language::*;
	use english::Formatting as EnFormatting;
	match lang {
		French => french::french_number(&val),
		English => english::convert(val, EnFormatting::all()),
		Chinese => chinese::convert_all_fmt(val),
	}
}
