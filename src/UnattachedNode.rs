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
	pub fn with_attributes_and_text<S: Into<String>>(local_name: LocalName, attributes: Vec<Attribute>, text: S) -> Self
	{
		Self
		{
			local_name,
			attributes,
			children: vec!
			[
				Left(text.into()),
			],
		}
	}
	
	/// Represents an empty element with just text, such as <code>hello</code>
	#[inline(always)]
	pub fn with_text<S: Into<String>>(local_name: LocalName, text: S) -> Self
	{
		Self
		{
			local_name,
			attributes: vec![],
			children: vec!
			[
				Left(text.into()),
			],
		}
	}
	
	/// Add an id attribute.
	#[inline(always)]
	pub fn with_id_attribute(self, id: &str) -> Self
	{
		self.with_attribute(local_name!("id").attribute(id))
	}
	
	/// Add a title attribute.
	#[inline(always)]
	pub fn with_title_attribute(self, title: &str) -> Self
	{
		self.with_attribute(local_name!("title").attribute(title))
	}
	
	/// Add a class attribute.
	#[inline(always)]
	pub fn with_class_attribute(self, classes: &str) -> Self
	{
		self.with_attribute(local_name!("class").attribute(classes))
	}
	
	/// Add a class attribute from classes
	#[inline(always)]
	pub fn with_class_attribute_from_classes<S: Deref<Target=str>>(self, classes: &[S]) -> Self
	{
		let mut class_string = String::new();
		let mut after_first = false;
		for class in classes.iter()
		{
			if after_first
			{
				class_string.push(' ');
			}
			else
			{
				after_first = true;
			}
			class_string.push_str(class.deref());
		}
		self.with_class_attribute(&class_string)
	}
	
	/// Add classes irrespective of whether a class attribute exists or not.
	#[inline(always)]
	pub fn with_classes<S: Deref<Target=str>>(mut self, classes: &[S]) -> Self
	{
		for class in classes.iter()
		{
			self = self.with_class(class.deref())
		}
		self
	}
	
	/// Add a class attribute if it does not exist, or appends to an existing class attribute if it does
	#[inline(always)]
	pub fn with_class(mut self, class: &str) -> Self
	{
		let class_local_name = local_name!("class");
		let mut found = false;
		for attribute in self.attributes.iter_mut()
		{
			if attribute.name.is_only_local(&class_local_name)
			{
				if attribute.value.len32() != 0
				{
					attribute.value.push_char(' ');
				}
				attribute.value.push_slice(class);
				found = true;
				break;
			}
		}
		if found
		{
			self
		}
		else
		{
			self.with_class_attribute(class)
		}
	}
	
	/// Add an attribute.
	#[inline(always)]
	pub fn with_attribute(mut self, attribute: Attribute) -> Self
	{
		self.attributes.push(attribute);
		self
	}
	
	/// Add a child element.
	#[inline(always)]
	pub fn with_child_element(mut self, child_element: UnattachedNode) -> Self
	{
		self.children.push(Right(child_element));
		self
	}
	
	/// Add a child text. If there is an existing child text element, appends its text to it.
	#[inline(always)]
	pub fn with_child_text(mut self, child_text: String) -> Self
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
		self
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
