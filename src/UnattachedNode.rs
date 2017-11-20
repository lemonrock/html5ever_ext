// This file is part of html5ever_ext. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of html5ever_ext. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT.


/// Represents the structure of nodes unattached to a DOM.
/// Designed to make it easy to create an entire graph of nodes before adding it.
#[derive(Debug, Clone)]
pub struct UnattachedNode
{
	/// Local name of this node
	pub local_name: LocalName,
	
	/// Attributes of this node
	pub attributes: Vec<Attribute>,
	
	/// The children of this node
	pub children: Vec<Either<String, UnattachedNode>>,
}

impl UnattachedNode
{
	/// Represents an empty element, such as <br>
	#[inline(always)]
	pub fn empty(local_name: LocalName) -> Self
	{
		Self::with_attributes(local_name, vec![])
	}
	
	/// Represents an empty element with just attributes, such as <img src="/path/to/source.png">
	#[inline(always)]
	pub fn with_attributes(local_name: LocalName, attributes: Vec<Attribute>) -> Self
	{
		Self
		{
			local_name,
			attributes,
			children: vec![],
		}
	}
	
	/// Represents an empty element with just attributes and text, such as <details open>hello</details>
	#[inline(always)]
	pub fn with_attributes_and_text(local_name: LocalName, attributes: Vec<Attribute>, text: String) -> Self
	{
		Self
		{
			local_name,
			attributes,
			children: vec!
			[
				Left(text),
			],
		}
	}
	
	/// Add an attribute.
	#[inline(always)]
	pub fn add_attribute(&mut self, attribute: Attribute)
	{
		self.attributes.push(attribute);
	}
	
	/// Add a child element.
	#[inline(always)]
	pub fn add_child_element(&mut self, child_element: UnattachedNode)
	{
		self.children.push(Right(child_element));
	}
	
	/// Add a child text. If there is an existing child text element, appends its text to it.
	/// Returns a mutable reference to child_text.
	#[inline(always)]
	pub fn add_child_text(&mut self, child_text: String) -> &mut String
	{
		if self.children.is_empty()
		{
			self.children.push(Left(child_text));
		}
		else
		{
			// Odd design to avoid double-mutable borrow of self.children
			let unmatched = match self.children.last_mut().unwrap().as_mut().left()
			{
				Some(existing_text) =>
				{
					existing_text.push_str(&child_text);
					true
				}
				None => false,
			};
			if unmatched
			{
				self.children.push(Left(child_text));
			}
		}
		self.children.last_mut().unwrap().as_mut().left().unwrap()
	}
	
	/// Attach this node as a child of the document_node (ie as a root node)
	#[inline(always)]
	pub fn attach_to_document_node(self, rc_dom: &mut RcDom) -> Rc<Node>
	{
		let node = rc_dom.append_new_element_to_document_node(self.local_name.qual_name(), self.attributes);
		Self::process_children(rc_dom, self.children, node)
	}
	
	/// Attach this node as a child of parent_node
	#[inline(always)]
	pub fn attach_to_parent_node(self, rc_dom: &mut RcDom, parent_node: &Rc<Node>) -> Rc<Node>
	{
		let node = rc_dom.append_new_element_to_parent_node(parent_node, self.local_name.qual_name(), self.attributes);
		Self::process_children(rc_dom, self.children, node)
	}
	
	/// Attach this node before the sibling_node
	#[inline(always)]
	pub fn attach_to_before_sibling_node(self, rc_dom: &mut RcDom, sibling_node: &Rc<Node>) -> Rc<Node>
	{
		let node = rc_dom.append_new_element_before_sibling_node(sibling_node, self.local_name.qual_name(), self.attributes);
		Self::process_children(rc_dom, self.children, node)
	}
	
	#[inline(always)]
	fn process_children(rc_dom: &mut RcDom, mut children: Vec<Either<String, UnattachedNode>>, parent_node: Rc<Node>) -> Rc<Node>
	{
		for child in children.drain(..)
		{
			match child
			{
				Left(text) => parent_node.append_text(rc_dom, &text),
				Right(child) =>
				{
					child.attach_to_parent_node(rc_dom, &parent_node);
				}
			}
		}
		parent_node
	}
}
