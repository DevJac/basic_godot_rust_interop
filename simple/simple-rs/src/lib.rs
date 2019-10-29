mod bindings;

use std::ffi::{c_void, CString};
use std::mem::{size_of, MaybeUninit};
use std::os::raw::c_int;
use std::ptr;

static mut API: *const bindings::godot_gdnative_core_api_struct = ptr::null();
static mut NATIVESCRIPT_API: *const bindings::godot_gdnative_ext_nativescript_api_struct =
    ptr::null();

#[no_mangle]
pub unsafe extern "C" fn godot_gdnative_init(
    p_options: *const bindings::godot_gdnative_init_options,
) {
    API = (*p_options).api_struct;
    for i in 0..((*API).num_extensions as isize) {
        if let bindings::GDNATIVE_API_TYPES_GDNATIVE_EXT_NATIVESCRIPT =
            (**(*API).extensions.offset(i)).type_
        {
            NATIVESCRIPT_API = *(*API).extensions.offset(i)
                as *const bindings::godot_gdnative_ext_nativescript_api_struct;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn godot_gdnative_terminate(
    _p_options: *const bindings::godot_gdnative_terminate_options,
) {
    API = ptr::null();
    NATIVESCRIPT_API = ptr::null();
}

#[no_mangle]
pub unsafe extern "C" fn godot_nativescript_init(p_handle: *mut c_void) {
    let create = bindings::godot_instance_create_func {
        create_func: Some(simple_constructor),
        method_data: ptr::null_mut(),
        free_func: None,
    };
    let destroy = bindings::godot_instance_destroy_func {
        destroy_func: Some(simple_destructor),
        method_data: ptr::null_mut(),
        free_func: None,
    };
    let simple_cstr = CString::new("Simple").unwrap();
    let reference_cstr = CString::new("Reference").unwrap();
    (*NATIVESCRIPT_API)
        .godot_nativescript_register_class
        .unwrap()(
        p_handle,
        simple_cstr.as_ptr(),
        reference_cstr.as_ptr(),
        create,
        destroy,
    );
    let get_data = bindings::godot_instance_method {
        method: Some(simple_get_data),
        method_data: ptr::null_mut(),
        free_func: None,
    };
    let attributes = bindings::godot_method_attributes {
        rpc_type: bindings::godot_method_rpc_mode_GODOT_METHOD_RPC_MODE_DISABLED,
    };
    let get_data_cstr = CString::new("get_data").unwrap();
    (*NATIVESCRIPT_API)
        .godot_nativescript_register_method
        .unwrap()(
        p_handle,
        simple_cstr.as_ptr(),
        get_data_cstr.as_ptr(),
        attributes,
        get_data,
    );
}

#[repr(C)]
pub struct UserData {
    data: CString,
}

pub unsafe extern "C" fn simple_constructor(
    _p_instance: *mut bindings::godot_object,
    _p_method_data: *mut c_void,
) -> *mut c_void {
    let user_data = (*API).godot_alloc.unwrap()(size_of::<UserData>() as i32);
    (*(user_data as *mut UserData)).data = CString::new("Hello from Rust!\n").unwrap();
    user_data
}

pub unsafe extern "C" fn simple_destructor(
    _p_instance: *mut bindings::godot_object,
    _p_method_data: *mut c_void,
    p_user_data: *mut c_void,
) {
    (*API).godot_free.unwrap()(p_user_data);
}

pub unsafe extern "C" fn simple_get_data(
    _p_instance: *mut bindings::godot_object,
    _p_method_data: *mut c_void,
    p_user_data: *mut c_void,
    _p_num_args: c_int,
    _p_args: *mut *mut bindings::godot_variant,
) -> bindings::godot_variant {
    let user_data = p_user_data as *mut UserData;
    let mut data = MaybeUninit::<bindings::godot_string>::uninit();
    let mut ret = MaybeUninit::<bindings::godot_variant>::uninit();
    (*API).godot_string_new.unwrap()(data.as_mut_ptr());
    (*API).godot_string_parse_utf8.unwrap()(data.as_mut_ptr(), (*user_data).data.as_ptr());
    (*API).godot_variant_new_string.unwrap()(ret.as_mut_ptr(), data.as_mut_ptr());
    (*API).godot_string_destroy.unwrap()(data.as_mut_ptr());
    ret.assume_init()
}
