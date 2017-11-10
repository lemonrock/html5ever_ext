// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


/// A serializer that, unlike that in the html5ever crate (which is private and used via `::html5ever::serialize::serialize()`), tries hard to minify HTML and make it compression-friendly.
/// Use this struct directly if you need to serialize multiple nodes or doms to one writer, or control when flushing of the output writer should occur.
/// Otherwise, use the trait `Minify`.
///
/// This serializer will:-
///
/// * write value-omitted, quote-less and both single- and double-quoted attributes to minimize their length.
/// * omit opening and closing tags as permitted, optionally retaining those needed for AMP pages.
/// * converts element names, attribute names and DTD names to ASCII lower-case.
/// * will not write out text nodes consisting entirely of inter-element whitespace.
/// * will normalize inter-element whitespace sequences in text nodes, except when within a <code>, <samp>, <kbd> or <pre> element, unless `collapse_whitespace` is explicitly set to false.
/// * will correctly write DTDs with public and system ids, unlike the regular one in html5ever.
/// * will not escape the backtick (grave) `\`` in attribute values. At one time Internet Explorer used to use this as a third way to quote attribute values.
///
/// The serializer has a small number of limitations:-
///
/// * it always escapes ampersands `&` (and less-than `<` in raw text) where escaping is required even though there are rare places they could be left unescaped.
/// * does not differentiate between the different kinds of text blocks (template, etc) beyond can-be-escaped and does-not-need-to-be-escaped.
/// * does not reorder class names or attributes for possibly better compression.
///
#[derive(Debug, Clone)]
pub struct UltraMinifyingHtmlSerializer<W: Write>
{
	html_head_and_body_tags_are_optional: bool,
	preserve_comments: bool,
	preserve_processing_instructions: bool,
	writer: W,
}

impl<W: Write> UltraMinifyingHtmlSerializer<W>
{
	/// Creates a new writer.
	/// If creating AMP pages, set `html_head_and_body_tags_are_optional` to false.
	/// To preserve comments, set `preserve_comments` to true.
	/// To preserve processing instructions, set `preserve_processing_instructions` to true.
	#[inline(always)]
	pub fn new(html_head_and_body_tags_are_optional: bool, preserve_comments: bool, preserve_processing_instructions: bool, writer: W) -> Self
	{
		Self
		{
			writer,
			html_head_and_body_tags_are_optional,
			preserve_comments,
			preserve_processing_instructions,
		}
	}
	
	/// Serializes a HTML document object model.
	/// `collapse_whitespace` should normally by `true`. If a `<pre>`, `<code>`, `<samp>`, or `<kbd>` element is encountered, it is set to `false`.
	/// Output is flushed after serialization finishes.
	#[inline(always)]
	pub fn serialize_rc_dom(&mut self, rc_dom: &RcDom, collapse_whitespace: bool) -> io::Result<()>
	{
		self.serialize_node(&rc_dom.document, collapse_whitespace, true)
	}
	
	/// Serializes a HTML document object model node.
	/// Can be called repeatedly.
	/// `collapse_whitespace` should normally by `true`. If a `<pre>`, `<code>`, `<samp>`, or `<kbd>` element is encountered, it is set to `false`.
	/// If serializing HTML fragments, make `flush_when_serialized` true for each fragment serialized.
	pub fn serialize_node(&mut self, node: &Rc<Node>, collapse_whitespace: bool, flush_when_serialized: bool) -> io::Result<()>
	{
		match node.data
		{
			Comment { ref contents } if self.preserve_comments => self.write_comment(contents)?,
			
			ProcessingInstruction { ref target, ref contents } if self.preserve_processing_instructions => self.write_processing_instruction(target, contents)?,
			
			Doctype { ref name, ref public_id, ref system_id } => self.write_doctype(name, public_id, system_id)?,
			
			Text { ref contents } => self.write_text(contents, collapse_whitespace, node.parent())?,
			
			Document => for child_node in node.children.borrow().iter()
			{
				self.serialize_node(child_node, collapse_whitespace, false)?;
			},
			
			NodeData::Element { ref name, ref attrs, .. } =>
			{
				if !self.omit_start_element(node, name, attrs)
				{
					self.write_start_element(name, attrs)?;
				}
				
				if name.can_have_children()
				{
					let collapse_whitespace_of_children = if collapse_whitespace
					{
						name.can_collapse_whitespace()
					}
					else
					{
						false
					};
					
					for child_node in node.children.borrow().iter()
					{
						self.serialize_node(child_node, collapse_whitespace_of_children, false)?;
					}
					
					if !self.omit_end_element(node, name)
					{
						self.write_end_element(name)?;
					}
				}
			}
			
			_ => (),
		}
		
		if flush_when_serialized
		{
			self.writer.flush()
		}
		else
		{
			Ok(())
		}
	}
	
