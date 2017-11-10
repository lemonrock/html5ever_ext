// This file is part of html5ever_ext. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of html5ever_ext. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT.


#[inline(always)]
fn is_space_character(character: char) -> bool
{
	// "The space characters, for the purposes of this specification, are U+0020 SPACE, "tab" (U+0009, "LF" (U+000A, "FF" (U+000C, and "CR" (U+000D."
	match character
	{
		'\u{0020}' | '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' => true,
		
		_ => false,
	}
}
