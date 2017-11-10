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
	
	/// Verify that this HTML DOM is valid.
	#[inline(always)]
	fn verify(&self, context: &Path) -> Result<(), HtmlError>;
	
	/// Remove all comments and processing instructions and make the DOCTYPE a simple 'html' (for HTML 5).
	fn recursively_strip_nodes_of_comments_and_processing_instructions_and_create_sane_doc_type(&self, context: &Path) -> Result<(), HtmlError>;
	
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
		use ::html5ever::driver::parse_document;
		use ::html5ever::driver::ParseOpts;
		use ::html5ever::tokenizer::TokenizerOpts;
		use ::html5ever::tree_builder::QuirksMode;
		use ::html5ever::tree_builder::TreeBuilderOpts;
		use ::html5ever::tendril::TendrilSink;
		
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
		
		let path = html_document_file_path.as_ref();
		let document = parser.from_utf8().from_file(path).context(path)?;
		Ok(document)
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
