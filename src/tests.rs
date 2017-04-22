// Cross language i64 to String conversion utility
//
// Copyright (c) 2017 est31 <MTest31@outlook.com>
// and contributors. All rights reserved.
// Licensed under MIT license, or Apache 2 license,
// at your option. Please see the LICENSE file
// attached to this source distribution for details.

use super::*;
use Language::*;

#[test]
fn test_lib() {
	assert_eq!(convert(English, 42), "Forty-Two");
	assert_eq!(convert(French, 42), "quarante-deux");
	assert_eq!(convert(Chinese, 42), "四十二");
}

#[test]
#[cfg(feature = "isolang")]
fn test_lang_conversion() {
	use Language::*;
	use isolang::Language as IsoLang;
	let fr = IsoLang::from_639_1("fr").unwrap();
	assert_eq!(Language::from_iso_language(fr), Some(French));
	let en = IsoLang::from_639_1("en").unwrap();
	assert_eq!(Language::from_iso_language(en), Some(English));
	let zh = IsoLang::from_639_1("zh").unwrap();
	assert_eq!(Language::from_iso_language(zh), Some(Chinese));
}
