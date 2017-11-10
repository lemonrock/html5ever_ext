// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


quick_error!
{
	/// Represents errors that can happen within loading or minifying HTML.
	#[derive(Debug)]
	pub enum HtmlError
	{
		/// An input-output error when processing a HTML file.
		Io(path: PathBuf, cause: ::std::io::Error)
		{
			cause(cause)
			description(cause.description())
			display("I/O error with {:?} was '{}'", path, cause)
			context(path: &'a Path, cause: ::std::io::Error) -> (path.to_path_buf(), cause)
		}
		
		/// A HTML file, when processed, is invalid according to `reason`.
		InvalidFile(path: PathBuf, reason: String)
		{
			description(&reason)
			display("The file {:?} can not be used because: {}", path, reason)
		}
	}
}
