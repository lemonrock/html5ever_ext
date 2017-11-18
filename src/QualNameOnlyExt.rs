// This file is part of html5ever_ext. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of html5ever_ext. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT.


/// Additional methods solely for QualName
pub trait QualNameOnlyExt: Sized
{
	/// Produces local-only QualName
	#[inline(always)]
	fn local(local_name: LocalName) -> Self;
}

impl QualNameOnlyExt for QualName
{
	#[inline(always)]
	fn local(local_name: LocalName) -> Self
	{
		QualName::new(None, ns!(), local_name)
	}
}
