//
// A rust binding for the GSL library by Guillaume Gomez (guillaume1.gomez@gmail.com)
//

use crate::{MatrixF64, Value, VectorF64};
use ffi::FFI;

pub struct MultilargeLinearType {
    inner: *const sys::gsl_multilarge_linear_type,
}

impl FFI<sys::gsl_multilarge_linear_type> for MultilargeLinearType {
    fn wrap(_r: *mut sys::gsl_multilarge_linear_type) -> Self {
        unimplemented!()
    }

    fn soft_wrap(_r: *mut sys::gsl_multilarge_linear_type) -> Self {
        unimplemented!()
    }

    fn unwrap_shared(&self) -> *const sys::gsl_multilarge_linear_type {
        self.inner
    }

    fn unwrap_unique(&mut self) -> *mut sys::gsl_multilarge_linear_type {
        unimplemented!()
    }
}

impl MultilargeLinearType {
    pub fn normal() -> MultilargeLinearType {
        unsafe {
            Self {
                inner: sys::gsl_multilarge_linear_normal,
            }
        }
    }

    pub fn tsqr() -> MultilargeLinearType {
        unsafe {
            Self {
                inner: sys::gsl_multilarge_linear_tsqr,
            }
        }
    }
}

pub struct MultilargeLinear {
    inner: *mut sys::gsl_multilarge_linear_workspace,
}

impl FFI<sys::gsl_multilarge_linear_workspace> for MultilargeLinear {
    fn wrap(inner: *mut sys::gsl_multilarge_linear_workspace) -> Self {
        Self { inner }
    }

    fn soft_wrap(r: *mut sys::gsl_multilarge_linear_workspace) -> Self {
        Self::wrap(r)
    }

    fn unwrap_shared(&self) -> *const sys::gsl_multilarge_linear_workspace {
        self.inner as *const _
    }

    fn unwrap_unique(&mut self) -> *mut sys::gsl_multilarge_linear_workspace {
        self.inner
    }
}

impl MultilargeLinear {
    pub fn alloc(t: MultilargeLinearType, p: usize) -> Option<MultilargeLinear> {
        let s = unsafe { sys::gsl_multilarge_linear_alloc(t.unwrap_shared(), p) };
        if s.is_null() {
            None
        } else {
            Some(Self { inner: s })
        }
    }

    pub fn name(&self) -> Option<String> {
        let n = unsafe { sys::gsl_multilarge_linear_name(self.unwrap_shared()) };
        if n.is_null() {
            return None;
        }
        let mut len = 0;
        loop {
            if unsafe { *n.offset(len) } == 0 {
                break;
            }
            len += 1;
        }
        let slice = unsafe { ::std::slice::from_raw_parts(n as _, len as _) };
        ::std::str::from_utf8(slice).ok().map(|x| x.to_owned())
    }

    pub fn reset(&mut self) -> Value {
        unsafe { Value::from(sys::gsl_multilarge_linear_reset(self.unwrap_unique())) }
    }

    pub fn accumulate(&mut self, x: &mut MatrixF64, y: &mut VectorF64) -> Value {
        unsafe {
            Value::from(sys::gsl_multilarge_linear_accumulate(
                x.unwrap_unique(),
                y.unwrap_unique(),
                self.unwrap_unique(),
            ))
        }
    }
}

impl Drop for MultilargeLinear {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                sys::gsl_multilarge_linear_free(self.inner);
            }
            self.inner = ::std::ptr::null_mut();
        }
    }
}
