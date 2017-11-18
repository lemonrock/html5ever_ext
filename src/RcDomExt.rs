// This file is part of cordial. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of cordial. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/cordial/master/COPYRIGHT.


/// This trait adds additional methods to a HTML DOM.
pub trait RcDomExt: Sized + Minify
{
	/// Creates an instance of an HTML DOM from a file path which is verified, stripped and with a sane DocType.
	#[inline(always)]
	fn from_file_path_verified_and_stripped_of_comments_and_processing_instructions_and_with_a_sane_doc_type<P: AsRef<Path>>(html_document_file_path: P) -> Result<Self, HtmlError>;
	
	/// Creates an instance of an HTML DOM from a file path
	#[inline(always)]
	fn from_file_path<P: AsRef<Path>>(file_path: P) -> Result<Self, HtmlError>;
	
	/// Creates an instance of an HTML DOM from bytes which is verified, stripped and with a sane DocType.
	#[inline(always)]
	fn from_bytes_verified_and_stripped_of_comments_and_processing_instructions_and_with_a_sane_doc_type<P: AsRef<Path>>(bytes: &[u8], context: P) -> Result<Self, HtmlError>;
	
	/// Creates an instance of an HTML DOM from bytes
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Self;
	
	/// Verify that this HTML DOM is valid.
	#[inline(always)]
	fn verify(&self, context: &Path) -> Result<(), HtmlError>;
	
	/// Remove all comments and processing instructions and make the DOCTYPE a simple 'html' (for HTML 5).
	fn recursively_strip_nodes_of_comments_and_processing_instructions_and_create_sane_doc_type(&self, context: &Path) -> Result<(), HtmlError>;
	
	/// Moves a node's children to the document node
	#[inline(always)]
	fn move_node_children_to_document_node(&mut self, node: &Rc<Node>);
	
	/// Appends a new element node to the document node
	#[inline(always)]
	fn move_node_to_document_node(&mut self, node: &Rc<Node>);
	
	/// Appends a new element node to the document node
	#[inline(always)]
	fn append_new_element_to_document_node(&mut self, qualified_name: QualName, attributes: Vec<Attribute>);
	
	/// Appends a new comment node to the document node
	#[inline(always)]
	fn append_new_comment_to_document_node(&mut self, comment: &str);
	
	/// Appends a new processing instruction node to the document node
	#[inline(always)]
	fn append_new_processing_instruction_to_document_node(&mut self, target: &str, data: &str);
	
	/// Appends a text node to the document node
	#[inline(always)]
	fn append_text_to_document_node(&mut self, text: &str);
	
	/// Moves a node's children to another parent node
	#[inline(always)]
	fn move_node_children_to_parent_node(&mut self, parent_node: &Rc<Node>, node: &Rc<Node>);
	
	/// Moves an existing element node to a parent node
	#[inline(always)]
	fn move_node_to_parent_node(&mut self, parent_node: &Rc<Node>, node: &Rc<Node>);
	
	/// Moves an existing element before a sibling node
	#[inline(always)]
	fn move_node_before_sibling_node(&mut self, sibling_node: &Rc<Node>, node: &Rc<Node>);
	
	#[doc(hidden)]
	#[inline(always)]
	fn _create_parent_less_element(&mut self, qualified_name: QualName, attributes: Vec<Attribute>) -> Rc<Node>;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _create_parent_less_comment(&mut self, comment: &str) -> Rc<Node>;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _create_parent_less_processing_instruction(&mut self, target: &str, data: &str) -> Rc<Node>;
	
	/// Appends a new element node to a parent node
	#[inline(always)]
	fn append_new_element_to_parent_node(&mut self, parent_node: &Rc<Node>, qualified_name: QualName, attributes: Vec<Attribute>);
	
	/// Appends a new element before a sibling node
	#[inline(always)]
	fn append_new_element_before_sibling_node(&mut self, sibling_node: &Rc<Node>, qualified_name: QualName, attributes: Vec<Attribute>);
	
	/// Appends a new comment node to a parent node
	#[inline(always)]
	fn append_new_comment_to_parent_node(&mut self, parent_node: &Rc<Node>, comment: &str);
	
