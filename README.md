# nuhound
A Rust library for enhanced error tracking

Rust programmers often find the question mark operator invaluable in extracting values from
Result and Option and immediately returning to the calling context in the case of an Err or
None. This crate provides some enhancements to this functionality by:
- Converting Result::Err and Option::None values to a single nuhound type error;
- Creating an error chain that can help pinpoint the source of the error;
- Providing a `disclose` feature that enhances error messages by incuding the filename, line
number and column number of the source file that caused the error. This functionality is
provided by the here! macro when the `disclose` feature is enabled;
- Simplifying error handling with a concise and consistent Rust style.
- Providing a simple implementation that requires minimal changes to your coding experience.

Remember to add this to Cargo.toml:
```text
[features]
# To help diagnose errors, use the disclose feature when compiling.
# This ensures that the source file name and line number are displayed
# when using the here! macro.
# example usage: cargo build --features=disclose
disclose = []
```
## Example

The following example shows how the `here` macro is used to report an error but still retain
the underlying error or errors that can be displayed using the `trace` method.
```
use nuhound::{Report, here, ResultExtension};

fn generate_error() -> Report<u32> {
    let text = "NaN";
    let value = text.parse::<u32>().report(|e| here!(e, "Oh dear - '{}' could not be \
    converted to an integer", text))?;
    Ok(value)
}

let result = generate_error();

match result {
    Ok(_) => unreachable!(),
    Err(e) => {
        println!("Display the error:\n{e}\n");
        println!("Or trace the error:\n{}\n", e.trace());
    }
}
// This will emit:
// Display the error:
// Oh dear - 'NaN' could not be converted to an integer
//
// Or trace the error:
// 0: Oh dear - 'NaN' could not be converted to an integer
// 1: invalid digit found in string
//
// This will also show the name of the file causing the error
// and the line and column number if the code is compiled with
// the disclose feature enabled.
```

## License

This project is licensed under either:

- Apache License, Version 2.0, ([LICENSE.apache-2.0](LICENSE.apache-2.0) or
   https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE.MIT](LICENSE.MIT) or
   https://opensource.org/licenses/MIT)
