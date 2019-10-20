# dirty_static

[![Build Status](https://travis-ci.org/mistodon/dirty_static.svg?branch=master)](https://travis-ci.org/mistodon/dirty_static)
[![Crates.io](https://img.shields.io/crates/v/dirty_static.svg)](https://crates.io/crates/dirty_static)
[![Docs.rs](https://docs.rs/resource/badge.svg)](https://docs.rs/dirty_static/0.1.0/dirty_static/)

This crate provides a container for a value, `DirtyStatic`, which
allows mutation in debug mode (via `UnsafeCell`), but not in
release mode.

This allows you to tweak data while testing an application,
without having that data be mutable when the application is
released.

There are also two features available:

1. `force-dynamic` which allows replacing the value of a
    `DirtyStatic` even in release mode.
2. `force-static` which disallows replacing the value of a
    `DirtyStatic` even in debug mode.

## Usage

```rust
// In debug mode
use dirty_static::DirtyStatic;

let c = DirtyStatic::new(100);
unsafe {
    c.replace(200);
}

assert_eq!(*c, 200);
```

```rust
// In release mode
use dirty_static::DirtyStatic;

let c = DirtyStatic::new(100);
unsafe {
    c.replace(200);
}

assert_eq!(*c, 100);
```