	fn write_start_element(&mut self, name: &QualName, attributes: &RefCell<Vec<Attribute>>) -> io::Result<()>
	{
		self.write_all(b"<")?;
		self.write_all_qualified_name(&name)?;
		for attribute in attributes.borrow().iter()
		{
			let attribute_name = &attribute.name;
			let attribute_value = attribute.value.deref();
			
			
			// Write space before attribute
			
			self.write_all(b" ")?;
			
			
			// Write attribute name
			
			// Special exemption to write xmlns:xmlns as xmlns
			if attribute_name.ns == ns!(xmlns) && attribute_name.local == local_name!("xmlns")
			{
				self.write_all_str("xmlns")?;
			}
			else
			{
				self.write_all_qualified_name(attribute_name)?;
			}
			
			
			// Write attribute value (with '=' only if not-an-empty attribute)
			
			if !attribute_value.is_empty()
			{
				// From HTML 5 specification at https://www.w3.org/TR/html5/syntax.html#attributes-0
				// "Unquoted form: must not contain any literal space characters, any U+0022 QUOTATION MARK characters ("), U+0027 APOSTROPHE characters ('), "=" (U+003D) characters, "<" (U+003C) characters, ">" (U+003E) characters, or U+0060 GRAVE ACCENT characters (`)"
				// "The space characters, for the purposes of this specification, are U+0020 SPACE, "tab" (U+0009), "LF" (U+000A), "FF" (U+000C), and "CR" (U+000D)."
				
				let mut can_write_unquoted = true;
				let mut contains_double_quotes = false;
				let mut contains_single_quotes = false;
				for character in attribute_value.chars()
				{
					match character
					{
						'\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' | '\u{003C}' | '\u{003D}' | '\u{003E}' | '\u{0060}' =>
						{
							can_write_unquoted = false;
						}
						'\u{0022}' =>
						{
							can_write_unquoted = false;
							contains_double_quotes = true;
						}
						'\u{0027}' =>
						{
							can_write_unquoted = false;
							contains_single_quotes = true;
						}
						_ => (),
					}
				}
				
				self.write_all(b"=")?;
				
				
				// In theory, we don't always have to escape ampersand (`&`). In practice, because of "An ambiguous ampersand is a U+0026 AMPERSAND character (&) that is followed by one or more alphanumeric ASCII characters, followed by a ";" (U+003B) character, where these characters do not match any of the names given in the named character references section" in the HTML 5 specification, we do; it would be rare for an unescaped ampersand to be unambiguous.
				
				if can_write_unquoted
				{
					self.write_attribute_value_escaping_only_ampersand(attribute_value)?;
				}
				// Write as ='attribute_value' and escape single quotes `'` in attribute_value if `contains_single_quotes`
				else if contains_double_quotes
				{
					self.write_single_quote()?;
					if contains_single_quotes
					{
						self.write_attribute_value_escaping_ampersand_and_single_quote(attribute_value)?;
					}
					// There are no single quotes
					else
					{
						self.write_attribute_value_escaping_only_ampersand(attribute_value)?;
					}
					self.write_single_quote()?;
				}
				// Write as ="attribute_value"; since we've previously evaluated contains_double_quotes as false, there can be no double quotes in attribute_value
				else if contains_single_quotes
				{
					self.write_double_quote()?;
					self.write_attribute_value_escaping_only_ampersand(attribute_value)?;
					self.write_double_quote()?;
				}
				// does not contain double or single quotes; prefer the single quoted form ='attribute_value'
				else
				{
					self.write_single_quote()?;
					self.write_attribute_value_escaping_only_ampersand(attribute_value)?;
					self.write_single_quote()?;
				}
			}
		}
		self.write_all(b">")
	}
	
