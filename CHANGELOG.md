# 0.1.4

* Update to `alloc_layout_extra` stabilization and rename.

# 0.1.3

* Add `owning_ref::new` wrapping `Vec::new_in` to create an empty `OwningSlice` with no capacity.

# 0.1.2

* Add `owning_ref::from_raw` to unsafely crate an `OwningRef<'_, T>` from a raw pointer.
* Add `owning_slice::{from_raw, empty_from_raw, full_from_raw}` to unsafely crate an `OwningSlice<'_, T>` from a raw slice pointer.
* Add a `Default` implementation for `NoopAllocator`.

# 0.1.1

* Add `owning_ref::from_maybeuninit_write` to safely create a `OwningRef<'_, T>` from a `&'_ mut MaybeUninit<T>` by writing a `T` into it.

# 0.1.0

Initial release.
