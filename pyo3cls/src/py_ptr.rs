// Copyright (c) 2017-present PyO3 Project and Contributors

use syn;
use quote::Tokens;

pub fn build_ptr(cls: syn::Ident, ast: &mut syn::DeriveInput) -> Tokens {
    let ptr = &ast.ident;
    let dummy_const = syn::Ident::new(format!("_IMPL_PYO3_CLS_PTR_{}", ast.ident));

    quote! {
        #[feature(specialization)]
        #[allow(non_upper_case_globals, unused_attributes,
                unused_qualifications, unused_variables, non_camel_case_types)]
        const #dummy_const: () = {
            use std;
            extern crate pyo3 as _pyo3;

            // thread-safe, because any python related operations require a Python<'p> token.
            unsafe impl Send for #ptr {}
            unsafe impl Sync for #ptr {}

            impl _pyo3::Park<#cls> for #cls {
                type ParkTarget = #ptr;

                fn park(&self) -> #ptr {
                    let token = _pyo3::PyObjectWithToken::token(self);
                    let ptr = self.clone_ref(token).into_ptr();

                    #ptr(unsafe{_pyo3::PyPtr::from_owned_ptr(ptr)})
                }
                unsafe fn from_owned_ptr(ptr: *mut _pyo3::ffi::PyObject) -> #ptr {
                    std::mem::transmute(ptr)
                }
                unsafe fn from_borrowed_ptr(ptr: *mut _pyo3::ffi::PyObject) -> #ptr {
                    _pyo3::ffi::Py_INCREF(ptr);
                    std::mem::transmute(ptr)
                }
            }

            impl _pyo3::PythonPtr<#cls> for #ptr {

                #[inline]
                fn as_ref(&self, _py: Python) -> &#cls {
                    let offset = <#cls as _pyo3::typeob::PyTypeInfo>::offset();
                    unsafe {
                        let ptr = (self.as_ptr() as *mut u8).offset(offset) as *mut #cls;
                        ptr.as_ref().unwrap()
                    }
                }
                #[inline]
                fn as_mut(&self, _py: Python) -> &mut #cls {
                    let offset = <#cls as _pyo3::typeob::PyTypeInfo>::offset();
                    unsafe {
                        let ptr = (self.as_ptr() as *mut u8).offset(offset) as *mut #cls;
                        ptr.as_mut().unwrap()
                    }
                }
            }

            impl std::ops::Deref for #ptr {
                type Target = _pyo3::pointers::PyPtr;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl _pyo3::PyClone for #ptr {
                fn clone_ref(&self, py: _pyo3::Python) -> _pyo3::PyObject {
                    _pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr())
                }
            }
            impl _pyo3::PyClonePtr for #ptr {
                fn clone_ptr(&self, _py: _pyo3::Python) -> #ptr {
                    #ptr(unsafe{ _pyo3::PyPtr::from_borrowed_ptr(self.as_ptr()) })
                }
            }
            impl _pyo3::ToPyObject for #ptr {
                fn to_object(&self, py: Python) -> _pyo3::PyObject {
                    _pyo3::PyObject::from_borrowed_ptr(py, self.as_ptr())
                }
            }
            impl _pyo3::IntoPyObject for #ptr {
                fn into_object(self, _py: Python) -> _pyo3::PyObject {
                    unsafe {std::mem::transmute(self)}
                }
            }
            impl _pyo3::IntoPyPointer for #ptr {
                /// Gets the underlying FFI pointer, returns a owned pointer.
                #[inline]
                #[must_use]
                fn into_ptr(self) -> *mut ffi::PyObject {
                    self.0.into_ptr()
                }
            }
        };
    }
}
