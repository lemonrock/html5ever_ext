// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


#[derive(Clone)]
struct ElementNode
{
	node: Rc<Node>,
}

impl Debug for ElementNode
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		self.node.debug_fmt(f)
	}
}

impl QualNameExt for ElementNode
{
	#[inline(always)]
	fn is_unprefixed_and_unnamespaced(&self) -> bool
	{
		self.node.is_unprefixed_and_unnamespaced()
	}
	
	#[inline(always)]
	fn is_only_local(&self, local_name: &LocalName) -> bool
	{
		self.node.is_only_local(local_name)
	}
	
	#[inline(always)]
	fn is_only_local_of(&self, local_names: &[LocalName]) -> bool
	{
		self.node.is_only_local_of(local_names)
	}
	
	#[inline(always)]
	fn can_have_children(&self) -> bool
	{
		self.node.can_have_children()
	}
	
	#[inline(always)]
	fn text_content_should_be_escaped(&self) -> bool
	{
		self.node.text_content_should_be_escaped()
	}
}

impl Element for ElementNode
{
	type Impl = OurSelectorImpl;
	
	#[inline(always)]
	fn opaque(&self) -> OpaqueElement
	{
		OpaqueElement::new(self.node.as_ref())
	}
	
	#[inline(always)]
	fn parent_element(&self) -> Option<Self>
	{
		self.node.parent().map(|node| Self { node })
	}
	
	#[inline(always)]
	fn first_child_element(&self) -> Option<Self>
	{
		self.first_or_last_child_element(false)
	}
	
	#[inline(always)]
	fn last_child_element(&self) -> Option<Self>
	{
		self.first_or_last_child_element(true)
	}
	
	#[inline(always)]
	fn prev_sibling_element(&self) -> Option<Self>
	{
		self.previous_or_next_sibling_element(false)
	}
	
	#[inline(always)]
	fn next_sibling_element(&self) -> Option<Self>
	{
		self.previous_or_next_sibling_element(true)
	}
	
	#[inline(always)]
	fn is_html_element_in_html_document(&self) -> bool
	{
		match self.node.data
		{
			NodeData::Element { ref name, .. } => name.is_only_local(&local_name!("html")),
			
			_ => false,
		}
	}
	
	#[inline(always)]
	fn get_local_name(&self) -> &<Self::Impl as SelectorImpl>::BorrowedLocalName
	{
		match self.node.data
		{
			NodeData::Element { ref name, .. } => name.local.deref(),
			
			_ => "",
		}
	}
	
	#[inline(always)]
	fn get_namespace(&self) -> &<Self::Impl as SelectorImpl>::BorrowedNamespaceUrl
	{
		match self.node.data
		{
			NodeData::Element { ref name, .. } => name.ns.deref(),
			
			_ => "",
		}
	}
	
	#[inline(always)]
	fn attr_matches(&self, ns: &NamespaceConstraint<&<Self::Impl as SelectorImpl>::NamespaceUrl>, local_name: &<Self::Impl as SelectorImpl>::LocalName, operation: &AttrSelectorOperation<&<Self::Impl as SelectorImpl>::AttrValue>)
		-> bool
	{
		use self::NamespaceConstraint::*;
		
		match self.node.data
		{
			NodeData::Element { ref attrs, .. } =>
			{
				for attribute in attrs.borrow().iter()
				{
					if attribute.name.local.deref() == local_name.deref()
					{
						match *ns
						{
							Any => return operation.eval_str(attribute.value.deref()),
							Specific(&NamespaceUrl(ref atom)) => if atom.deref() == attribute.name.ns.deref()
							{
								return operation.eval_str(attribute.value.deref());
							},
						}
					}
				}
				false
			},
			
			_ => false,
		}
	}
	
	#[inline(always)]
	fn match_non_ts_pseudo_class<F: FnMut(&Self, ElementSelectorFlags)>(&self, pc: &<Self::Impl as SelectorImpl>::NonTSPseudoClass, _context: &mut LocalMatchingContext<Self::Impl>, _relevant_link: &RelevantLinkStatus, _flags_setter: &mut F) -> bool
	{
		use ::css::domain::selectors::NonTreeStructuralPseudoClass::*;
		
		match *pc
		{
			any_link(_) | link | visited => self.is_link(),
			
			_ => true,
		}
	}
	
	#[inline(always)]
	fn match_pseudo_element(&self, _pe: &<Self::Impl as SelectorImpl>::PseudoElement, _context: &mut MatchingContext) -> bool
	{
		true
	}
	
