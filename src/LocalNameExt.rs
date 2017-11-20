// This file is part of html5ever_ext. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of html5ever_ext. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT.


/// Additional helpers to make LocalName more pleasant to work with
pub trait LocalNameExt: Sized
{
	/// Turns a LocalName into a Local QualName
	#[inline(always)]
	fn qual_name(self) -> QualName;
	
	/// Turns a LocalName into an Attribute with a Local QualName and value
	#[inline(always)]
	fn attribute(self, value: &str) -> Attribute;
	
	/// Turns a LocalName into an Attribute with a Local QualName and no value
	#[inline(always)]
	fn empty_attribute(self) -> Attribute
	{
		self.attribute("")
	}
	
	/// Turns a LocalName into an empty UnattachedNode
	#[inline(always)]
	fn empty_node(self) -> UnattachedNode;
	
	/// Turns a LocalName into an UnattachedNode with attributes
	#[inline(always)]
	fn with_attributes(self, attributes: Vec<Attribute>) -> UnattachedNode;
	
	/// Turns a LocalName into an UnattachedNode with attributes and text
	#[inline(always)]
	fn with_attributes_and_text<S: Into<String>>(self, attributes: Vec<Attribute>, text: S) -> UnattachedNode;
	
	/// Turns a LocalName into an UnattachedNode with text
	#[inline(always)]
	fn with_text<S: Into<String>>(self, text: S) -> UnattachedNode;
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
	
	#[inline(always)]
	fn empty_node(self) -> UnattachedNode
	{
		UnattachedNode::empty(self)
	}
	
	#[inline(always)]
	fn with_attributes(self, attributes: Vec<Attribute>) -> UnattachedNode
	{
		UnattachedNode::with_attributes(self, attributes)
	}
	
	#[inline(always)]
	fn with_attributes_and_text<S: Into<String>>(self, attributes: Vec<Attribute>, text: S) -> UnattachedNode
	{
		UnattachedNode::with_attributes_and_text(self, attributes, text)
	}
	
	#[inline(always)]
	fn with_text<S: Into<String>>(self, text: S) -> UnattachedNode
	{
		UnattachedNode::with_text(self, text)
	}
}
