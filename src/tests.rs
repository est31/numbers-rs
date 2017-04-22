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
	assert_eq!(convert(English, 42), "fortytwo");
	assert_eq!(convert(French, 42), "quarante-deux");
	assert_eq!(convert(Chinese, 42), "四十二");
}
