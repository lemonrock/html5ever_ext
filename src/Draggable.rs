// This file is part of html5ever_ext. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of html5ever_ext. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT.


/// Valid values of `draggable` global attribute
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Draggable
{
	/// "" / "true"
	Yes,
	
	/// "false"
	No,
	
	/// "auto"
	Automatic,
}

impl Draggable
{
	#[inline(always)]
	fn to_str(&self) -> &'static str
	{
		use self::Draggable::*;
		
		match *self
		{
			Yes => "",
			No => "false",
			Automatic => "auto",
		}
	}
}