	#[inline(always)]
	fn is_link(&self) -> bool
	{
		match self.node.data
		{
			NodeData::Element { ref name, ref attrs, .. } =>
			{
				if name.is_only_local_of(&[local_name!("a"), local_name!("area"), local_name!("link")])
				{
					Self::_use_attribute_value(&local_name!("href"), |_| true, false, attrs)
				}
				else
				{
					false
				}
			},
			
			_ => false
		}
	}
	
	#[inline(always)]
	fn has_id(&self, id: &<Self::Impl as SelectorImpl>::Identifier, case_sensitivity: CaseSensitivity) -> bool
	{
		if id.is_empty()
		{
			return false;
		}
		
		self.use_attribute_value(&local_name!("id"), |id_attribute_value| Self::case_sensitive_equality(case_sensitivity, id_attribute_value, id.deref()), false)
	}
	
	#[inline(always)]
	fn has_class(&self, name: &<Self::Impl as SelectorImpl>::ClassName, case_sensitivity: CaseSensitivity) -> bool
	{
		if name.is_empty()
		{
			return false;
		}
		
		self.use_attribute_value(&local_name!("class"), |class_attribute_value| class_attribute_value.split(SELECTOR_WHITESPACE).any(|class| Self::case_sensitive_equality(case_sensitivity, &**name, class)), false)
	}
	
	#[inline(always)]
	fn is_empty(&self) -> bool
	{
		self.node.children.borrow().is_empty()
	}
	
	#[inline(always)]
	fn is_root(&self) -> bool
	{
		if self.is_only_local(&local_name!("html"))
		{
			if let Some(parent) = self.parent_element()
			{
				match parent.node.data
				{
					Document => return true,
					_ => return false,
				}
			}
		}
		false
	}
}

impl ElementNode
{
	#[inline(always)]
	fn use_attribute_value<R, AttributeValueUser: Fn(&str) -> R>(&self, attribute_name: &LocalName, attribute_value_user: AttributeValueUser, default: R) -> R
	{
		match self.node.data
		{
			NodeData::Element { ref attrs, .. } => Self::_use_attribute_value(attribute_name, attribute_value_user, default, attrs),
			
			_ => default,
		}
	}
	
	#[inline(always)]
	fn _use_attribute_value<R, AttributeValueUser: Fn(&str) -> R>(attribute_name: &LocalName, attribute_value_user: AttributeValueUser, default: R, attrs: &RefCell<Vec<Attribute>>) -> R
	{
		for attribute in attrs.borrow().iter()
		{
			if attribute.name.is_only_local(attribute_name)
			{
				return attribute_value_user(attribute.value.deref());
			}
		}
		default
	}
	
	#[inline(always)]
	fn first_or_last_child_element(&self, last: bool) -> Option<Self>
	{
		#[inline(always)]
		fn iterate<'a, I: Iterator<Item=&'a std::rc::Rc<Node>>>(mut children_iterator: I) -> Option<ElementNode>
		{
			let mut child_node;
			while
			{
				child_node = children_iterator.next();
				child_node.is_some()
			}
			{
				let possible = child_node.unwrap();
				match possible.data
				{
					NodeData::Element { .. } => return Some(ElementNode
					{
						node: possible.clone(),
					}),
					
					_ => (),
				}
			}
			None
		}
		
		let borrowed = self.node.children.borrow();
		let iterator = borrowed.iter();
		if last
		{
			iterate(iterator.rev())
		}
		else
		{
			iterate(iterator)
		}
		
	}
	
	#[inline(always)]
	fn previous_or_next_sibling_element(&self, next: bool) -> Option<Self>
	{
		#[inline(always)]
		fn iterate<'a, I: Iterator<Item=&'a std::rc::Rc<Node>>>(this: &ElementNode, sibling_iterator: I) -> Option<ElementNode>
		{
			let mut previous_sibling = None;
			for current_sibling in sibling_iterator
			{
				if Rc::ptr_eq(&this.node, &current_sibling)
				{
					return previous_sibling;
				}
				
				match current_sibling.data
				{
					NodeData::Element { .. } =>
					{
						previous_sibling = Some(ElementNode
						{
							node: current_sibling.clone(),
						});
					}
					
					_ => (),
				}
			}
			unreachable!();
		}
		
		if let Some(parent) = self.parent_element()
		{
			let borrowed = parent.node.children.borrow();
			let iterator = borrowed.iter();
			if next
			{
				iterate(self, iterator.rev())
			}
			else
			{
				iterate(self, iterator)
			}
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	fn case_sensitive_equality(case_sensitivity: CaseSensitivity, left: &str, right: &str) -> bool
	{
		case_sensitivity.eq(left.as_bytes(), right.as_bytes())
	}
}
