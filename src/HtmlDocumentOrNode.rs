// This file is part of html5ever_ext. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of html5ever_ext. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT.


/// A wrapper type that lets one use either a RcDom or a RcNode in APIs.
pub enum RcDomOrRcNode
{
	Dom(RcDom),
	Node(Rc<Node>),
}

impl Minify for RcDomOrRcNode
{
	#[inline(always)]
	fn debug_fmt<W: fmt::Write>(&self, f: &mut W) -> fmt::Result
	{
		match *self
		{
			RcDomOrRcNode::Dom(ref inner) => inner.debug_fmt(f),
			RcDomOrRcNode::Node(ref inner) => inner.debug_fmt(f),
		}
	}
	
	#[inline(always)]
	fn minify_to_file_path<P: AsRef<Path>>(&self, html_head_and_body_tags_are_optional: bool, collapse_whitespace: bool, html_file_path: P) -> Result<(), HtmlError>
	{
		match *self
		{
			RcDomOrRcNode::Dom(ref inner) => inner.minify_to_file_path(html_head_and_body_tags_are_optional, collapse_whitespace, html_file_path),
			RcDomOrRcNode::Node(ref inner) => inner.minify_to_file_path(html_head_and_body_tags_are_optional, collapse_whitespace, html_file_path),
		}
	}
	
	#[inline(always)]
	fn minify_to_bytes(&self, html_head_and_body_tags_are_optional: bool, collapse_whitespace: bool) -> Vec<u8>
	{
		match *self
		{
			RcDomOrRcNode::Dom(ref inner) => inner.minify_to_bytes(html_head_and_body_tags_are_optional, collapse_whitespace),
			RcDomOrRcNode::Node(ref inner) => inner.minify_to_bytes(html_head_and_body_tags_are_optional, collapse_whitespace),
		}
	}
	
	#[inline(always)]
	fn minify_to_writer<W: Write>(&self, html_head_and_body_tags_are_optional: bool, collapse_whitespace: bool, writer: W) -> io::Result<()>
	{
		match *self
		{
			RcDomOrRcNode::Dom(ref inner) => inner.minify_to_writer(html_head_and_body_tags_are_optional, collapse_whitespace, writer),
			RcDomOrRcNode::Node(ref inner) => inner.minify_to_writer(html_head_and_body_tags_are_optional, collapse_whitespace, writer),
		}
	}
}

impl QualNameExt for RcDomOrRcNode
{
	#[inline(always)]
	fn is_unprefixed_and_html_namespace_or_none(&self) -> bool
	{
		match *self
		{
			RcDomOrRcNode::Dom(ref inner) => inner.is_unprefixed_and_html_namespace_or_none(),
			RcDomOrRcNode::Node(ref inner) => inner.is_unprefixed_and_html_namespace_or_none(),
		}
	}
	
	#[inline(always)]
	fn is_only_local(&self, local_name: &LocalName) -> bool
	{
		match *self
		{
			RcDomOrRcNode::Dom(ref inner) => inner.is_only_local(local_name),
			RcDomOrRcNode::Node(ref inner) => inner.is_only_local(local_name),
		}
	}
	
	#[inline(always)]
	fn is_only_local_of(&self, local_names: &[LocalName]) -> bool
	{
		match *self
		{
			RcDomOrRcNode::Dom(ref inner) => inner.is_only_local_of(local_names),
			RcDomOrRcNode::Node(ref inner) => inner.is_only_local_of(local_names),
		}
	}
	
	#[inline(always)]
	fn can_have_children(&self) -> bool
	{
		match *self
		{
			RcDomOrRcNode::Dom(ref inner) => inner.can_have_children(),
			RcDomOrRcNode::Node(ref inner) => inner.can_have_children(),
		}
	}
	
	#[inline(always)]
	fn text_content_should_be_escaped(&self) -> bool
	{
		match *self
		{
			RcDomOrRcNode::Dom(ref inner) => inner.text_content_should_be_escaped(),
			RcDomOrRcNode::Node(ref inner) => inner.text_content_should_be_escaped(),
		}
	}
	
	#[inline(always)]
	fn can_collapse_whitespace(&self) -> bool
	{
		match *self
		{
			RcDomOrRcNode::Dom(ref inner) => inner.can_collapse_whitespace(),
			RcDomOrRcNode::Node(ref inner) => inner.can_collapse_whitespace(),
		}
	}
}

impl Selectable for RcDomOrRcNode
{
	#[inline]
	fn find_all_matching_child_nodes_depth_first_including_this_one<MatchUser: FnMut(&Rc<Node>) -> bool>(&self, selector: &OurSelector, match_user: &mut MatchUser) -> bool
	{
		match *self
		{
			RcDomOrRcNode::Dom(ref inner) => inner.find_all_matching_child_nodes_depth_first_including_this_one(selector, match_user),
			RcDomOrRcNode::Node(ref inner) => inner.find_all_matching_child_nodes_depth_first_including_this_one(selector, match_user),
		}
	}
	
	#[inline]
	fn matches(&self, selector: &OurSelector) -> bool
	{
		match *self
		{
			RcDomOrRcNode::Dom(ref inner) => inner.matches(selector),
			RcDomOrRcNode::Node(ref inner) => inner.matches(selector),
		}
	}
}
