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

impl<'a> From<&'a str> for UnattachedNode
{
	#[inline(always)]
	fn from(local_name: &'a str) -> Self
	{
		LocalName::from(local_name).empty_node()
	}
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
		if id.is_empty()
		{
			return self;
		}
		self.with_attribute(local_name!("id").attribute(id))
	}
	
	/// Add a title attribute.
	#[inline(always)]
	pub fn with_title_attribute(self, title: &str) -> Self
	{
		if title.is_empty()
		{
			return self;
		}
		self.with_attribute(local_name!("title").attribute(title))
	}
	
	//noinspection SpellCheckingInspection
	/// Add an accesskey attribute.
	#[inline(always)]
	pub fn with_accesskey_attribute(self, accesskey: &str) -> Self
	{
		if accesskey.is_empty()
		{
			return self;
		}
		self.with_attribute(local_name!("accesskey").attribute(accesskey))
	}
	
	//noinspection SpellCheckingInspection
	/// Add a lang attribute.
	#[inline(always)]
	pub fn with_lang_attribute(self, lang: &str) -> Self
	{
		self.with_attribute(local_name!("lang").attribute(lang))
	}
	
	//noinspection SpellCheckingInspection
	/// Add a contenteditable attribute.
	#[inline(always)]
	pub fn with_contenteditable_attribute(self, editable: bool) -> Self
	{
		let value = if editable
		{
			""
		}
		else
		{
			"false"
		};
		
		self.with_attribute(local_name!("contenteditable").attribute(value))
	}
	
	//noinspection SpellCheckingInspection
	/// Add a spellcheck attribute.
	#[inline(always)]
	pub fn with_spellcheck_attribute(self, spellcheck: bool) -> Self
	{
		let value = if spellcheck
		{
			"true"
		}
		else
		{
			"false"
		};
		
		self.with_attribute(LocalName::from("spellcheck").attribute(value))
	}
	
	//noinspection SpellCheckingInspection
	/// Add a tabindex attribute.
	#[inline(always)]
	pub fn with_tabindex_attribute(self, tabindex: i32) -> Self
	{
		self.with_attribute(local_name!("tabindex").attribute(&format!("{}", tabindex)))
	}
	
	//noinspection SpellCheckingInspection
	/// Add a hidden attribute.
	#[inline(always)]
	pub fn with_hidden_attribute(self, hidden: bool) -> Self
	{
		if hidden
		{
			self.with_attribute(local_name!("hidden").empty_attribute())
		}
		else
		{
			self
		}
	}
	
	//noinspection SpellCheckingInspection
	/// Add a contextmenu attribute.
	#[inline(always)]
	pub fn with_contextmenu_attribute(self, id: &str) -> Self
	{
		if id.is_empty()
		{
			return self;
		}
		self.with_attribute(local_name!("contextmenu").attribute(id))
	}
	
	/// Add a data attribute.
	#[inline(always)]
	pub fn with_data_attribute(self, name: &str, value: &str) -> Self
	{
		const PREFIX: &'static str = "data-";
		let mut attribute_name = String::with_capacity(PREFIX.len() + name.len());
		attribute_name.push_str(PREFIX);
		attribute_name.push_str(name);
		let local_name = LocalName::from(attribute_name.as_str());
		
		self.with_attribute(local_name.attribute(value))
	}
	
	/// Add a draggable attribute.
	#[inline(always)]
	pub fn with_draggable_attribute(self, draggable: Draggable) -> Self
	{
		self.with_attribute(local_name!("draggable").attribute(draggable.to_str()))
	}
	
	/// Add a dir attribute.
	#[inline(always)]
	pub fn with_dir_attribute(self, dir: Dir) -> Self
	{
		self.with_attribute(local_name!("dir").attribute(dir.to_str()))
	}
	
	/// Add a href attribute.
	#[inline(always)]
	pub fn with_href_attribute(self, href: &str) -> Self
	{
		self.with_attribute(local_name!("href").attribute(href))
	}
	
	/// Add a class attribute.
	#[inline(always)]
	pub fn with_class_attribute(self, classes: &str) -> Self
	{
		self.with_attribute(local_name!("class").attribute(classes))
	}
	
	/// Add a class attribute.
	#[inline(always)]
	pub fn with_role_attribute(self, role: AriaRole) -> Self
	{
		self.with_attribute(local_name!("role").attribute(role.to_str()))
	}
	
	/// Add a class attribute from classes
	#[inline(always)]
	pub fn with_class_attribute_from_classes<S: Deref<Target=str>>(self, classes: &[S]) -> Self
	{
		let mut class_string = String::new();
		let mut after_first = false;
		for class in classes.iter()
		{
			let value = class.deref();
			if !value.is_empty()
			{
				if after_first
				{
					class_string.push(' ');
				}
				else
				{
					after_first = true;
				}
				class_string.push_str(value);
			}
		}
		self.with_class_attribute(&class_string)
	}
	
	/// Add classes irrespective of whether a class attribute exists or not.
	#[inline(always)]
	pub fn with_classes<S: Deref<Target=str>>(mut self, classes: &[S]) -> Self
	{
		for class in classes.iter()
		{
			let value = class.deref();
			if !value.is_empty()
			{
				self = self.with_class(value)
			}
		}
		self
	}
	
	/// Add a class attribute if it does not exist, or appends to an existing class attribute if it does
	#[inline(always)]
	pub fn with_class(mut self, value: &str) -> Self
	{
		if value.is_empty()
		{
			return self;
		}
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
				attribute.value.push_slice(value);
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
			self.with_class_attribute(value)
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