	// Does not return true for those elements that can not have children
	//noinspection SpellCheckingInspection
	fn omit_end_element(&self, node: &Rc<Node>, name: &QualName) -> bool
	{
		// The html, head and body start tags can not be omitted for the Google AMP variant of HTML.
		if name.is_unprefixed_and_html_namespace_or_none()
		{
			match name.local
			{
				local_name!("html") if self.html_head_and_body_tags_are_optional =>
				{
					// "An html element's end tag may be omitted if the html element is not immediately followed by a comment."
					// get parent(), iterate its children to find this node, then look for following node
					if let Some(next_sibling_node) = node.next_sibling(false)
					{
						match next_sibling_node.data
						{
							Comment { .. } => false,
							
							_ => true,
						}
					}
					else
					{
						true
					}
				}
				
				local_name!("head") if self.html_head_and_body_tags_are_optional =>
				{
					// "A head element's end tag may be omitted if the head element is not immediately followed by a space character or a comment."
					if let Some(next_sibling_node) = node.next_sibling(false)
					{
						match next_sibling_node.data
						{
							Text { ref contents } => if let Some(character) = contents.borrow().deref().chars().nth(0)
							{
								!is_space_character(character)
							}
							else
							{
								true
							},
							
							Comment { .. } => false,
							
							_ => true,
						}
					}
					else
					{
						true
					}
				}
				
				local_name!("body") if self.html_head_and_body_tags_are_optional =>
				{
					// "A body element's end tag may be omitted if the body element is not immediately followed by a comment."
					if let Some(next_sibling_node) = node.next_sibling(false)
					{
						match next_sibling_node.data
						{
							Comment { .. } => false,
							
							_ => true,
						}
					}
					else
					{
						true
					}
				}
				
				local_name!("li") =>
				{
					// "An li element's end tag may be omitted if the li element is immediately followed by another li element or if there is no more content in the parent element."
					match node.next_sibling(false)
					{
						Some(following) => following.is_only_local(&local_name!("li")),
						None => true,
					}
				}
				
				local_name!("dt") =>
				{
					// "A dt element's end tag may be omitted if the dt element is immediately followed by another dt element or a dd element."
					match node.next_sibling(false)
					{
						Some(following) => following.is_only_local_of(&[local_name!("dt"), local_name!("dd")]),
						None => false,
					}
				}
				
				local_name!("dd") =>
				{
					// "A dd element's end tag may be omitted if the dd element is immediately followed by another dd element or a dt element, or if there is no more content in the parent element."
					match node.next_sibling(false)
					{
						Some(following) => following.is_only_local_of(&[local_name!("dd"), local_name!("dt")]),
						None => true,
					}
				}
				
				local_name!("p") =>
				{
					// "A p element's end tag may be omitted if the p element is immediately followed by an address, article, aside, blockquote, div, dl, fieldset, footer, form, h1, h2, h3, h4, h5, h6, header, hgroup, hr, main, nav, ol, p, pre, section, table, or ul, element, or if there is no more content in the parent element and the parent element is not an a element."
					match node.next_sibling(false)
					{
						Some(following) => following.is_only_local_of(&[local_name!("address"), local_name!("article"), local_name!("aside"), local_name!("blockquote"), local_name!("div"), local_name!("dl"), local_name!("fieldset"), local_name!("footer"), local_name!("form"), local_name!("h1"), local_name!("h2"), local_name!("h3"), local_name!("h4"), local_name!("h5"), local_name!("h6"), local_name!("header"), local_name!("hgroup"), local_name!("hr"), local_name!("main"), local_name!("nav"), local_name!("ol"), local_name!("p"), local_name!("pre"), local_name!("section"), local_name!("table"), local_name!("ul")]),
						None => match node.parent()
						{
							None => false,
							Some(parent) => !parent.is_only_local(&local_name!("a"))
						}
					}
				}
				
				local_name!("rb") =>
				{
					// "An rb element's end tag may be omitted if the rb element is immediately followed by an rb, rt, rtc or rp element, or if there is no more content in the parent element."
					match node.next_sibling(false)
					{
						Some(following) => following.is_only_local_of(&[local_name!("rb"), local_name!("rt"), local_name!("rtc"), local_name!("rp")]),
						None => true,
					}
				}
				
				local_name!("rt") =>
				{
					// "An rt element's end tag may be omitted if the rt element is immediately followed by an rb, rt, rtc, or rp element, or if there is no more content in the parent element."
					match node.next_sibling(false)
					{
						Some(following) => following.is_only_local_of(&[local_name!("rb"), local_name!("rt"), local_name!("rtc"), local_name!("rp")]),
						None => true,
					}
				}
				
				local_name!("rtc") =>
				{
					// "An rtc element's end tag may be omitted if the rtc element is immediately followed by an rb, rtc or rp element, or if there is no more content in the parent element."
					match node.next_sibling(false)
					{
						Some(following) => following.is_only_local_of(&[local_name!("rb"), local_name!("rt"), local_name!("rtc"), local_name!("rp")]),
						None => true,
					}
				}
				
				local_name!("rp") =>
				{
					// "An rp element's end tag may be omitted if the rp element is immediately followed by an rb, rt, rtc or rp element, or if there is no more content in the parent element."
					match node.next_sibling(false)
					{
						Some(following) => following.is_only_local_of(&[local_name!("rb"), local_name!("rt"), local_name!("rtc"), local_name!("rp")]),
						None => true,
					}
				}
				
				local_name!("optgroup") =>
				{
					// "An optgroup element's end tag may be omitted if the optgroup element is immediately followed by another optgroup element, or if there is no more content in the parent element."
					match node.next_sibling(false)
					{
						Some(following) => following.is_only_local(&local_name!("optgroup")),
						None => true,
					}
				}
				
				local_name!("option") =>
				{
					// "An option element's end tag may be omitted if the option element is immediately followed by another option element, or if it is immediately followed by an optgroup element, or if there is no more content in the parent element."
					match node.next_sibling(false)
					{
						Some(following) => following.is_only_local_of(&[local_name!("option"), local_name!("optgroup")]),
						None => true,
					}
				}
				
				local_name!("colgroup") => Self::omit_end_element_colgroup(node),
				
				local_name!("thead") => Self::omit_end_element_thead(node),
				
				local_name!("tbody") => Self::omit_end_element_tbody(node),
				
				local_name!("tfoot") => Self::omit_end_element_tfoot(node),
				
				local_name!("tr") =>
				{
					// A tr element's end tag may be omitted if the tr element is immediately followed by another tr element, or if there is no more content in the parent element."
					match node.next_sibling(false)
					{
						Some(following) => following.is_only_local(&local_name!("tr")),
						None => true,
					}
				}
				
				local_name!("td") =>
				{
					// "A td element's end tag may be omitted if the td element is immediately followed by a td or th element, or if there is no more content in the parent element."
					match node.next_sibling(false)
					{
						Some(following) => following.is_only_local_of(&[local_name!("td"), local_name!("th")]),
						None => true,
					}
				}
				
				local_name!("th") =>
				{
					// "A th element's end tag may be omitted if the th element is immediately followed by a td or th element, or if there is no more content in the parent element."
					match node.next_sibling(false)
					{
						Some(following) => following.is_only_local_of(&[local_name!("td"), local_name!("th")]),
						None => true,
					}
				}
				
				_ => false,
			}
		}
		else
		{
			false
		}
	}
	
