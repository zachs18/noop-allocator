# 0.1.1

* Add `owning_ref::from_maybeuninit_write` to safely create a `OwningRef<'_, T>` from a `&'_ mut MaybeUninit<T>` by writing a `T` into it.

# 0.1.0

Initial release.
