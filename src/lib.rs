// Cross language i64 to String conversion utility
//
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors. All rights reserved.
// Licensed under MIT license, or Apache 2 license,
// at your option. Please see the LICENSE file
// attached to this source distribution for details.

pub extern crate french_numbers as french;
pub extern crate english_numbers as english;
pub extern crate chinese_numbers as chinese;

/// A language to convert into
#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Language {
	French,
	English,
	Chinese,
}

/// Creates a string representation of the `i64` given
pub fn convert(lang :Language, val :i64) -> String {
	use Language::*;
	use english::Formatting as EnFormatting;
	match lang {
		French => french::french_number(&val),
		English => english::convert(val, EnFormatting::default()),
		Chinese => chinese::convert_all_fmt(val),
	}
}
