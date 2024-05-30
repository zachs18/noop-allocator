# `noop_allocator`

This crate provides an `Allocator` which does nothing: `NoopAllocator<'_>`.

Specifically:
* [`allocate`] and [`allocate_zeroed`] will return `Err` for any
  non-zero-sized allocation requests
* [`deallocate`] is a no-op, and does not require that `ptr` be "currently
  allocated", or fit `layout`.
* [`shrink`], [`grow`], and [`grow_zeroed`] do not require that `ptr` be
  "currently allocated", or fit `old_layout`, and will successfully return
  the original pointer unchanged (with the length of the new layout) if the
  `ptr` is aligned for the new layout and the new layout is smaller or the
  same size as the old layout.

This type is usable as an [`Allocator`] when you want to borrow an existing
memory range for use in a collection type, for example in
[`Box`][alloc::boxed::Box] or [`Vec`][alloc::vec::Vec].

# Safety:

Many functions in this crate assume that `impl Allocator for
NoopAllocator<'_>` as described above is sound, but `feature(allocator_api)`
is unstable and the preconditions may change.
