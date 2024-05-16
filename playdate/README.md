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
- `playdate->system->isCrankDocked()` is implemented as `crank_state()`
- `playdate->system->setAutoLockDisabled()` renamed to `set_auto_lock_enabled()`
- `playdate->system->setCrankSoundsDisabled()` renamed to
  `set_crank_sounds_enabled()`
- `playdate->system->getMenuItemUserData()` and
  `playdate->system->setMenuItemUserData()` are not implemented. To use external
  data with menu items, pass values into the
  provided menu item closure.
- `playdate->system->removeAllMenuItems()` not implemented
