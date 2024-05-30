#![no_std]
#![feature(allocator_api)]
#![feature(alloc_layout_extra)]
#![warn(rust_2018_idioms)]

#[cfg(feature = "alloc")]
extern crate alloc;
use core::{
    alloc::{AllocError, Allocator, Layout},
    marker::PhantomData,
    ptr::NonNull,
};

/// An [`Allocator`] that does nothing.
///
/// Specifically:
/// * [`allocate`][NoopAllocator::allocate] and
///   [`allocate_zeroed`][NoopAllocator::allocate_zeroed] will return `Err` for
///   any non-zero-sized allocation requests
/// * [`deallocate`][NoopAllocator::deallocate] is a no-op, and does not require
///   that `ptr` be "currently allocated", or fit `layout`.
/// * [`shrink`][NoopAllocator::shrink], [`grow`][NoopAllocator::grow], and
///   [`grow_zeroed`][NoopAllocator::grow_zeroed] do not require that `ptr` be
///   "currently allocated", or fit `old_layout`, and will successfully return
///   the original pointer unchanged (with the length of the new layout) if the
///   `ptr` is aligned for the new layout and the new layout is smaller or the
///   same size as the old layout.
///
/// This type is usable as an [`Allocator`] when you want to borrow an existing
/// memory range for use in a single-allocation collection type, for example in
/// [`Box`][alloc::boxed::Box] or [`Vec`][alloc::vec::Vec].
///
/// # Safety:
///
/// Many functions in this crate assume that `impl Allocator for
/// NoopAllocator<'_>` as described above is sound, but `feature(allocator_api)`
/// is unstable and the preconditions may change.
#[repr(transparent)]
pub struct NoopAllocator<'a>(PhantomData<&'a ()>);

impl<'a> NoopAllocator<'a> {
    /// Creates a new `NoopAllocator<'a>`.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

unsafe impl Allocator for NoopAllocator<'_> {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        if layout.size() == 0 {
            Ok(NonNull::slice_from_raw_parts(layout.dangling(), 0))
        } else {
            Err(AllocError)
        }
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        // intentionally empty
    }

    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        self.grow_zeroed(ptr, old_layout, new_layout)
    }

    unsafe fn grow_zeroed(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() >= old_layout.size(),
            "`new_layout.size()` must be greater than or equal to `old_layout.size()`"
        );
        if new_layout.size() > old_layout.size()
            || (ptr.as_ptr() as usize & (new_layout.align() - 1) != 0)
        {
            return Err(AllocError);
        }

        let new_ptr = NonNull::slice_from_raw_parts(ptr, new_layout.size());

        Ok(new_ptr)
    }

    unsafe fn shrink(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        debug_assert!(
            new_layout.size() <= old_layout.size(),
            "`new_layout.size()` must be smaller than or equal to `old_layout.size()`"
        );

        if new_layout.size() > old_layout.size()
            || (ptr.as_ptr() as usize & (new_layout.align() - 1) != 0)
        {
            return Err(AllocError);
        }

        let new_ptr = NonNull::slice_from_raw_parts(ptr, new_layout.size());

        Ok(new_ptr)
    }
}

#[cfg(feature = "alloc")]
pub mod owning_ref;
#[cfg(feature = "alloc")]
pub mod owning_slice;