	/// Appends a new comment before a sibling node
	#[inline(always)]
	fn append_new_comment_before_sibling_node(&mut self, sibling_node: &Rc<Node>, comment: &str);
	
	/// Appends a new processing instruction node to a parent node
	#[inline(always)]
	fn append_new_processing_instruction_to_parent_node(&mut self, parent_node: &Rc<Node>, target: &str, data: &str);
	
	/// Appends a new processing instruction before a sibling node
	#[inline(always)]
	fn append_new_processing_instruction_before_sibling_node(&mut self, sibling_node: &Rc<Node>, target: &str, data: &str);
	
	/// Appends a text node to a parent node
	#[inline(always)]
	fn append_text_to_parent_node(&mut self, parent_node: &Rc<Node>, text: &str);
	
	/// Appends a text node before a sibling node
	#[inline(always)]
	fn append_text_before_sibling_node(&mut self, sibling_node: &Rc<Node>, text: &str);
	
	#[doc(hidden)]
	#[inline(always)]
	fn _verify_is_document_and_not_a_fragment(&self, context: &Path) -> Result<(), HtmlError>;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _verify_has_no_errors(&self, context: &Path) -> Result<(), HtmlError>;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _verify_has_no_quirks(&self, context: &Path) -> Result<(), HtmlError>;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _verify_root_element(&self, context: &Path) -> Result<(), HtmlError>;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _parser() -> Utf8LossyDecoder<Parser<RcDom>>
	{
		let tree_sink = RcDom::default();
		let parse_options = ParseOpts
		{
			tokenizer: TokenizerOpts
			{
				exact_errors: true,
				discard_bom: true,
				profile: false,
				initial_state: None,
				last_start_tag_name: None,
			},
			tree_builder: TreeBuilderOpts
			{
				exact_errors: true,
				scripting_enabled: true,
				iframe_srcdoc: false,
				drop_doctype: false,
				ignore_missing_rules: false,
				quirks_mode: QuirksMode::NoQuirks,
			},
		};
		let parser = parse_document(tree_sink, parse_options);
		parser.from_utf8()
	}
}

impl RcDomExt for RcDom
{
	#[inline(always)]
	fn from_file_path_verified_and_stripped_of_comments_and_processing_instructions_and_with_a_sane_doc_type<P: AsRef<Path>>(html_document_file_path: P) -> Result<Self, HtmlError>
	{
		let path = html_document_file_path.as_ref();
		
		let document = Self::from_file_path(path)?;
		document.verify(path)?;
		document.recursively_strip_nodes_of_comments_and_processing_instructions_and_create_sane_doc_type(path)?;
		Ok(document)
	}
	
	#[inline(always)]
	fn from_file_path<P: AsRef<Path>>(html_document_file_path: P) -> Result<Self, HtmlError>
	{
		let path = html_document_file_path.as_ref();
		let document = Self::_parser().from_file(path).context(path)?;
		Ok(document)
	}
	
	#[inline(always)]
	fn from_bytes_verified_and_stripped_of_comments_and_processing_instructions_and_with_a_sane_doc_type<P: AsRef<Path>>(bytes: &[u8], context: P) -> Result<Self, HtmlError>
	{
		let path = context.as_ref();
		let document = Self::from_bytes(bytes);
		document.verify(path)?;
		document.recursively_strip_nodes_of_comments_and_processing_instructions_and_create_sane_doc_type(path)?;
		Ok(document)
	}
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Self
	{
		Self::_parser().one(bytes)
	}
	
	#[inline(always)]
	fn verify(&self, context: &Path) -> Result<(), HtmlError>
	{
		self._verify_is_document_and_not_a_fragment(context)?;
		self._verify_has_no_errors(context)?;
		self._verify_has_no_quirks(context)?;
		self._verify_root_element(context)
	}
	
	fn recursively_strip_nodes_of_comments_and_processing_instructions_and_create_sane_doc_type(&self, context: &Path) -> Result<(), HtmlError>
	{
		let document = &self.document;
		document.validate_children_and_remove_comments_and_processing_instructions(context)?;
		
		let doctype_node = Node
		{
			parent: Cell::new(Some(Rc::downgrade(document))),
			children: RefCell::new(Vec::new()),
			data: Doctype
			{
				name: "html".into(),
				public_id: "".into(),
				system_id: "".into(),
			},
		};
		document.children.borrow_mut().insert(0, Rc::new(doctype_node));
		Ok(())
	}
	