	// https://www.w3.org/TR/html5/syntax.html#optional-tags
	//noinspection SpellCheckingInspection
	fn omit_start_element(&self, node: &Rc<Node>, name: &QualName, attributes: &RefCell<Vec<Attribute>>) -> bool
	{
		let attributes = attributes.borrow();
		
		// If an element has no attributes it may be eligible for its start tag to be omitted.
		// The html, head and body start tags can not be omitted for the Google AMP variant of HTML.
		if attributes.is_empty() && name.is_unprefixed_and_html_namespace_or_none()
		{
			match name.local
			{
				local_name!("html") if self.html_head_and_body_tags_are_optional =>
				{
					// "An html element's start tag may be omitted if the first thing inside the html element is not a comment."
					if let Some(first_child) = node.first_child()
					{
						match first_child.data
						{
							Comment { .. } => false,
							
							_ => true,
						}
					}
					else
					{
						false
					}
				}
				
				local_name!("head") if self.html_head_and_body_tags_are_optional =>
				{
					// "A head element's start tag may be omitted if the element is empty, or if the first thing inside the head element is an element."
					if let Some(first_child) = node.first_child()
					{
						match first_child.data
						{
							NodeData::Element { .. } => true,
							
							_ => false,
						}
					}
					else
					{
						true
					}
				}
				
				local_name!("body") if self.html_head_and_body_tags_are_optional =>
				{
					// "A body element's start tag may be omitted if the element is empty, or if the first thing inside the body element is not a space character or a comment, except if the first thing inside the body element is a meta, link, script, style, or template element."
					if let Some(first_child) = node.first_child()
					{
						match first_child.data
						{
							Comment { .. } => false,
							
							Text { ref contents } => if let Some(character) = contents.borrow().deref().chars().nth(0)
							{
								!is_space_character(character)
							}
							else
							{
								true
							},
							
							NodeData::Element { ref name, .. } => if name.is_unprefixed_and_html_namespace_or_none()
							{
								match name.local
								{
									local_name!("meta") | local_name!("link") | local_name!("script") | local_name!("style") | local_name!("template") => false,
									
									_ => true,
								}
							}
							else
							{
								false
							},
							
							_ => false,
						}
					}
					else
					{
						true
					}
				}
				
				local_name!("colgroup") =>
				{
					// "A colgroup element's start tag may be omitted if the first thing inside the colgroup element is a col element, and if the element is not immediately preceded by another colgroup element whose end tag has been omitted. (It can't be omitted if the element is empty.)"
					if let Some(first_child) = node.first_child()
					{
						if first_child.is_only_local(&local_name!("col"))
						{
							// if the element is not immediately preceded by another colgroup element whose end tag has been omitted
							if let Some(ref previous_sibling) = node.previous_sibling(false)
							{
								if previous_sibling.is_only_local(&local_name!("colgroup"))
								{
									!Self::omit_end_element_colgroup(previous_sibling)
								}
								else
								{
									true
								}
							}
							else
							{
								true
							}
						}
						else
						{
							false
						}
					}
					else
					{
						false
					}
				}
				
				local_name!("tbody") =>
				{
					// "A tbody element's start tag may be omitted if the first thing inside the tbody element is a tr element, and if the element is not immediately preceded by a tbody, thead, or tfoot element whose end tag has been omitted. (It can't be omitted if the element is empty.)"
					if let Some(first_child) = node.first_child()
					{
						if first_child.is_only_local(&local_name!("tr"))
						{
							// if the element is not immediately preceded by a tbody, thead, or tfoot element whose end tag has been omitted
							if let Some(ref previous_sibling) = node.previous_sibling(false)
							{
								if previous_sibling.is_only_local(&local_name!("tbody"))
								{
									!Self::omit_end_element_tbody(previous_sibling)
								}
								else if previous_sibling.is_only_local(&local_name!("thead"))
								{
									!Self::omit_end_element_thead(previous_sibling)
								}
								else if previous_sibling.is_only_local(&local_name!("tfoot"))
								{
									!Self::omit_end_element_tfoot(previous_sibling)
								}
								else
								{
									true
								}
							}
							else
							{
								true
							}
						}
						else
						{
							false
						}
					}
					else
					{
						false
					}
				}
				
				_ => false,
			}
		}
		else
		{
			false
		}
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn omit_end_element_colgroup(colgroup_node: &Rc<Node>) -> bool
	{
		// "A colgroup element's end tag may be omitted if the colgroup element is not immediately followed by a space character or a comment."
		match colgroup_node.next_sibling(false)
		{
			Some(following) => match following.data
			{
				Text { ref contents } => if let Some(character) = contents.borrow().deref().chars().nth(0)
				{
					!is_space_character(character)
				}
				else
				{
					true
				},
				
				Comment { .. } => false,
				
				_ => true,
			},
			
			None => true,
		}
	}

	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn omit_end_element_tbody(tbody_node: &Rc<Node>) -> bool
	{
		// "A tbody element's end tag may be omitted if the tbody element is immediately followed by a tbody or tfoot element, or if there is no more content in the parent element."
		match tbody_node.next_sibling(false)
		{
			Some(following) => following.is_only_local_of(&[local_name!("tbody"), local_name!("tfoot")]),
			None => true,
		}
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn omit_end_element_thead(thead_node: &Rc<Node>) -> bool
	{
		// "A thead element's end tag may be omitted if the thead element is immediately followed by a tbody or tfoot element."
		match thead_node.next_sibling(false)
		{
			Some(following) => following.is_only_local_of(&[local_name!("tbody"), local_name!("tfoot")]),
			None => false,
		}
	}
	
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn omit_end_element_tfoot(tfoot_node: &Rc<Node>) -> bool
	{
		// "A tfoot element's end tag may be omitted if the tfoot element is immediately followed by a tbody element, or if there is no more content in the parent element."
		match tfoot_node.next_sibling(false)
		{
			Some(following) => following.is_only_local(&local_name!("tbody")),
			None => true,
		}
	}
	
	#[inline(always)]
	fn write_end_element(&mut self, name: &QualName) -> io::Result<()>
	{
		self.write_all(b"</")?;
		self.write_all_qualified_name(&name)?;
		self.write_all(b">")
	}
	
	#[inline(always)]
	fn write_text<S: Deref<Target=str>>(&mut self, contents: &RefCell<S>, collapse_whitespace: bool, parent: Option<Rc<Node>>) -> io::Result<()>
	{
		let contents = contents.borrow();
		let contents = contents.deref();
		
		if is_inter_element_whitespace(contents)
		{
			return Ok(());
		}
		
		if let Some(parent) = parent
		{
			if parent.text_content_should_be_escaped()
			{
				if collapse_whitespace
				{
					let mut previous_was_whitespace = false;
					for character in contents.chars()
					{
						if is_space_character(character)
						{
							if previous_was_whitespace
							{
								// Suppress writing multiple whitespace characters
							}
							else
							{
								// Write ' ', ie normalize whitespace
								self.write_all(b" ")?;
								previous_was_whitespace = true;
							}
						}
						else
						{
							self.write_char_escaped(character)?;
							previous_was_whitespace = false;
						}
					}
					Ok(())
				}
				else
				{
					self.write_text_escaped(contents)
				}
			}
			else
			{
				self.write_all_deref(contents)
			}
		}
		else
		{
			self.write_all_deref(contents)
		}
	}
	
	#[inline(always)]
	fn write_comment<S: Deref<Target=str>>(&mut self, contents: &S) -> io::Result<()>
	{
		self.write_all(b"<!--")?;
		self.write_all_deref(contents)?;
		self.write_all(b"-->")
	}
	
	#[inline(always)]
	fn write_doctype<S: Deref<Target=str>>(&mut self, name: &S, public_id: &S, system_id: &S) -> io::Result<()>
	{
		self.write_all(b"<!DOCTYPE ")?;
		self.write_all_str(&name.deref().to_ascii_lowercase())?;
		
		if !public_id.is_empty()
		{
			self.write_all(b"PUBLIC \"")?;
			self.write_all_deref(public_id)?;
			self.write_all(b"\" ")?;
			if !system_id.is_empty()
			{
				self.write_double_quote()?;
				self.write_all_deref(system_id)?;
				self.write_double_quote()?;
			}
		}
		else if !system_id.is_empty()
		{
			self.write_all(b"SYSTEM \"")?;
			self.write_all_deref(system_id)?;
			self.write_all(b"\" ")?;
		}
		
		self.write_all(b">")
	}
	
	#[inline(always)]
	fn write_processing_instruction<S: Deref<Target=str>>(&mut self, target: &S, contents: &S) -> io::Result<()>
	{
		self.write_all(b"<?")?;
		self.write_all_deref(target)?;
		self.write_all(b" ")?;
		self.write_all_deref(contents)?;
		self.write_all(b">")
	}
	
	#[inline(always)]
	fn write_text_escaped<S: Deref<Target=str>>(&mut self, contents: &S) -> io::Result<()>
	{
		let text = contents.deref();
		for character in text.chars()
		{
			self.write_char_escaped(character)?;
		}
		Ok(())
	}
	
	// The specification https://w3c.github.io/html/single-page.html#writing-html-documents-elements implies we do not always need to escape '<' and '&'.
	// For instance:-
	// "8.1.2.6. Restrictions on the contents of raw text and escapable raw text elements. The text in raw text and escapable raw text elements must not contain any occurrences of the string "</" (U+003C LESS-THAN SIGN, U+002F SOLIDUS) followed by characters that case-insensitively match the tag name of the element followed by one of U+0009 CHARACTER TABULATION (tab), U+000A LINE FEED (LF), U+000C FORM FEED (FF), U+000D CARRIAGE RETURN (CR), U+0020 SPACE, U+003E GREATER-THAN SIGN (>), or U+002F SOLIDUS (/)."
	// However, the rules for this vary with the parent element in subtle ways (eg <template> vs other element types).
	#[inline(always)]
	fn write_char_escaped(&mut self, character: char) -> io::Result<()>
	{
		match character
		{
			'&' => self.write_ampersand_escape(),
			
			'<' => self.write_all(b"&lt;"),
			
			_ => self.write_char(character),
		}
	}
	
	#[inline(always)]
	fn write_attribute_value_escaping_ampersand_and_single_quote(&mut self, attribute_value: &str) -> io::Result<()>
	{
		for character in attribute_value.chars()
		{
			match character
			{
				'&' => self.write_ampersand_escape()?,
				
				'\u{0027}' => self.write_apostrophe_escape()?,
				
				_ => self.write_char(character)?,
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn write_attribute_value_escaping_only_ampersand(&mut self, attribute_value: &str) -> io::Result<()>
	{
		for character in attribute_value.chars()
		{
			match character
			{
				'&' => self.write_ampersand_escape()?,
				
				_ => self.write_char(character)?,
			}
		}
		
		Ok(())
	}
	
	#[inline(always)]
	fn write_single_quote(&mut self) -> io::Result<()>
	{
		self.write_all(b"'")
	}
	
	#[inline(always)]
	fn write_double_quote(&mut self) -> io::Result<()>
	{
		self.write_all(b"\"")
	}
	
	#[inline(always)]
	fn write_ampersand_escape(&mut self) -> io::Result<()>
	{
		self.write_all(b"&amp;")
	}
	
	#[inline(always)]
	fn write_apostrophe_escape(&mut self) -> io::Result<()>
	{
		// Strictly speaking `&apos;` is more descriptive but `&#39;` is shorter
		self.write_all(b"&#39;")
	}
	
	#[inline(always)]
	fn write_char(&mut self, character: char) -> io::Result<()>
	{
		let mut buffer: [u8; 4] = unsafe { uninitialized() };
		character.encode_utf8(&mut buffer);
		
		self.write_all(&buffer[0 .. character.len_utf8()])
	}
	
	#[inline(always)]
	fn write_all_qualified_name(&mut self, name: &QualName) -> io::Result<()>
	{
		if let Some(ref prefix) = name.prefix
		{
			self.write_all_str(&prefix.deref().to_ascii_lowercase())?;
			self.write_all(b":")?;
		}
		self.write_all_str(&name.local.deref().to_ascii_lowercase())
	}
	
	#[inline(always)]
	fn write_all_deref<S: Deref<Target=str>>(&mut self, content: &S) -> io::Result<()>
	{
		self.write_all_str(content.deref())
	}
	
	#[inline(always)]
	fn write_all_str(&mut self, content: &str) -> io::Result<()>
	{
		self.write_all(content.as_bytes())
	}
	
	#[inline(always)]
	fn write_all(&mut self, content: &[u8]) -> io::Result<()>
	{
		self.writer.write_all(content)
	}
}
