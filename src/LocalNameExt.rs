// This file is part of html5ever_ext. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of html5ever_ext. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT.


/// Additional helpers to make LocalName more pleasant to work with
pub trait LocalNameExt
{
	/// Turns a LocalName into a Local QualName
	#[inline(always)]
	fn qual_name(self) -> QualName;
	
	/// Turns a LocalName into an Attribute with a Local QualName and value
	#[inline(always)]
	fn attribute(self, value: &str) -> Attribute;
}

impl LocalNameExt for LocalName
{
	#[inline(always)]
	fn qual_name(self) -> QualName
	{
		QualName::local(self)
	}
	
	#[inline(always)]
	fn attribute(self, value: &str) -> Attribute
	{
		Attribute::local(self, value)
	}
}
