# Playdate

Rust wrapper around raw bindgen wrappers for C code

## Usage

Create a new API with `Playdate::new()`. If you're using `playdate_init`, this
is done for you via the `pd_init` macro.

## Differences to the C API

- `playdate->system->realloc()` not implemented - use `core::alloc` functions
  instead.
- functions starting with `get_` omit the get prefix to be more idiomatic Rust.
  For example, `get_menu_item_title()` is instead `menu_item_title()`.
- `playdate->system->setUpdateCallback()` is not implemented
