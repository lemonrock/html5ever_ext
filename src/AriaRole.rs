// This file is part of html5ever_ext. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of html5ever_ext. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/html5ever_ext/master/COPYRIGHT.


//noinspection SpellCheckingInspection
/// Valid values of Aria `role` global attribute
/// See [Aria Roles 101](http://www.webteacher.ws/2010/10/14/aria-roles-101/) for more.
/// Navigation roles are probably the most useful:-
/// - application
/// - banner
/// - complementary
/// - contentinfo
/// - form
/// - main
/// - navigation
/// - search
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum AriaRole
{
	/// "alert"
	alert,
	
	/// "alertdialog"
	alertdialog,
	
	/// "button"
	button,
	
	/// "checkbox"
	checkbox,
	
	/// "dialog"
	dialog,
	
	/// "gridcell"
	gridcell,
	
	/// "link"
	link,
	
	/// "log"
	log,
	
	/// "marquee"
	marquee,
	
	/// "menuitem"
	menuitem,
	
	/// "menuitemcheckbox"
	menuitemcheckbox,
	
	/// "menuitemradio"
	menuitemradio,
	
	/// "option"
	option,
	
	/// "progressbar"
	progressbar,
	
	/// "radio"
	radio,
	
	/// "scrollbar"
	scrollbar,
	
	/// "slider"
	slider,
	
	/// "spinbutton"
	spinbutton,
	
	/// "status"
	status,
	
	/// "tab"
	tab,
	
	/// "tabpanel"
	tabpanel,
	
	/// "textbox"
	textbox,
	
	/// "timer"
	timer,
	
	/// "tooltip"
	tooltip,
	
	/// "treeitem"
	treeitem,
	
	/// "combobox"
	combobox,
	
	/// "grid"
	grid,
	
	/// "listbox"
	listbox,
	
	/// "menu"
	menu,
	
	/// "menubar"
	menubar,
	
	/// "radiogroup"
	radiogroup,
	
	/// "tablist"
	tablist,
	
	/// "tree"
	tree,
	
	/// "treegrid"
	treegrid,
	
	/// "article"
	article,
	
	/// "columnheader"
	columnheader,
	
	/// "definition"
	definition,
	
	/// "directory"
	directory,
	
	/// "document"
	document,
	
	/// "group"
	group,
	
	/// "heading"
	heading,
	
	/// "img"
	img,
	
	/// "list"
	list,
	
	/// "listitem"
	listitem,
	
	/// "math"
	math,
	
	/// "note"
	note,
	
	/// "presentation"
	presentation,
	
	/// "region"
	region,
	
	/// "row"
	row,
	
	/// "rowheader"
	rowheader,
	
	/// "separator"
	separator,
	
	/// "toolbar"
	toolbar,
	
	/// "application"
	application,
	
	/// "banner"
	banner,
	
	/// "complementary"
	complementary,
	
	/// "contentinfo"
	contentinfo,
	
	/// "form"
	form,
	
	/// "main"
	main,
	
	/// "navigation"
	navigation,
	
	/// "search"
	search,
}

impl AriaRole
{
	//noinspection SpellCheckingInspection
	#[inline(always)]
	fn to_str(&self) -> &'static str
	{
		use self::AriaRole::*;
		
		match *self
		{
			alert => "alert",
			
			alertdialog => "alertdialog",
			
			button => "button",
			
			checkbox => "checkbox",
			
			dialog => "dialog",
			
			gridcell => "gridcell",
			
			link => "link",
			
			log => "log",
			
			marquee => "marquee",
			
			menuitem => "menuitem",
			
			menuitemcheckbox => "menuitemcheckbox",
			
			menuitemradio => "menuitemradio",
			
			option => "option",
			
			progressbar => "progressbar",
			
			radio => "radio",
			
			scrollbar => "scrollbar",
			
			slider => "slider",
			
			spinbutton => "spinbutton",
			
			status => "status",
			
			tab => "tab",
			
			tabpanel => "tabpanel",
			
			textbox => "textbox",
			
			timer => "timer",
			
			tooltip => "tooltip",
			
			treeitem => "treeitem",
			
			combobox => "combobox",
			
			grid => "grid",
			
			listbox => "listbox",
			
			menu => "menu",
			
			menubar => "menubar",
			
			radiogroup => "radiogroup",
			
			tablist => "tablist",
			
			tree => "tree",
			
			treegrid => "treegrid",
			
			article => "article",
			
			columnheader => "columnheader",
			
			definition => "definition",
			
			directory => "directory",
			
			document => "document",
			
			group => "group",
			
			heading => "heading",
			
			img => "img",
			
			list => "list",
			
			listitem => "listitem",
			
			math => "math",
			
			note => "note",
			
			presentation => "presentation",
			
			region => "region",
			
			row => "row",
			
			rowheader => "rowheader",
			
			separator => "separator",
			
			toolbar => "toolbar",
			
			application => "application",
			
			banner => "banner",
			
			complementary => "complementary",
			
			contentinfo => "contentinfo",
			
			form => "form",
			
			main => "main",
			
			navigation => "navigation",
			
			search => "search",
		}
	}
}
