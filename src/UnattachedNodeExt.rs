// This file is part of html5ever_ext. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of html5ever_ext. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT.


/// Helper trait to make it easier to turn UnattachedNodes into DOMs and HTML fragments
pub trait UnattachedNodeExt: Sized
{
	/// Helper method to turn a node into a DOM
	fn to_rc_dom(self) -> RcDom;
	
	/// Helper method to turn a node into a HTML fragment
	fn to_html_fragment(self) -> String
	{
		self.to_rc_dom().minify_to_string(false)
	}
}

impl UnattachedNodeExt for Vec<UnattachedNode>
{
	fn to_rc_dom(self) -> RcDom
	{
		let mut rc_dom = RcDom::default();
		for node in self
		{
			node.attach_to_document_node(&mut rc_dom);
		}
		rc_dom
	}
}