	#[inline(always)]
	fn move_node_children_to_document_node(&mut self, node: &Rc<Node>)
	{
		let document = self.document.clone();
		self.move_node_children_to_parent_node(&document, node)
	}
	
	#[inline(always)]
	fn move_node_to_document_node(&mut self, node: &Rc<Node>)
	{
		let document = self.document.clone();
		self.move_node_to_parent_node(&document, node)
	}
	
	#[inline(always)]
	fn append_new_element_to_document_node(&mut self, qualified_name: QualName, attributes: Vec<Attribute>)
	{
		let document = self.document.clone();
		self.append_new_element_to_parent_node(&document, qualified_name, attributes)
	}
	
	#[inline(always)]
	fn append_new_comment_to_document_node(&mut self, comment: &str)
	{
		let document = self.document.clone();
		self.append_new_comment_to_parent_node(&document, comment)
	}
	
	#[inline(always)]
	fn append_new_processing_instruction_to_document_node(&mut self, target: &str, data: &str)
	{
		let document = self.document.clone();
		self.append_new_processing_instruction_to_parent_node(&document, target, data)
	}
	
	#[inline(always)]
	fn append_text_to_document_node(&mut self, text: &str)
	{
		let document = self.document.clone();
		self.append(&document, AppendText(StrTendril::from_slice(text)));
	}
	
	#[inline(always)]
	fn move_node_children_to_parent_node(&mut self, parent_node: &Rc<Node>, node: &Rc<Node>)
	{
		self.reparent_children(node, parent_node);
	}
	
	#[inline(always)]
	fn move_node_to_parent_node(&mut self, parent_node: &Rc<Node>, node: &Rc<Node>)
	{
		self.remove_from_parent(node);
		self.append(parent_node, AppendNode(node.clone()))
	}
	
