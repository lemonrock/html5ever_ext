// This file is part of html5ever_ext. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT. No part of html5ever_ext, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of html5ever_ext. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT.


#![warn(missing_docs)]


//! # html5_ext
//!
//! This is set of unofficial extensions to the [html5ever](https://github.com/servo/html5ever) crate's RcDom and Node structs, including a ***minifying HTML5 serializer*** and support for ***CSS matching**.
//!
//! It re-exports the `css` and `html5ever` crates, and useful DOM types hidden inside the `::html5ever::markup5ever::rcdom` module.
//!
//!
//! ## How Tos
//!
//!
//! ### To load and minify HTML5
//!
//! ```
//! extern crate html5_ext;
//! use ::html5ever_ext::RcDom;
//! use ::html5ever_ext::RcDomExt;
//! use ::html5ever_ext::Minify;
//!
//! let rc_dom = RcDom::from_file_path_verified_and_stripped_of_comments_and_processing_instructions_and_with_a_sane_doc_type("/path/to/document.html").expect("invalid HTML");
//! rc_dom.minify_to_file_path();
//! ```
//!
//! There are additional methods available on `Minify` to minify to a byte array or a generic `Write`-implementing writer.
//!
//! For more control, eg over serializing multiple node graphs, use the struct `UltraMinifyingHtmlSerializer` directly.
//!
//!
//! ### To match CSS selectors
//!
//! ```
//! extern crate html5_ext;
//! use ::html5ever_ext::RcDom;
//! use ::html5ever_ext::RcDomExt;
//! use ::html5ever_ext::parse_css_selector;
//! use ::html5ever_ext::Selectable;
//! use ::html5ever_ext::NodeExt;
//!
//! let rc_dom = RcDom::from_file_path_verified_and_stripped_of_comments_and_processing_instructions_and_with_a_sane_doc_type("/path/to/document.html").expect("invalid HTML");
//!
//! let selector = parse_css_selector("p.myclass").unwrap();
//!
//! assert!(!rc_dom.matches(&selector));
//!
//! rc_dom.find_all_matching_child_nodes_depth_first_including_this_one(&selector, |node|
//! {
//! 	// Done this way because Rc<Node> does not implement Debug, but NodeExt implement debug_fmt() which is identical as possible.
//! 	let mut debug = String::new();
//! 	node.debug_fmt(&mut debug).unwrap();
//! 	write!("Found node {}", &debug);
//! })
//! ```
//!
//! ### To work with Nodes
//!
//! Use the `NodeExt`, `Minify`, `Selectable` and `QualNameExt` traits.
//!


pub extern crate css;
#[macro_use] pub extern crate html5ever;
#[macro_use] extern crate quick_error;


pub use ::css::parse_css_selector;
use ::css::domain::atRules::namespace::NamespaceUrl;
use ::css::domain::selectors::matches;
use ::css::domain::selectors::OurSelector;
use ::css::domain::selectors::OurSelectorImpl;
use ::css::selectors::Element;
use ::css::selectors::OpaqueElement;
use ::css::selectors::SelectorImpl;
use ::css::selectors::attr::AttrSelectorOperation;
use ::css::selectors::attr::CaseSensitivity;
use ::css::selectors::attr::NamespaceConstraint;
use ::css::selectors::attr::SELECTOR_WHITESPACE;
use ::css::selectors::matching::ElementSelectorFlags;
use ::css::selectors::matching::LocalMatchingContext;
use ::css::selectors::matching::MatchingContext;
use ::css::selectors::matching::RelevantLinkStatus;
pub use ::html5ever::Attribute;
pub use ::html5ever::LocalName;
pub use ::html5ever::interface::QualName;
use ::html5ever::tendril::NonAtomic;
use ::html5ever::tendril::Tendril;
use ::html5ever::tendril::fmt::UTF8;
pub use ::html5ever::rcdom::Node;
pub use ::html5ever::rcdom::NodeData;
use ::html5ever::rcdom::NodeData::*;
pub use ::html5ever::rcdom::RcDom;
use ::quick_error::ResultExt;
use ::std::ascii::AsciiExt;
use ::std::cell::Cell;
use ::std::cell::RefCell;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::io;
use ::std::io::Write;
use ::std::mem::uninitialized;
use ::std::ops::Deref;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::rc::Rc;


include!("Selectable.rs");
include!("ElementNode.rs");
include!("HtmlError.rs");
include!("Minify.rs");
include!("NodeExt.rs");
include!("QualNameExt.rs");
include!("RcDomExt.rs");
include!("UltraMinifyingHtmlSerializer.rs");
