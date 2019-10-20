# dirty_const

A container for an immutable value that allows sneaky reloading in debug mode (via UnsafeCell) while keeping the data safe and constant in release mode.

This allows you to tweak data while testing an application, without having that data be mutable when the application is released.