	#[inline(always)]
	fn move_node_before_sibling_node(&mut self, sibling_node: &Rc<Node>, node: &Rc<Node>)
	{
		self.remove_from_parent(node);
		self.append_before_sibling(sibling_node, AppendNode(node.clone()))
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _create_parent_less_element(&mut self, qualified_name: QualName, attributes: Vec<Attribute>) -> Rc<Node>
	{
		let mut element_flags = ElementFlags::default();
		element_flags.template = qualified_name.is_only_local(&local_name!("template"));
		element_flags.mathml_annotation_xml_integration_point = false;
		self.create_element(qualified_name, attributes, element_flags)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _create_parent_less_comment(&mut self, comment: &str) -> Rc<Node>
	{
		self.create_comment(StrTendril::from_slice(comment))
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _create_parent_less_processing_instruction(&mut self, target: &str, data: &str) -> Rc<Node>
	{
		self.create_pi(StrTendril::from_slice(target), StrTendril::from_slice(data))
	}
	
	#[inline(always)]
	fn append_new_element_to_parent_node(&mut self, parent_node: &Rc<Node>, qualified_name: QualName, attributes: Vec<Attribute>)
	{
		let node = self._create_parent_less_element(qualified_name, attributes);
		self.append(parent_node, AppendNode(node))
	}
	
	#[inline(always)]
	fn append_new_element_before_sibling_node(&mut self, sibling_node: &Rc<Node>, qualified_name: QualName, attributes: Vec<Attribute>)
	{
		let node = AppendNode(self._create_parent_less_element(qualified_name, attributes));
		self.append_before_sibling(sibling_node, node)
	}
	
	#[inline(always)]
	fn append_new_comment_to_parent_node(&mut self, parent_node: &Rc<Node>, comment: &str)
	{
		let node = self._create_parent_less_comment(comment);
		self.append(parent_node, AppendNode(node))
	}
	
	#[inline(always)]
	fn append_new_comment_before_sibling_node(&mut self, sibling_node: &Rc<Node>, comment: &str)
	{
		let node = self._create_parent_less_comment(comment);
		self.append_before_sibling(sibling_node, AppendNode(node))
	}
	
	#[inline(always)]
	fn append_new_processing_instruction_to_parent_node(&mut self, parent_node: &Rc<Node>, target: &str, data: &str)
	{
		let node = self._create_parent_less_processing_instruction(target, data);
		self.append(parent_node, AppendNode(node))
	}
	
	#[inline(always)]
	fn append_new_processing_instruction_before_sibling_node(&mut self, sibling_node: &Rc<Node>, target: &str, data: &str)
	{
		let node = self._create_parent_less_processing_instruction(target, data);
		self.append_before_sibling(sibling_node, AppendNode(node))
	}
	
	#[inline(always)]
	fn append_text_to_parent_node(&mut self, parent_node: &Rc<Node>, text: &str)
	{
		self.append(parent_node, AppendText(StrTendril::from_slice(text)));
	}
	
	#[inline(always)]
	fn append_text_before_sibling_node(&mut self, sibling_node: &Rc<Node>, text: &str)
	{
		self.append_before_sibling(sibling_node, AppendText(StrTendril::from_slice(text)))
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _verify_is_document_and_not_a_fragment(&self, context: &Path) -> Result<(), HtmlError>
	{
		match self.document.data
		{
			Document => Ok(()),
			_ => Err(HtmlError::InvalidFile(context.to_path_buf(), "HTML should be a rooted document".to_owned())),
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _verify_has_no_errors(&self, context: &Path) -> Result<(), HtmlError>
	{
		if self.errors.is_empty()
		{
			Ok(())
		}
		else
		{
			Err(HtmlError::InvalidFile(context.to_path_buf(), format!("HTML parsed with errors '{:?}'", self.errors)))
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _verify_has_no_quirks(&self, context: &Path) -> Result<(), HtmlError>
	{
		use ::html5ever::tree_builder::QuirksMode;
		
		if self.quirks_mode == QuirksMode::NoQuirks
		{
			Ok(())
		}
		else
		{
			Err(HtmlError::InvalidFile(context.to_path_buf(), "HTML should not need quirks for parsing in 2017".to_owned()))
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn _verify_root_element(&self, context: &Path) -> Result<(), HtmlError>
	{
		let mut has_doc_type = false;
		let mut has_html_root = false;
		for child_of_document in self.document.children.borrow().iter()
		{
			match child_of_document.data
			{
				Text { .. } => return Err(HtmlError::InvalidFile(context.to_path_buf(), "Text nodes are not allowed in the root".to_owned())),
				
				Document => return Err(HtmlError::InvalidFile(context.to_path_buf(), "Document nodes are not allowed in the root".to_owned())),
				
				Doctype { ref name, ref public_id, ref system_id } =>
				{
					if has_doc_type
					{
						return Err(HtmlError::InvalidFile(context.to_path_buf(), "multiple DOCTYPE".to_owned()));
					}
					has_doc_type = true;
					if has_html_root
					{
						return Err(HtmlError::InvalidFile(context.to_path_buf(), "DOCTYPE after html root".to_owned()));
					}
					if !name.eq_ignore_ascii_case("html")
					{
						return Err(HtmlError::InvalidFile(context.to_path_buf(), format!("Non html DOCTYPE '{}' found in document root", name)));
					}
					if !public_id.is_empty()
					{
						return Err(HtmlError::InvalidFile(context.to_path_buf(), format!("Non empty DOCTYPE public id '{}' found in document root", public_id)));
					}
					if !system_id.is_empty()
					{
						return Err(HtmlError::InvalidFile(context.to_path_buf(), format!("Non empty DOCTYPE system id '{}' found in document root", system_id)));
					}
				},
				
				NodeData::Element { ref name, .. } =>
				{
					if !name.is_only_local(&local_name!("html"))
					{
						return Err(HtmlError::InvalidFile(context.to_path_buf(), format!("Non html-element '{:?}' found in document root", name)));
					}
					if has_html_root
					{
						return Err(HtmlError::InvalidFile(context.to_path_buf(), "Multiple html elements in document root".to_owned()));
					}
					has_html_root = true;
				}
				
				ProcessingInstruction { .. } | Comment { .. } => (), //ignore
			}
		}
		
		Ok(())
	}
}
