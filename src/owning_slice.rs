//! Functions to produce an [`OwningSlice<'_, T>`][OwningSlice], a.k.a. a
//! `Vec<T, NoopAllocator<'_>>`, from mutably borrowed `MaybeUninit<T>`s.
//!
//! Note that there are no functions which take `&mut ManuallyDrop` here, even
//! as `unsafe fn`s, since `Vec` may use it's spare capacity in ways that
//! violate `ManuallyDrop<T>`'s validity invariants, not just it's safety
//! invariants.

use core::{marker::PhantomData, mem::MaybeUninit};

use crate::NoopAllocator;
use alloc::vec::Vec;

/// An owning slice reference boorrowing a memory location but owning the value
/// in it, implemented as `Vec<T, NoopAllocator<'a>>`.
pub type OwningSlice<'a, T> = Vec<T, NoopAllocator<'a>>;

/// Create a `OwningSlice<'a, T>` with a length and capacity of 1 from a `&'a
/// mut MaybeUninit<T>>`.
///
/// # Safety:
///
/// The `T` must be initialized, and dropping or removing the element from the
/// `OwningSlice` leaves the `MaybeUninit` semantically without a value, see
/// [`MaybeUninit::assume_init_mut`] and [`MaybeUninit::assume_init_drop`].
///
/// # Examples:
///
/// ```rust
/// # use std::mem::MaybeUninit;
/// use noop_allocator::owning_slice;
/// let mut buf: MaybeUninit<String> = MaybeUninit::new("Hello, world!".to_string());
/// let mut vec = unsafe { owning_slice::from_maybeuninit(&mut buf) };
/// assert_eq!(vec, ["Hello, world!"]);
/// assert_eq!(vec.capacity(), 1);
/// assert_eq!(vec.pop().as_deref(), Some("Hello, world!"));
/// vec.push("Hi!".to_string());
/// assert_eq!(vec, ["Hi!"]);
/// ```
pub unsafe fn from_maybeuninit<T>(slot: &mut MaybeUninit<T>) -> OwningSlice<'_, T> {
    Vec::from_raw_parts_in(
        slot as *mut MaybeUninit<T> as *mut T,
        1,
        1,
        NoopAllocator(PhantomData),
    )
}

/// Create a `OwningSlice<'a, T>` with a given length from a `&'a mut
/// [MaybeUninit<T>]>`. The capacity is the length of the given slice.
///
/// # Safety:
///
/// All slice elements in `[0..length]` must be initialized, see
/// [`MaybeUninit::assume_init_mut`] and [`MaybeUninit::assume_init_drop`].
///
/// # Examples:
///
/// ```rust
/// # use std::mem::MaybeUninit;
/// use noop_allocator::owning_slice;
/// let mut buf: [MaybeUninit<String>; 4] = [const { MaybeUninit::uninit() }; 4];
/// buf[0] = MaybeUninit::new("Hello, world!".to_string());
/// let mut vec = unsafe { owning_slice::from_maybeuninit_slice(&mut buf, 1) };
/// assert_eq!(vec, ["Hello, world!"]);
/// assert_eq!(vec.capacity(), 4);
/// assert_eq!(vec.pop().as_deref(), Some("Hello, world!"));
/// vec.push("Hi!".to_string());
/// assert_eq!(vec, ["Hi!"]);
/// ```
pub unsafe fn from_maybeuninit_slice<T>(
    slot: &mut [MaybeUninit<T>],
    length: usize,
) -> OwningSlice<'_, T> {
    debug_assert!(length <= slot.len());
    Vec::from_raw_parts_in(
        slot as *mut [MaybeUninit<T>] as *mut [T] as *mut T,
        length,
        slot.len(),
        NoopAllocator(PhantomData),
    )
}

/// Create a `OwningSlice<'a, T>` with a length of 0 and a capacity of 1 from a
/// `&'a mut MaybeUninit<T>>`.
///
/// The `T` is not assumed to be initialized, so this is not an `unsafe`
/// function.
///
/// # Examples:
///
/// ```rust
/// # use std::mem::MaybeUninit;
/// use noop_allocator::owning_slice;
/// let mut buf: MaybeUninit<String> = MaybeUninit::uninit();
/// let mut vec = owning_slice::empty_from_maybeuninit(&mut buf);
/// assert!(vec.is_empty());
/// assert_eq!(vec.capacity(), 1);
/// vec.push("Hello, world!".to_string());
/// assert_eq!(vec, ["Hello, world!"]);
/// ```
pub fn empty_from_maybeuninit<T>(slot: &mut MaybeUninit<T>) -> OwningSlice<'_, T> {
    unsafe {
        Vec::from_raw_parts_in(
            slot as *mut MaybeUninit<T> as *mut T,
            0,
            1,
            NoopAllocator(PhantomData),
        )
    }
}

/// Create a `OwningSlice<'a, T>` with a length of 0 from a `&'a mut
/// [MaybeUninit<T>]>`. The capacity is the length of the given slice.
///
/// The slice elements are not assumed to be initialized, so this is not an
/// `unsafe` function.
///
/// # Exapmples
///
/// ```rust
/// # use std::mem::MaybeUninit;
/// use noop_allocator::owning_slice;
/// let mut buf: [MaybeUninit<String>; 4] = [const { MaybeUninit::uninit() }; 4];
/// let mut vec = owning_slice::empty_from_maybeuninit_slice(&mut buf);
/// assert!(vec.is_empty());
/// assert_eq!(vec.capacity(), 4);
/// vec.push("Hello, world!".to_string());
/// assert_eq!(vec, ["Hello, world!"]);
/// ```
pub fn empty_from_maybeuninit_slice<T>(slot: &mut [MaybeUninit<T>]) -> OwningSlice<'_, T> {
    unsafe {
        Vec::from_raw_parts_in(
            slot as *mut [MaybeUninit<T>] as *mut [T] as *mut T,
            0,
            slot.len(),
            NoopAllocator(PhantomData),
        )
    }
}
