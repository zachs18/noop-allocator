//! Functions to produce an [`OwningRef<'_, T>`][OwningRef], a.k.a. a `Box<T,
//! NoopAllocator<'_>>`, from a mutably borrowed `MaybeUninit<T>` or
//! `ManuallyDrop<T>`.
use core::{
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
};

use crate::NoopAllocator;
use alloc::boxed::Box;

/// An owning reference boorrowing a memory location but owning the value in it,
/// implemented as `Box<T, NoopAllocator<'a>>`.
pub type OwningRef<'a, T> = Box<T, NoopAllocator<'a>>;

/// Create a `OwningRef<'a, T>` from a `&'a mut ManuallyDrop<T>>`.
///
/// # Safety:
///
/// Dropping or moving out of the `OwningRef` leaves the borrowed `ManuallyDrop`
/// semantically without a value; see [`ManuallyDrop::drop`] and
/// [`ManuallyDrop::take`].
///
/// You must not use the `Box` in such a way that would violate the vailidity
/// invariant of the `ManuallyDrop<T>`.
pub unsafe fn from_manuallydrop<T: ?Sized>(slot: &mut ManuallyDrop<T>) -> OwningRef<'_, T> {
    Box::from_raw_in(
        slot as *mut ManuallyDrop<T> as *mut T,
        NoopAllocator(PhantomData),
    )
}

/// Create a `OwningRef<'a, T>` from a `&'a mut MaybeUninit<T>>`.
///
/// # Safety:
///
/// The `T` must be initialized, see [`MaybeUninit::assume_init_mut`] and
/// [`MaybeUninit::assume_init_drop`].
pub unsafe fn from_maybeuninit<T>(slot: &mut MaybeUninit<T>) -> OwningRef<'_, T> {
    Box::from_raw_in(
        slot as *mut MaybeUninit<T> as *mut T,
        NoopAllocator(PhantomData),
    )
}

/// Create a `OwningRef<'a, T>` from a `&'a mut MaybeUninit<T>>` by writing a
/// value into it.
///
/// The `MaybeUninit<T>` will be overwritten with `value`.
pub fn from_maybeuninit_write<T>(slot: &mut MaybeUninit<T>, value: T) -> OwningRef<'_, T> {
    slot.write(value);
    unsafe {
        Box::from_raw_in(
            slot as *mut MaybeUninit<T> as *mut T,
            NoopAllocator(PhantomData),
        )
    }
}

/// Create a `OwningRef<'a, [T]>` from a `&'a mut [MaybeUninit<T>]>`.
///
/// # Safety:
///
/// All slice elements must be initialized, see [`MaybeUninit::assume_init_mut`]
/// and [`MaybeUninit::assume_init_drop`].
pub unsafe fn from_maybeuninit_slice<T>(slot: &mut [MaybeUninit<T>]) -> OwningRef<'_, [T]> {
    Box::from_raw_in(
        slot as *mut [MaybeUninit<T>] as *mut [T],
        NoopAllocator(PhantomData),
    )
}
