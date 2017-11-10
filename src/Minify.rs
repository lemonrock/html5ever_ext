// This file is part of html5ever_ext. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of html5ever_ext. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT.


/// Minifies and serializes a html5ever HTML DOM (RcDom) or node (Rc<Node>, aka Handle).
pub trait Minify
{
	/// Minifies and serializes an instance of an HTML DOM to file.
	/// If creating AMP pages, set `html_head_and_body_tags_are_optional` to false.
	/// Wraps use of `UltraMinifyingHtmlSerializer`.
	/// If you need to serialize multiple RcDom or Node objects to the same writer, consider using `UltraMinifyingHtmlSerializer` directly.
	#[inline(always)]
	fn minify_to_file_path<P: AsRef<Path>>(&self, html_head_and_body_tags_are_optional: bool, html_file_path: P) -> Result<(), HtmlError>;
	
	/// Minifies and serializes an instance of an HTML DOM to a vector of bytes.
	/// If creating AMP pages, set `html_head_and_body_tags_are_optional` to false.
	/// Wraps use of `UltraMinifyingHtmlSerializer`.
	/// If you need to serialize multiple RcDom or Node objects to the same writer, consider using `UltraMinifyingHtmlSerializer` directly.
	#[inline(always)]
	fn minify_to_bytes(&self, html_head_and_body_tags_are_optional: bool) -> io::Result<Vec<u8>>;
	
	/// Minifies and serializes an instance of an HTML DOM to a writer.
	/// If creating AMP pages, set `html_head_and_body_tags_are_optional` to false.
	/// Wraps use of `UltraMinifyingHtmlSerializer`.
	/// If you need to serialize multiple RcDom or Node objects to the same writer, consider using `UltraMinifyingHtmlSerializer` directly.
	#[inline(always)]
	fn minify_to_writer<W: Write>(&self, html_head_and_body_tags_are_optional: bool, writer: W) -> io::Result<()>;
}

impl Minify for RcDom
{
	#[inline(always)]
	fn minify_to_file_path<P: AsRef<Path>>(&self, html_head_and_body_tags_are_optional: bool, html_file_path: P) -> Result<(), HtmlError>
	{
		self.document.minify_to_file_path(html_head_and_body_tags_are_optional, html_file_path)
	}
	
	#[inline(always)]
	fn minify_to_bytes(&self, html_head_and_body_tags_are_optional: bool) -> io::Result<Vec<u8>>
	{
		self.document.minify_to_bytes(html_head_and_body_tags_are_optional)
	}
	
	#[inline(always)]
	fn minify_to_writer<W: Write>(&self, html_head_and_body_tags_are_optional: bool, writer: W) -> io::Result<()>
	{
		UltraMinifyingHtmlSerializer::new(html_head_and_body_tags_are_optional, writer).serialize_rc_dom(self)
	}
}

impl Minify for Rc<Node>
{
	#[inline(always)]
	fn minify_to_file_path<P: AsRef<Path>>(&self, html_head_and_body_tags_are_optional: bool, html_file_path: P) -> Result<(), HtmlError>
	{
		use ::std::fs::File;
		
		let path = html_file_path.as_ref();
		
		let file = File::create(path).context(path)?;
		
		self.minify_to_writer(html_head_and_body_tags_are_optional, file).context(path)?;
		
		Ok(())
	}
	
	#[inline(always)]
	fn minify_to_bytes(&self, html_head_and_body_tags_are_optional: bool) -> io::Result<Vec<u8>>
	{
		let mut bytes = Vec::new();
		
		self.minify_to_writer(html_head_and_body_tags_are_optional, &mut bytes)?;
		
		Ok(bytes)
	}
	
	#[inline(always)]
	fn minify_to_writer<W: Write>(&self, html_head_and_body_tags_are_optional: bool, writer: W) -> io::Result<()>
	{
		UltraMinifyingHtmlSerializer::new(html_head_and_body_tags_are_optional, writer).serialize_node(self, true)
	}
}
