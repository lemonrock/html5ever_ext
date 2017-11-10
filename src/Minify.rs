// This file is part of html5ever_ext. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of html5ever_ext. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT.


/// Minifies and serializes a html5ever HTML DOM (RcDom) or node (Rc<Node>, aka Handle).
pub trait Minify
{
	#[doc(hidden)]
	const PRESERVE_COMMENTS: bool = false;
	
	#[doc(hidden)]
	const PRESERVE_PROCESSING_INSTRUCTIONS: bool = false;
	
	#[doc(hidden)]
	const COLLAPSE_WHITESPACE: bool = true;
	
	/// Identical to impl Debug's fmt() method, except we can't impl Debug for a trait and struct we don't own in this crate.
	#[inline(always)]
	fn debug_fmt<W: fmt::Write>(&self, f: &mut W) -> fmt::Result;
	
	/// A debug string representing this node and any children.
	#[inline(always)]
	fn debug_string(&self) -> String
	{
		let mut debug = String::new();
		self.debug_fmt(&mut debug).unwrap();
		debug
	}
	
	/// Minifies and serializes an instance of an HTML DOM to file.
	/// `only_serialize_children` should be true only if you do not want to serialize '&self', eg `<main><p>...</p>><p>...</p></main>` with `only_serialize_children` set to true would serialize `<p>...</p>><p>...</p>`.
	/// Note that this has no effect on a RcDom - it will still serialize an entire document, including a DOCTYPE, etc.
	/// If creating AMP pages, set `html_head_and_body_tags_are_optional` to false.
	/// If you need to serialize multiple RcDom or Node objects to the same writer, or need more control, consider using `UltraMinifyingHtmlSerializer`.
	#[inline(always)]
	fn minify_to_file_path<P: AsRef<Path>>(&self, html_head_and_body_tags_are_optional: bool, html_file_path: P) -> Result<(), HtmlError>
	{
		use ::std::fs::File;
		
		let path = html_file_path.as_ref();
		
		let file = File::create(path).context(path)?;
		
		self.minify_to_writer(html_head_and_body_tags_are_optional, file).context(path)?;
		
		Ok(())
	}
	
	/// Minifies and serializes an instance of an HTML DOM to String.
	/// `only_serialize_children` should be true only if you do not want to serialize '&self', eg `<main><p>...</p>><p>...</p></main>` with `only_serialize_children` set to true would serialize `<p>...</p>><p>...</p>`.
	/// Note that this has no effect on a RcDom - it will still serialize an entire document, including a DOCTYPE, etc.
	/// If creating AMP pages, set `html_head_and_body_tags_are_optional` to false.
	/// If you need to serialize multiple RcDom or Node objects to the same writer, or need more control, consider using `UltraMinifyingHtmlSerializer`.
	#[inline(always)]
	fn minify_to_string(&self, html_head_and_body_tags_are_optional: bool) -> String
	{
		let bytes = self.minify_to_bytes(html_head_and_body_tags_are_optional);
		String::from_utf8(bytes).unwrap()
	}
	
	/// Minifies and serializes an instance of an HTML DOM to a vector of bytes.
	/// `only_serialize_children` should be true only if you do not want to serialize '&self', eg `<main><p>...</p>><p>...</p></main>` with `only_serialize_children` set to true would serialize `<p>...</p>><p>...</p>`.
	/// Note that this has no effect on a RcDom - it will still serialize an entire document, including a DOCTYPE, etc.
	/// If creating AMP pages, set `html_head_and_body_tags_are_optional` to false.
	/// If you need to serialize multiple RcDom or Node objects to the same writer, or need more control, consider using `UltraMinifyingHtmlSerializer`.
	#[inline(always)]
	fn minify_to_bytes(&self, html_head_and_body_tags_are_optional: bool) -> Vec<u8>
	{
		let mut bytes = Vec::new();
		
		self.minify_to_writer(html_head_and_body_tags_are_optional, &mut bytes).unwrap();
		
		bytes
	}
	
	/// Minifies and serializes an instance of an HTML DOM to a writer.
	/// `only_serialize_children` should be true only if you do not want to serialize '&self', eg `<main><p>...</p>><p>...</p></main>` with `only_serialize_children` set to true would serialize `<p>...</p>><p>...</p>`.
	/// Note that this has no effect on a RcDom - it will still serialize an entire document, including a DOCTYPE, etc.
	/// If creating AMP pages, set `html_head_and_body_tags_are_optional` to false.
	/// If you need to serialize multiple RcDom or Node objects to the same writer, or need more control, consider using `UltraMinifyingHtmlSerializer`.
	#[inline(always)]
	fn minify_to_writer<W: Write>(&self, html_head_and_body_tags_are_optional: bool, writer: W) -> io::Result<()>;
	
	#[doc(hidden)]
	#[inline(always)]
	fn _serializer<W: Write>(html_head_and_body_tags_are_optional: bool, writer: W) -> UltraMinifyingHtmlSerializer<W>
	{
		UltraMinifyingHtmlSerializer::new(html_head_and_body_tags_are_optional, Self::PRESERVE_COMMENTS, Self::PRESERVE_PROCESSING_INSTRUCTIONS, writer)
	}
}

impl Minify for RcDom
{
	#[inline(always)]
	fn debug_fmt<W: fmt::Write>(&self, f: &mut W) -> fmt::Result
	{
		self.document.debug_fmt(f)
	}
	
	#[inline(always)]
	fn minify_to_writer<W: Write>(&self, html_head_and_body_tags_are_optional: bool, writer: W) -> io::Result<()>
	{
		self.document.minify_to_writer(html_head_and_body_tags_are_optional, writer)
	}
}

impl Minify for Rc<Node>
{
	#[inline(always)]
	fn debug_fmt<W: fmt::Write>(&self, f: &mut W) -> fmt::Result
	{
		write!(f, "{}", self.minify_to_string(true))
	}
	
	#[inline(always)]
	fn minify_to_writer<W: Write>(&self, html_head_and_body_tags_are_optional: bool, writer: W) -> io::Result<()>
	{
		Self::_serializer(html_head_and_body_tags_are_optional, writer).serialize_node(self, Self::COLLAPSE_WHITESPACE, true)
	}
}

/// Use this to minify on the children of a Rc<Node>, eg node.children.debug_string()
impl Minify for RefCell<Vec<Rc<Node>>>
{
	#[inline(always)]
	fn debug_fmt<W: fmt::Write>(&self, f: &mut W) -> fmt::Result
	{
		for node in self.borrow().iter()
		{
			node.debug_fmt(f)?;
		}
		Ok(())
	}
	
	#[inline(always)]
	fn minify_to_writer<W: Write>(&self, html_head_and_body_tags_are_optional: bool, mut writer: W) -> io::Result<()>
	{
		{
			let mut serializer = Self::_serializer(html_head_and_body_tags_are_optional, &mut writer);
			
			for node in self.borrow().iter()
			{
				serializer.serialize_node(node, Self::COLLAPSE_WHITESPACE, false)?;
			}
		}
		
		writer.flush()
	}
}
