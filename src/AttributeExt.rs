// This file is part of html5ever_ext. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of html5ever_ext. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT.


/// Additional methods for working with attributes
pub trait AttributeExt: Sized
{
	/// Makes an Attribute with a Local QualName and value
	#[inline(always)]
	fn local(local_name: LocalName, value: &str) -> Self;
	
	/// Makes an empty Attribute with a Local QualName
	#[inline(always)]
	fn empty(local_name: LocalName) -> Self
	{
		Self::local(local_name, "")
	}
}

impl AttributeExt for Attribute
{
	#[inline(always)]
	fn local(local_name: LocalName, value: &str) -> Self
	{
		Self
		{
			name: QualName::local(local_name),
			value: StrTendril::from_slice(value),
		}
	}
}
