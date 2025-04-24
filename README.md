# nuhound
A Rust library for enhanced error tracking

Rust programmers often find the question mark operator invaluable in extracting values from
Result and Option and immediately returning to the calling context in the case of an Err or
None. This crate provides some enhancements to this functionality by:
- Converting Result::Err and Option::None values to a single nuhound type error;
- Creating an error chain that can help pinpoint the source of the error;
- Providing a `disclose` feature that enhances error messages by including the filename, line
number and column number of the source file that caused the error. This functionality is
provided by the `here!`, `convert!`, `examine!` and `custom!` macros when the `disclose`
feature is enabled;
- Simplifying error handling in a concise and consistent Rust style.
- Providing a simple implementation that requires minimal changes to your coding experience.

Remember to add this to Cargo.toml:
```text
[features]
# To help diagnose errors, use the disclose feature when compiling.
# This ensures that the source file name and line number are displayed
# when using the here!, convert!, examine! and custom! macros.
# example usage: cargo build --features=disclose
isclose = []
```
## Examples

### here!

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
        #[cfg(feature = "disclose")]
        eprintln!("{}", e.trace());
        #[cfg(not(feature = "disclose"))]
        eprintln!("{}", e);
    },
}
// With the disclose feature enabled the code will emit:
// 0: src/main.rs:6:48: Oh dear - 'NaN' could not be converted to an integer
// 1: invalid digit found in string
//
// With the disclose feature disabled the code will emit:
// Oh dear - 'NaN' could not be converted to an integer
```

### convert! and examine!

The following example shows how the `convert` and `examine` macros are used to simplify error
tracing. This is achieved by encapsulating rust methods and functions that return values in the form of
`Result<T, E>`.  Using these macros affords the same error handling capabilities as the `here`
macro but in a more compact form.

Notice that the `convert` macro is used to translate the error produced by `text.parse` into a
`nuhound` error. The `examine` macro is used when the code generating an error is already a
`nuhound` type. It would, however, be possible to replace the `examine` macros in this example with
`convert`, but it is more code efficient to use `examine` whenever possible.
```
use nuhound::{Report, ResultExtension, examine, convert};

fn my_result() -> Report<()> {
    let text = "NaN";
    let _value = convert!(text.parse::<u32>(), "Oh dear - '{}' could not be \
    converted to an integer", text)?;
    Ok(())
}

fn layer2() -> Report<()> {
    let _result = examine!(my_result(), "Layer 2 failure")?;
    Ok(())
}

fn layer1() -> Report<()> {
    let _result = examine!(layer2(), "Layer 1 failure")?;
    Ok(())
}

match layer1() {
    Ok(_) => unreachable!(),
    Err(e) => {
        #[cfg(feature = "disclose")]
        eprintln!("{}", e.trace());
        #[cfg(not(feature = "disclose"))]
        eprintln!("{}", e);
    },
}
// With the disclose feature enabled the code will emit:
// 0: src/main.rs:16:23: Layer 1 failure
// 1: src/main.rs:11:23: Layer 2 failure
// 2: src/main.rs:6:22: Oh dear - 'NaN' could not be converted to an integer
// 3: invalid digit found in string
//
// With the disclose feature disabled the code will emit:
// Layer 1 failure
```

### custom!

This example shows how the `custom!` macro could be used to generate an error based on a
conditional branch
```
use nuhound::{Report, ResultExtension, examine, custom};

fn my_custom() -> Report<()> {
    let reason = "No reason at all";
    if reason != "" {
        custom!("This just fails because of: {}", reason)
    } else {
        Ok(())
    }
}

fn layer2() -> Report<()> {
    let _result = examine!(my_custom(), "Layer 2 failure")?;
    Ok(())
}

fn layer1() -> Report<()> {
    let _result = examine!(layer2(), "Top level failure")?;
    Ok(())
}

match layer1() {
    Ok(_) => unreachable!(),
    Err(e) => {
        #[cfg(feature = "disclose")]
        eprintln!("{}", e.trace());
        #[cfg(not(feature = "disclose"))]
        eprintln!("{}", e);
    },
}
// With the disclose feature enabled the code will emit:
// 0: src/main.rs:19:23: Top level failure
// 1: src/main.rs:14:23: Layer 2 failure
// 2: src/main.rs:7:13: This just fails because of: No reason at all
//
// With the disclose feature disabled the code will emit:
// Top level failure
```

### Option handling

The `convert!` macro can be used with an Option to handle 'None' as a type of error. In this
example we attempt to get a value from a vector with an out-of-range index. The 'get' will
return a None value that is handled by the `convert!` macro.
```
use nuhound::{Report, ResultExtension, OptionExtension, examine, convert};

fn my_option() -> Report<()> {
    let vector = vec![0,1,2,3];
    let index = 4;
    let value = convert!(vector.get(index), "Index {index} is out of range")?;
    println!("Value = {value}");
    Ok(())
}

fn layer2() -> Report<()> {
    let _result = examine!(my_option(), "Layer 2 failure")?;
    Ok(())
}

fn layer1() -> Report<()> {
    let _result = examine!(layer2(), "Top level failure")?;
    Ok(())
}

match layer1() {
    Ok(_) => unreachable!(),
    Err(e) => {
        #[cfg(feature = "disclose")]
        eprintln!("{}", e.trace());
        #[cfg(not(feature = "disclose"))]
        eprintln!("{}", e);
    },
}
// With the disclose feature enabled the code will emit:
// 0: src/main.rs:18:23: Top level failure
// 1: src/main.rs:13:23: Layer 2 failure
// 2: src/main.rs:7:21: Index 4 is out of range
// 3: Option::None detected
//
// With the disclose feature disabled the code will emit:
// Top level failure
```

### Using closures

The `convert` and `examine` macros may used with closures provided they are delimited with
curly braces. The example shown here encloses the vector get, as above, in a closure.

```
use nuhound::{Report, ResultExtension, OptionExtension, examine, convert};

fn my_closure_test() -> Report<()> {
    let vector = vec![0,1,2,3];
    let index = 4;
    // Notice that the closure is delimited with curly braces
    let _ = convert!({|| 
        vector.get(index)
    }(), "Index out of range")?;
    Ok(())
}

fn layer2() -> Report<()> {
    let _result = examine!(my_closure_test(), "Layer 2 failure")?;
    Ok(())
}

fn layer1() -> Report<()> {
    let _result = examine!(layer2(), "Top level failure")?;
    Ok(())
}

match layer1() {
    Ok(_) => unreachable!(),
    Err(e) => {
        #[cfg(feature = "disclose")]
        eprintln!("{}", e.trace());
        #[cfg(not(feature = "disclose"))]
        eprintln!("{}", e);
    },
}
// With the disclose feature enabled the code will emit:
// 0: src/main.rs:20:23: Top level failure
// 1: src/main.rs:15:23: Layer 2 failure
// 2: src/main.rs:8:17: Index out of range
// 3: Option::None detected
//
// With the disclose feature disabled the code will emit:
// Top level failure
```

## License

This project is licensed under either:

- Apache License, Version 2.0, ([LICENSE.apache-2.0](LICENSE.apache-2.0) or
   https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE.MIT](LICENSE.MIT) or
   https://opensource.org/licenses/MIT)
