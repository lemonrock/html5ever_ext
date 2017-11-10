// This file is part of css-purify. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css-purify. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css-purify/master/COPYRIGHT.


/// This trait adds methods for finding DOM nodes matching a CSS selector
pub trait Selectable
{
	/// Recursively find element nodes that match this selector.
	/// Return true from MatchUser to abort early.
	/// The result of this function is true if MatchUser aborted early, or false otherwise.
	#[inline]
	fn find_all_matching_child_nodes_depth_first_including_this_one<MatchUser: FnMut(&Rc<Node>) -> bool>(&self, selector: &OurSelector, match_user: &mut MatchUser) -> bool;
	
	/// Returns whether this element matches this selector.
	/// For the RcDom and &'a [RcDom], is currently always false.
	#[inline]
	fn matches(&self, selector: &OurSelector) -> bool;
}

impl<'a> Selectable for &'a [RcDom]
{
	#[inline]
	fn find_all_matching_child_nodes_depth_first_including_this_one<MatchUser: FnMut(&Rc<Node>) -> bool>(&self, selector: &OurSelector, match_user: &mut MatchUser) -> bool
	{
		for rc_dom in self.iter()
		{
			if rc_dom.find_all_matching_child_nodes_depth_first_including_this_one(selector, match_user)
			{
				return true;
			}
		}
		false
	}
	
	#[inline]
	fn matches(&self, selector: &OurSelector) -> bool
	{
		for rc_dom in self.iter()
		{
			if rc_dom.matches(selector)
			{
				return true;
			}
		}
		false
	}
}

impl Selectable for RcDom
{
	#[inline]
	fn find_all_matching_child_nodes_depth_first_including_this_one<MatchUser: FnMut(&Rc<Node>) -> bool>(&self, selector: &OurSelector, match_user: &mut MatchUser) -> bool
	{
		self.document.find_all_matching_child_nodes_depth_first_including_this_one(selector, match_user)
	}
	
	#[inline]
	fn matches(&self, selector: &OurSelector) -> bool
	{
		self.document.matches(selector)
	}
}

impl<'a> Selectable for Rc<Node>
{
	#[inline]
	fn find_all_matching_child_nodes_depth_first_including_this_one<MatchUser: FnMut(&Rc<Node>) -> bool>(&self, selector: &OurSelector, match_user: &mut MatchUser) -> bool
	{
		if self.matches(selector)
		{
			let should_finish = match_user(self);
			if should_finish
			{
				return true;
			}
		}
		
		for child in self.children.borrow().iter()
		{
			if child.find_all_matching_child_nodes_depth_first_including_this_one(selector, match_user)
			{
				return true;
			}
		}
		false
	}
	
	#[inline]
	fn matches(&self, selector: &OurSelector) -> bool
	{
		match self.data
		{
			NodeData::Element { .. } => matches(selector, &ElementNode
			{
				node: self.clone(),
			}),
			
			_ => false,
		}
	}
}
