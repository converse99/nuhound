# nuhound
Rust error tracker and handler

Rust programmers often find the question mark operator invaluable in extracting values from
Result and Option and immediately returning to the calling context in the case of an Err or
None. This crate provides some enhancements to this functionality by:
- Converting Result::Err and Option::None values to a single nuhound type error;
- Creating an error chain that can help pinpoint the source of the error;
- Providing a `disclose` feature that enhances the error message by incuding the filename, line
number and column number of the source file that caused the error;
- Simplifying error handling with a concise and consistent Rust style.
- Minimisation of typing required to implement the crate.

Remember to add this to Cargo.toml:
```text
[features]
# To help diagnose errors, use the disclose feature when compiling.
# This ensures that the source file name and line number are displayed
# when using the here! macro.
# example usage: cargo build --features=disclose
disclose = []
```
