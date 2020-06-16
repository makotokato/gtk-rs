// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::GString;
use gtk_sys;
use std::cmp;
use std::fmt;
use std::mem;

glib_wrapper! {
    #[derive(Debug, Hash)]
    pub struct TreePath(Boxed<gtk_sys::GtkTreePath>);

    match fn {
        copy => |ptr| gtk_sys::gtk_tree_path_copy(mut_override(ptr)),
        free => |ptr| gtk_sys::gtk_tree_path_free(ptr),
        get_type => || gtk_sys::gtk_tree_path_get_type(),
    }
}

impl TreePath {
    pub fn new() -> TreePath {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_tree_path_new()) }
    }

    pub fn new_first() -> TreePath {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_tree_path_new_first()) }
    }

    //pub fn from_indices(first_index: i32, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> TreePath {
    //    unsafe { TODO: call gtk_sys:gtk_tree_path_new_from_indices() }
    //}

    pub fn from_indicesv(indices: &[i32]) -> TreePath {
        assert_initialized_main_thread!();
        let length = indices.len() as usize;
        unsafe {
            from_glib_full(gtk_sys::gtk_tree_path_new_from_indicesv(
                indices.to_glib_none().0,
                length,
            ))
        }
    }

    pub fn from_string(path: &str) -> TreePath {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_tree_path_new_from_string(
                path.to_glib_none().0,
            ))
        }
    }

    pub fn append_index(&mut self, index_: i32) {
        unsafe {
            gtk_sys::gtk_tree_path_append_index(self.to_glib_none_mut().0, index_);
        }
    }

    fn compare(&self, b: &TreePath) -> i32 {
        unsafe { gtk_sys::gtk_tree_path_compare(self.to_glib_none().0, b.to_glib_none().0) }
    }

    pub fn down(&mut self) {
        unsafe {
            gtk_sys::gtk_tree_path_down(self.to_glib_none_mut().0);
        }
    }

    pub fn get_depth(&self) -> i32 {
        unsafe { gtk_sys::gtk_tree_path_get_depth(mut_override(self.to_glib_none().0)) }
    }

    pub fn get_indices_with_depth(&mut self) -> Vec<i32> {
        unsafe {
            let mut depth = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                gtk_sys::gtk_tree_path_get_indices_with_depth(
                    self.to_glib_none_mut().0,
                    depth.as_mut_ptr(),
                ),
                depth.assume_init() as usize,
            );
            ret
        }
    }

    pub fn is_ancestor(&self, descendant: &TreePath) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_tree_path_is_ancestor(
                mut_override(self.to_glib_none().0),
                mut_override(descendant.to_glib_none().0),
            ))
        }
    }

    pub fn is_descendant(&self, ancestor: &TreePath) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_tree_path_is_descendant(
                mut_override(self.to_glib_none().0),
                mut_override(ancestor.to_glib_none().0),
            ))
        }
    }

    pub fn next(&mut self) {
        unsafe {
            gtk_sys::gtk_tree_path_next(self.to_glib_none_mut().0);
        }
    }

    pub fn prepend_index(&mut self, index_: i32) {
        unsafe {
            gtk_sys::gtk_tree_path_prepend_index(self.to_glib_none_mut().0, index_);
        }
    }

    pub fn prev(&mut self) -> bool {
        unsafe { from_glib(gtk_sys::gtk_tree_path_prev(self.to_glib_none_mut().0)) }
    }

    fn to_string(&self) -> GString {
        unsafe {
            from_glib_full(gtk_sys::gtk_tree_path_to_string(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    pub fn up(&mut self) -> bool {
        unsafe { from_glib(gtk_sys::gtk_tree_path_up(self.to_glib_none_mut().0)) }
    }
}

impl Default for TreePath {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for TreePath {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == 0
    }
}

impl Eq for TreePath {}

impl PartialOrd for TreePath {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.compare(other).partial_cmp(&0)
    }
}

impl Ord for TreePath {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.compare(other).cmp(&0)
    }
}

impl fmt::Display for TreePath {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
