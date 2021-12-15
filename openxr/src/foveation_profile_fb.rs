use std::sync::Arc;

use crate::*;

pub struct FoveationProfileFB {
    inner: Arc<sys::FoveationProfileFB>,
}

impl FoveationProfileFB {
    #[inline]
    pub unsafe fn from_raw(handle: sys::FoveationProfileFB) -> Self {
        Self {
            inner: Arc::new(handle),
        }
    }

    #[inline]
    pub fn as_raw(&self) -> sys::FoveationProfileFB {
        *self.inner
    }
}
