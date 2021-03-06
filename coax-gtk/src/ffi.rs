use std::ffi::{CStr, CString};
use std::ptr;
use glib::translate::ToGlibPtr;
use glib_sys::{gpointer, g_markup_escape_text};
use gtk;
use gtk_sys::{self, GtkListBoxRow};
use gobject_sys::{GObject, g_object_set_data_full, g_object_get_data};
use libc::{c_char, c_int, ssize_t};

lazy_static! {
    pub static ref KEY_ID: CString = CString::new("id").unwrap();
    pub static ref TSTAMP: CString = CString::new("tstamp").unwrap();
}

extern fn drop_box(p: gpointer) {
    unsafe {
        Box::from_raw(p);
    }
}

pub fn set_data<'a, T, A>(obj: &'a T, k: &CStr, a: A)
    where T: ToGlibPtr<'a, *mut GObject>
{
    let data  = Box::into_raw(Box::new(a)) as gpointer;
    let stash = obj.to_glib_none();
    unsafe {
        g_object_set_data_full(stash.0, k.as_ptr(), data, Some(drop_box))
    }
}

pub fn get_data<'a, T, A>(obj: &'a T, k: &CStr) -> Option<&'a A>
    where T: ToGlibPtr<'a, *mut GObject>
{
    let stash = obj.to_glib_none();
    unsafe {
        let ptr = g_object_get_data(stash.0, k.as_ptr()) as *mut A;
        ptr.as_ref()
    }
}

pub fn escape<'a>(s: &str) -> &'a CStr {
    unsafe {
        let ptr = g_markup_escape_text(s.as_ptr() as *const c_char, s.len() as ssize_t);
        CStr::from_ptr(ptr)
    }
}

unsafe extern fn cmp_rows_by_time(a: *mut GtkListBoxRow, b: *mut GtkListBoxRow, _: gpointer) -> c_int {
    let a_time_ptr = g_object_get_data(a as *mut GObject, TSTAMP.as_ptr()) as *mut i64;
    let b_time_ptr = g_object_get_data(b as *mut GObject, TSTAMP.as_ptr()) as *mut i64;
    match (a_time_ptr.as_ref(), b_time_ptr.as_ref()) {
        (Some(ta), Some(tb)) => (tb - ta) as c_int,
        _                    => 0
    }
}

pub fn set_sort_by_time(r: &gtk::ListBox) {
    let stash = r.to_glib_none();
    unsafe {
        gtk_sys::gtk_list_box_set_sort_func(stash.0, Some(cmp_rows_by_time), ptr::null_mut(), None)
    }
}

