// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


/// This trait adds additional methods to a a HTML DOM node.
pub trait NodeExt: Sized + Minify
{
	/// Validated a HTML DOM node, removes any child comments and processing instructions.
	fn validate_children_and_remove_comments_and_processing_instructions(&self, context: &Path) -> Result<(), HtmlError>;
	
	/// Returns the parent of this node.
	/// Returns None if there is no parent.
	/// Ordinarily, the 'root' node is of type 'Document'.
	/// The :root CSS pseudo-element matches the first html element node child of the root node of type 'Document', not 'Document' itself.
	#[inline(always)]
	fn parent(&self) -> Option<Self>;
	
	/// Returns the first child if extant, which may be an element, text node (or, if not preprocessed, a comment or processing instruction)
	#[inline(always)]
	fn first_child(&self) -> Option<Self>;
	
	/// Returns the previous sibling, or None if this is the first sibling
	#[inline(always)]
	fn previous_sibling(&self, skip_inter_element_whitespace_comment_or_processing_instructions: bool) -> Option<Self>
	{
		self._previous_or_next_sibling(false, skip_inter_element_whitespace_comment_or_processing_instructions)
	}
	
	/// Returns the next sibling, or None if this is the last sibling
	#[inline(always)]
	fn next_sibling(&self, skip_inter_element_whitespace_comment_or_processing_instructions: bool) -> Option<Self>
	{
		self._previous_or_next_sibling(true, skip_inter_element_whitespace_comment_or_processing_instructions)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _previous_or_next_sibling(&self, next: bool, skip_inter_element_whitespace_comment_or_processing_instructions: bool) -> Option<Self>;
	
	/// Used for determining siblings.
	#[inline(always)]
	fn is_inter_element_whitespace_comment_or_processing_instruction(&self) -> bool;
}

impl NodeExt for Rc<Node>
{
	fn validate_children_and_remove_comments_and_processing_instructions(&self, context: &Path) -> Result<(), HtmlError>
	{
		if !self.can_have_children() && !self.children.borrow().is_empty()
		{
			return Err(HtmlError::InvalidFile(context.to_path_buf(), format!("This node contains children when it should not ({}).", self.debug_string())));
		}
		
		let mut processed_children = Vec::with_capacity(self.children.borrow().len());
		
		let mut previous_was_text_node = false;
		let mut last_added_node_was_text_node = false;
		for child_node in self.children.borrow().iter()
		{
			match child_node.data
			{
				Document => return Err(HtmlError::InvalidFile(context.to_path_buf(), "Document nodes are not valid children".to_owned())),
				
				Doctype { .. } => match self.data
				{
					Document => (),
					
					_ => return Err(HtmlError::InvalidFile(context.to_path_buf(), "DOCTYPE nodes are not valid children except for Document nodes".to_owned())),
				},
				
				NodeData::Element { .. } =>
				{
					child_node.validate_children_and_remove_comments_and_processing_instructions(context)?;
					
					processed_children.push(child_node.clone());
					
					previous_was_text_node = false;
					last_added_node_was_text_node = false;
				}
				
				Comment { .. } | ProcessingInstruction { .. } =>
				{
					previous_was_text_node = false;
				},
				
				Text { ref contents } =>
				{
					if previous_was_text_node
					{
						return Err(HtmlError::InvalidFile(context.to_path_buf(), "Text nodes can not have a previous sibling which is also a text node".to_owned()));
					}
					
					// Discard inter-element whitespace
					if !is_inter_element_whitespace(contents.borrow().deref())
					{
						// Merge with a previous text node; this occurs because we remove comments and processing instructions
						if last_added_node_was_text_node
						{
							let previous_text_node: Rc<Node> = processed_children.pop().unwrap();
							match previous_text_node.data
							{
								Text { contents: ref previous_node_contents } =>
								{
									let merged_node = Node
									{
										parent: Cell::new(Some(Rc::downgrade(self))),
										children: RefCell::new(Vec::new()),
										data: Text
										{
											contents:
											{
												let previous_contents = previous_node_contents.borrow();
												let contents = contents.borrow();
												let mut merged_contents: Tendril<UTF8, NonAtomic> = Tendril::with_capacity(previous_contents.len32() + contents.len32());
												merged_contents.push_tendril(&previous_contents);
												merged_contents.push_tendril(&contents);
												RefCell::new(merged_contents)
											}
										}
									};
									processed_children.push(Rc::new(merged_node));
								}
								_ => unreachable!("Previously added a text node"),
							}
							// Already true, so no need for  last_added_node_was_text_node = true;
						}
						else
						{
							processed_children.push(child_node.clone());
							last_added_node_was_text_node = true;
						}
					}
					
					previous_was_text_node = true;
				}
			}
		}
		
		let mut children = self.children.borrow_mut();
		*children = processed_children;
		
		Ok(())
	}
	
	#[inline(always)]
	fn parent(&self) -> Option<Self>
	{
		let pointer = self.parent.as_ptr();
		unsafe
		{
			match *pointer
			{
				None => None,
				Some(ref weak_parent_node) => weak_parent_node.upgrade()
			}
		}
	}
	
	#[inline(always)]
	fn first_child(&self) -> Option<Self>
	{
		self.children.borrow().get(0).map(|child| child.clone())
	}
	
	#[inline(always)]
	fn is_inter_element_whitespace_comment_or_processing_instruction(&self) -> bool
	{
		match self.data
		{
			Comment { .. } | ProcessingInstruction { .. } => true,
			
			Text { ref contents } => is_inter_element_whitespace(contents.borrow().deref()),
			
			_ => false,
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _previous_or_next_sibling(&self, next: bool, skip_inter_element_whitespace_comment_or_processing_instructions: bool) -> Option<Self>
	{
		#[inline(always)]
		fn iterate<'a, I: Iterator<Item=&'a Rc<Node>>>(this: &Rc<Node>, skip_inter_element_whitespace_comment_or_processing_instructions: bool, sibling_iterator: I) -> Option<Rc<Node>>
		{
			let mut previous_sibling = None;
			for current_sibling in sibling_iterator
			{
				if Rc::ptr_eq(this, current_sibling)
				{
					return previous_sibling;
				}
				
				if skip_inter_element_whitespace_comment_or_processing_instructions
				{
					if !current_sibling.is_inter_element_whitespace_comment_or_processing_instruction()
					{
						previous_sibling = Some(current_sibling.clone());
					}
				}
				else
				{
					previous_sibling = Some(current_sibling.clone());
				}
			}
			unreachable!();
		}
		
		if let Some(parent) = self.parent()
		{
			let borrowed = parent.children.borrow();
			let iterator = borrowed.iter();
			if next
			{
				iterate(self, skip_inter_element_whitespace_comment_or_processing_instructions, iterator.rev())
			}
			else
			{
				iterate(self, skip_inter_element_whitespace_comment_or_processing_instructions, iterator)
			}
		}
		else
		{
			None
		}
	}
}
