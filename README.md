# dirty_const

[![Build Status](https://travis-ci.org/mistodon/dirty_const.svg?branch=master)](https://travis-ci.org/mistodon/dirty_const)
[![Crates.io](https://img.shields.io/crates/v/dirty_const.svg)](https://crates.io/crates/dirty_const)
[![Docs.rs](https://docs.rs/resource/badge.svg)](https://docs.rs/dirty_const/0.1.0/dirty_const/)

This crate provides a container for a value, `DirtyConst`, which
allows mutation in debug mode (via `UnsafeCell`), but not in
release mode.

This allows you to tweak data while testing an application,
without having that data be mutable when the application is
released.

There are also two features available:

1. `force-dynamic` which allows replacing the value of a
    `DirtyConst` even in release mode.
2. `force-static` which disallows replacing the value of a
    `DirtyConst` even in debug mode.

## Usage

```rust
// In debug mode
use dirty_const::DirtyConst;

let c = DirtyConst::new(100);
unsafe {
    c.replace(200);
}

assert_eq!(*c, 200);
```

```rust
// In release mode
use dirty_const::DirtyConst;

let c = DirtyConst::new(100);
unsafe {
    c.replace(200);
}

assert_eq!(*c, 100);
```
