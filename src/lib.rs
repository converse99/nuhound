//! A Rust library for enhanced error tracking
//! 
//! Rust programmers often find the question mark operator invaluable in extracting values from
//! Result and Option and immediately returning to the calling context in the case of an Err or
//! None. This crate provides some enhancements to this functionality by:
//! - Converting Result::Err and Option::None values to a single nuhound type error;
//! - Creating an error chain that can help pinpoint the source of the error;
//! - Providing a `disclose` feature that enhances error messages by including the filename, line
//! number and column number of the source file that caused the error. This functionality is
//! provided by the `here!`, `convert!`, `examine!` and `custom!` macros when the `disclose`
//! feature is enabled;
//! - Simplifying error handling in a concise and consistent Rust style.
//! - Providing a simple implementation that requires minimal changes to your coding experience.
//! 
//! Remember to add this to Cargo.toml:
//! ```text
//! [features]
//! ## To help diagnose errors, use the disclose feature when compiling.
//! ## This ensures that the source file name and line number are displayed
//! ## when using the here!, convert!, examine! and custom! macros.
//! ## example usage: cargo build --features=disclose
//! disclose = []
//! ```
//! ## Examples
//!
//! ### here!
//! 
//! The following example shows how the `here` macro is used to report an error but still retain
//! the underlying error or errors that can be displayed using the `trace` method.
//! ```
//! use nuhound::{Report, here, ResultExtension};
//! 
//! fn generate_error() -> Report<u32> {
//!     let text = "NaN";
//!     let value = text.parse::<u32>().report(|e| here!(e, "Oh dear - '{}' could not be \
//!     converted to an integer", text))?;
//!     Ok(value)
//! }
//! 
//! let result = generate_error();
//! 
//! match result {
//!     Ok(_) => unreachable!(),
//!     Err(e) => {
//!         #[cfg(feature = "disclose")]
//!         eprintln!("{}", e.trace());
//!         #[cfg(not(feature = "disclose"))]
//!         eprintln!("{}", e);
//!     },
//! }
//! // With the disclose feature enabled the code will emit:
//! // 0: src/main.rs:6:48: Oh dear - 'NaN' could not be converted to an integer
//! // 1: invalid digit found in string
//! //
//! // With the disclose feature disabled the code will emit:
//! // Oh dear - 'NaN' could not be converted to an integer
//! ```
//!
//! ### convert! and examine!
//!
//! The following example shows how the `convert` and `examine` macros are used to simplify error
//! tracing. This is achieved by encapsulating rust methods and functions that return values in the form of
//! `Result<T, E>`.  Using these macros affords the same error handling capabilities as the `here`
//! macro but in a more compact form.
//!
//! Notice that the `convert` macro is used to translate the error produced by `text.parse` into a
//! `nuhound` error. The `examine` macro is used when the code generating an error is already a
//! `nuhound` type. It would, however, be possible to replace the `examine` macros in this example with
//! `convert`, but it is more code efficient to use `examine` whenever possible.
//! ```
//! use nuhound::{Report, ResultExtension, examine, convert};
//!
//! fn my_result() -> Report<()> {
//!     let text = "NaN";
//!     let _value = convert!(text.parse::<u32>(), "Oh dear - '{}' could not be \
//!     converted to an integer", text)?;
//!     Ok(())
//! }
//!
//! fn layer2() -> Report<()> {
//!     let _result = examine!(my_result(), "Layer 2 failure")?;
//!     Ok(())
//! }
//!
//! fn layer1() -> Report<()> {
//!     let _result = examine!(layer2(), "Layer 1 failure")?;
//!     Ok(())
//! }
//!
//! match layer1() {
//!     Ok(_) => unreachable!(),
//!     Err(e) => {
//!         #[cfg(feature = "disclose")]
//!         eprintln!("{}", e.trace());
//!         #[cfg(not(feature = "disclose"))]
//!         eprintln!("{}", e);
//!     },
//! }
//! // With the disclose feature enabled the code will emit:
//! // 0: src/main.rs:16:23: Layer 1 failure
//! // 1: src/main.rs:11:23: Layer 2 failure
//! // 2: src/main.rs:6:22: Oh dear - 'NaN' could not be converted to an integer
//! // 3: invalid digit found in string
//! //
//! // With the disclose feature disabled the code will emit:
//! // Layer 1 failure
//! ```
//!
//! ### custom!
//!
//! This example shows how the `custom!` macro could be used to generate an error based on a
//! conditional branch
//! ```
//! use nuhound::{Report, ResultExtension, examine, custom};
//!
//! fn my_custom() -> Report<()> {
//!     let reason = "No reason at all";
//!     if reason != "" {
//!         custom!("This just fails because of: {}", reason)
//!     } else {
//!         Ok(())
//!     }
//! }
//!
//! fn layer2() -> Report<()> {
//!     let _result = examine!(my_custom(), "Layer 2 failure")?;
//!     Ok(())
//! }
//!
//! fn layer1() -> Report<()> {
//!     let _result = examine!(layer2(), "Top level failure")?;
//!     Ok(())
//! }
//!
//! match layer1() {
//!     Ok(_) => unreachable!(),
//!     Err(e) => {
//!         #[cfg(feature = "disclose")]
//!         eprintln!("{}", e.trace());
//!         #[cfg(not(feature = "disclose"))]
//!         eprintln!("{}", e);
//!     },
//! }
//! // With the disclose feature enabled the code will emit:
//! // 0: src/main.rs:19:23: Top level failure
//! // 1: src/main.rs:14:23: Layer 2 failure
//! // 2: src/main.rs:7:13: This just fails because of: No reason at all
//! //
//! // With the disclose feature disabled the code will emit:
//! // Top level failure
//! ```
//!
//! ### Option handling
//!
//! The `convert!` macro can be used with an Option to handle 'None' as a type of error. In this
//! example we attempt to get a value from a vector with an out-of-range index. The 'get' will
//! return a None value that is handled by the `convert!` macro.
//! ```
//! use nuhound::{Report, ResultExtension, OptionExtension, examine, convert};
//!
//! fn my_option() -> Report<()> {
//!     let vector = vec![0,1,2,3];
//!     let index = 4;
//!     let value = convert!(vector.get(index), "Index {index} is out of range")?;
//!     println!("Value = {value}");
//!     Ok(())
//! }
//!
//! fn layer2() -> Report<()> {
//!     let _result = examine!(my_option(), "Layer 2 failure")?;
//!     Ok(())
//! }
//!
//! fn layer1() -> Report<()> {
//!     let _result = examine!(layer2(), "Top level failure")?;
//!     Ok(())
//! }
//!
//! match layer1() {
//!     Ok(_) => unreachable!(),
//!     Err(e) => {
//!         #[cfg(feature = "disclose")]
//!         eprintln!("{}", e.trace());
//!         #[cfg(not(feature = "disclose"))]
//!         eprintln!("{}", e);
//!     },
//! }
//! // With the disclose feature enabled the code will emit:
//! // 0: src/main.rs:18:23: Top level failure
//! // 1: src/main.rs:13:23: Layer 2 failure
//! // 2: src/main.rs:7:21: Index 4 is out of range
//! // 3: Option::None detected
//! //
//! // With the disclose feature disabled the code will emit:
//! // Top level failure
//! ```
//!
//! ### Using closures
//!
//! The `convert` and `examine` macros may used with closures provided they are delimited with
//! curly braces. The example shown here encloses the vector get, as above, in a closure.
//!
//! ```
//! use nuhound::{Report, ResultExtension, OptionExtension, examine, convert};
//!
//! fn my_closure_test() -> Report<()> {
//!     let vector = vec![0,1,2,3];
//!     let index = 4;
//!     // Notice that the closure is delimited with curly braces
//!     let _ = convert!({|| 
//!         vector.get(index)
//!     }(), "Index out of range")?;
//!     Ok(())
//! }
//!
//! fn layer2() -> Report<()> {
//!     let _result = examine!(my_closure_test(), "Layer 2 failure")?;
//!     Ok(())
//! }
//!
//! fn layer1() -> Report<()> {
//!     let _result = examine!(layer2(), "Top level failure")?;
//!     Ok(())
//! }
//!
//! match layer1() {
//!     Ok(_) => unreachable!(),
//!     Err(e) => {
//!         #[cfg(feature = "disclose")]
//!         eprintln!("{}", e.trace());
//!         #[cfg(not(feature = "disclose"))]
//!         eprintln!("{}", e);
//!     },
//! }
//! // With the disclose feature enabled the code will emit:
//! // 0: src/main.rs:20:23: Top level failure
//! // 1: src/main.rs:15:23: Layer 2 failure
//! // 2: src/main.rs:8:17: Index out of range
//! // 3: Option::None detected
//! //
//! // With the disclose feature disabled the code will emit:
//! // Top level failure
//! ```
//!

#![allow(unused)]
use std::error::Error;
use std::fmt;
pub use proc_nuhound::{examine, convert, custom};
use std::any::Any;

/// The Report typedef is used to simplify [`Result`] enum usage when using the nuhound crate
///
/// # Example
/// ```
/// use nuhound::{Report, here, OptionExtension};
///
/// fn generate_error() -> Report<()> {
///     let value = None;
///     let message = "This is a test error messaage";
///     value.report(|e| here!(e, "{}", message))?;
///     Ok(())
/// }
///
/// let result = generate_error();
///
/// assert!(result.is_err());
/// println!("{:?}", result);
/// ```
pub type Report<T> = Result<T, Nuhound>;

//  here macro
/// Macro to prepare a Nuhound type error that can be handled by the calling context either by using
/// the '?' operator or by simply returning it as a Result::Err directly.
///
/// The macro creates an error message that can optionally contain the name of the source file and
/// location of the error. This behaviour is enabled by compiling the code with the 'disclose'
/// feature enabled.
///
/// This macro is particularly useful when using the `report` trait that can be found in
/// nuhound::OptionExtension or nuhound::ResultExtension..
///
/// # Examples
/// The following example shows how the `here` macro is used to report an error but still retain
/// the underlying error or errors that can be displayed using the `trace` method.
/// ```
/// use nuhound::{Report, here, ResultExtension};
///
/// fn generate_error() -> Report<u32> {
///     let text = "NaN";
///     let value = text.parse::<u32>().report(|e| here!(e, "Oh dear - '{}' could not be \
///     converted to an integer", text))?;
///     Ok(value)
/// }
///
/// let result = generate_error();
///
/// match result {
///     Ok(_) => unreachable!(),
///     Err(e) => {
///         println!("Display the error:\n{e}\n");
///         println!("Or trace the error:\n{}\n", e.trace());
///     }
/// }
/// // This will emit:
/// // Display the error:
/// // Oh dear - 'NaN' could not be converted to an integer
/// //
/// // Or trace the error:
/// // 0: Oh dear - 'NaN' could not be converted to an integer
/// // 1: invalid digit found in string
/// //
/// // This will also show the name of the file causing the error
/// // and the line and column number if the code is compiled with
/// // the disclose feature enabled.
///```
///
/// This example shows how the `here` macro in conjunction with the `Root` token can be used to
/// report a custom error omiting the underlying cause. Notice the trace method no longer emits
/// 'invalid digit found in string'.
///```
/// use nuhound::{Report, here, ResultExtension};
/// 
/// fn generate_error() -> Report<u32> {
///     let text = "NaN";
///     let value = text.parse::<u32>().report(|_| here!(Root, "Oh dear - '{}' could not be \
///     converted to an integer", text))?;
///     Ok(value)
/// }
/// 
/// let result = generate_error();
/// 
/// match result {
///     Ok(_) => unreachable!(),
///     Err(e) => {
///         println!("Display the error:\n{e}\n");
///         println!("Or trace the error:\n{}\n", e.trace());
///     }
/// }
/// // This will emit:
/// // Display the error:
/// // Oh dear - 'NaN' could not be converted to an integer
/// //
/// // Or trace the error:
/// // 0: Oh dear - 'NaN' could not be converted to an integer
/// //
/// // This will also show the name of the file causing the error
/// // and the line and column number if the code is compiled with
/// // the disclose feature enabled.
/// ```
///
/// This example shows the `here` macro being used to convert the underlying error message to a
/// Nuhound error. This enables the underlying file and location to be displayed when the code is
/// compiled with the disclose feature enabled.
/// ```
/// use nuhound::{Report, here, ResultExtension};
/// 
/// fn generate_error() -> Report<u32> {
///     let text = "NaN";
///     let value = text.parse::<u32>().report(|e| here!(e))?;
///     Ok(value)
/// }
/// 
/// let result = generate_error();
/// 
/// match result {
///     Ok(_) => unreachable!(),
///     Err(e) => {
///         println!("Display the error:\n{e}\n");
///         println!("Or trace the error:\n{}\n", e.trace());
///     }
/// }
/// // This will emit:
/// // Display the error:
/// // invalid digit found in string
///
/// // Or trace the error:
/// // 0: invalid digit found in string
/// ```
///
/// This example shows the `here` macro being used standalone. Note that it should be used with the
/// 'Root' token because there are no other associated errors.
/// ```
/// use nuhound::{Report, here};
/// 
/// fn generate_error() -> Report<u32> {
///     let value = 23_u32;
///     if value == 23 {
///         return Err(here!(Root, "value 23 not allowed"));
///     }
///     Ok(42)
/// }
/// 
/// let result = generate_error();
/// 
/// match result {
///     Ok(_) => unreachable!(),
///     Err(e) => println!("{e}"),
/// }
/// // This will emit:
/// // value 23 not allowed
/// ```
#[macro_export]
macro_rules! here {
    () => {
        $crate::here!(Root)
    };
    ( Root ) => {
        $crate::here!(Root, "unspecified error")
    };
    ( Root, $($inform:expr),+ ) => {{
        let inform = format!( $($inform),+ );
        #[cfg(feature="disclose")]
        let inform = format!("{}:{}:{}: {}", file!(), line!(), column!(), inform);
        $crate::Nuhound::new(inform)
    }};
    ( $caused_by:expr ) => {{
        let cause: &dyn std::error::Error = &$caused_by;
        match cause.source() {
            Some(source) => $crate::here!(source, "{}", $caused_by),
            None => $crate::here!(Root , "{}", $caused_by),
        }
    }};
    ( $caused_by:expr, $($inform:expr),+ ) => {{
        let mut cause: &dyn std::error::Error = &$caused_by;
        let mut causes = vec![$crate::Nuhound::new(cause)];
        while cause.source().is_some() {
            cause = cause.source().unwrap();
            causes.push($crate::Nuhound::new(cause));
        }

        let mut current = causes.pop();
        let mut chain = current.unwrap();
        current = causes.pop();
        while current.is_some() {
            chain = current.unwrap().caused_by(chain);
            current = causes.pop();
        }

        $crate::here!(Root, $($inform),+).caused_by(chain)
    }};
}

/// The structure holds the current error message as well as previous errors in a source chain that
/// is represented as a *cons list*. Enhanced debugging can be enabled by compiling the code with
/// the disclose feature enabled. This feature is available when Nuhound errors are generated using
/// the following macros: `here!`, `convert!`, `examine!` and `custom!`. Enhanced debugging
/// generates an error trace containing the source file name, line number and column number back to
/// the originating code.
///
/// # Example
///
/// ```
/// use std::fs::File;
/// 
/// use nuhound::{
///     Report,
///     here,
///     ResultExtension,
/// };
/// 
/// // Attempt to open a file that doesn't exist
/// fn level2() -> Report<()> {
///     // I assume there is no file in the current directory called this!
///     let filename = "xuhgd56qhsl";
///     let _file = File::open(filename).report(|e| here!(e, "Failed to open file '{}'", filename))?;
///     Ok(())
/// }
/// 
/// fn level1() -> Report<()> {
///     level2().report(|e| here!(e, "Well that's another fine mess"))?;
///     Ok(())
/// }
/// 
/// fn level0() -> Report<()> {
///     level1().report(|e| here!(e, "My user interface didn't work"))?;
///     Ok(())
/// }
/// 
/// fn run() -> Report<()> {
///     level0().report(|e| here!(e, "Better tell the end user"))?;
///     Ok(())
/// }
/// 
/// // Using the trace method in conjunction with the disclose feature helps
/// // the debugging process by showing exactly where the error occured
/// match run() {
///     Err(e) => println!("{}", e.trace()),
///     Ok(_) => unreachable!(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nuhound {
    source: Option<Box<Nuhound>>,
    message: String,
}

impl Error for Nuhound {
    /// Returns the source of the current error or `None` if no source information is available.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.source {
            Some(source) => Some(source.as_ref()),
            None => None,
        }
    }
}

impl fmt::Display for Nuhound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<&str> for Nuhound {
    fn from(value: &str) -> Nuhound {
        Nuhound::new(value.to_string())
    }
}

impl Nuhound {
    /// Create a Nuhound error.
    ///
    /// # Example
    ///
    /// ```
    /// use nuhound::Nuhound;
    ///
    /// let e = Nuhound::new("My custom error");
    /// ```
    pub fn new(inform: impl fmt::Display) -> Self {
        Self {
            source: None,
            message: inform.to_string(),
        }
    }

    /// Create a Nuhound error chain by appending and consolidating an existing error chain.
    /// Each element in the chain is converted into a Nuhound type.
    ///
    /// # Example
    ///
    /// ```
    /// use nuhound::{Nuhound, is_nuhound};
    /// use std::num::ParseIntError;
    ///
    /// fn generate_error() -> Result<u32, ParseIntError> {
    ///     let text = "NaN";
    ///     // This will fail because 'NaN' is not an integer
    ///     let value = text.parse::<u32>()?;
    ///     Ok(value)
    /// }
    ///
    /// match generate_error() {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => {
    ///         assert!(!is_nuhound(&e)); // This isn't a nuhound error
    ///         // Convert the underlying error 'e' to a Nuhound by linking
    ///         let my_error = Nuhound::link("Parse Integer failed", e);
    ///         assert!(is_nuhound(&my_error)); // This is a nuhound error
    ///         assert_eq!(my_error.trace(), " 0: Parse Integer failed\n 1: invalid digit found in string");
    ///     },
    /// }
    /// ```
    pub fn link(inform: impl fmt::Display, caused_by: impl Error) -> Self {
        // Take the whole chain converting each to Nuhound along the way
        // We assume that the chain may contain non-Nuhound errors
        let mut cause: &dyn Error = &caused_by;
        let mut causes = vec![Nuhound::new(cause)];
        while cause.source().is_some() {
            cause = cause.source().unwrap();
            causes.push(Nuhound::new(cause));
        }
        let mut current = causes.pop();
        let mut chain = current.unwrap();
        current = causes.pop();
        while current.is_some() {
            chain = current.unwrap().caused_by(chain);
            current = causes.pop();
        }

        // Finally add the top level message 'inform' to the chain
        Nuhound::new(inform).caused_by(chain)
    }

    /// Add a cause to an existing Nuhound error.
    ///
    /// ```
    /// use nuhound::{Nuhound, OptionExtension};
    ///
    /// let error_source = vec![1, 2, 3, 4].get(4).easy().unwrap_err();
    /// let my_error = Nuhound::new("Out of bounds").caused_by(error_source);
    /// println!("{}", my_error.trace());
    /// // emits:
    /// //  0: Out of bounds
    /// //  1: Option::None detected
    /// ```
    pub fn caused_by(mut self, source: Nuhound) -> Self {
        self.source = Some(Box::new(source));
        self
    }
 
    /// Create a list of errors starting at the most recent error and working backwards towards the
    /// the error source.
    ///
    /// ```
    /// use nuhound::{Nuhound, OptionExtension};
    ///
    /// let error_source = vec![1, 2, 3, 4].get(4).easy().unwrap_err();
    /// let my_error = Nuhound::new("Out of bounds").caused_by(error_source);
    /// println!("{}", my_error.trace());
    /// // emits:
    /// //  0: Out of bounds
    /// //  1: Option::None detected
    /// ```
    pub fn trace(&self) -> String {
        let mut trace_list = vec![format!(" 0: {}", self)];
        let mut n = 1;
        let mut item = self.source.as_ref();
        while item.is_some() {
            let this = item.unwrap();
            trace_list.push(format!("{:2}: {}", n, this));
            item = this.source.as_ref();
            n += 1;
        }
        trace_list.join("\n")
    }
}

/// Provides `Nuhound` trait support to `std::result::Result`. Remember to `use` this if you're
/// intending to use the `report()` and/or `easy()` methods with values of type `Result<T, E>` or
/// functions that return `Result<T, E>`.
pub trait ResultExtension<T, E> {
    /// Calls op lazily if the result is Err, otherwise returns the Ok value of self.
    ///
    /// This function can be used for control flow based on result values and is similar to the
    /// map_err function in the standard library. This function returns only Nuhound type errors and
    /// is designed to work well with the `here` macro.
    ///
    /// # Example:
    ///
    /// ```
    /// use nuhound::{Report, here, ResultExtension};
    /// 
    /// fn generate_error() -> Report<u32> {
    ///     let text = "NaN";
    ///     let value = text.parse::<u32>().report(|e| here!(e))?;
    ///     Ok(value)
    /// }
    /// 
    /// let result = generate_error();
    /// 
    /// match result {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => println!("Display the error:\n{e}\n"),
    /// }
    /// // This will emit:
    /// // Display the error:
    /// // invalid digit found in string
    /// ```
    fn report<O: FnOnce(E) -> Nuhound>(self, op: O) -> Result<T, Nuhound>;

    /// Lazily converts any error into a nuhound error, otherwise returns the Ok value of self.
    ///
    /// # Example:
    ///
    /// ```
    /// use nuhound::{Report, ResultExtension};
    /// 
    /// fn generate_error() -> Report<u32> {
    ///     let text = "NaN";
    ///     let value = text.parse::<u32>().easy()?;
    ///     Ok(value)
    /// }
    /// 
    /// let result = generate_error();
    /// 
    /// match result {
    ///     Ok(_) => unreachable!(),
    ///     Err(e) => println!("{e}"),
    /// }
    /// // This will emit:
    /// // invalid digit found in string
    /// ```
    fn easy(self) -> Result<T, Nuhound>;
}

impl<T, E: Error> ResultExtension<T, E> for Result<T, E> {
    fn report<O: FnOnce(E) -> Nuhound>(self, op: O) -> Result<T, Nuhound> {
        match self {
            Ok(val) => Ok(val),
            Err(e) => Err(op(e)),
        }
    }

    fn easy(self) -> Result<T, Nuhound> {
        match self {
            Ok(val) => Ok(val),
            Err(e) => {
                match e.source() {
                    Some(source) => {
                        let mut cause: &dyn Error = &source;
                        let mut causes = vec![Nuhound::new(cause)];
                        while cause.source().is_some() {
                            cause = cause.source().unwrap();
                            causes.push(Nuhound::new(cause));
                        }

                        let mut current = causes.pop();
                        let mut chain = current.unwrap();
                        current = causes.pop();
                        while current.is_some() {
                            chain = current.unwrap().caused_by(chain);
                            current = causes.pop();
                        }
                        Err(Nuhound::new(e).caused_by(chain))
                    },
                    None => Err(Nuhound::new(e)),
                }
            },
        }
    }
}

/// Provides `Nuhound` trait support to `std::option::Option`. Remember to `use` this if you're
/// intending to use the `report()` and/or `easy()` methods with values of type `Option<T>` or functions that
/// return `Option<T>`.
pub trait OptionExtension<T> {
    /// Transforms the `Option<T>` into a [`Result<T, Nuhound>`]
    ///
    /// This function has some simarlarity to ok_or_else in the standard library except that this
    /// returns a Nuhound type error and that a Nuhound error is passed as a paramter to op. It is
    /// designed to work well with the `here` macro.
    ///
    /// # Example
    ///
    /// ```
    /// use nuhound::{Report, here, OptionExtension};
    ///
    /// fn oob() -> Report<u32> {
    ///    let list: Vec<u32> = vec![1, 2, 3, 4,];
    ///    let bad_val = *list.get(4).report(|e| here!(e, "Index out of bounds"))?;
    ///    Ok(bad_val)
    /// }
    /// let bad = oob().unwrap_err();
    /// println!("{}", bad.trace());
    /// ```
    fn report<O: FnOnce(Nuhound) -> Nuhound>(self, op: O) -> Result<T, Nuhound>;

    /// Transforms the `Option<T>` into a [`Result<T, Nuhound>`].
    ///
    /// This is a simple method of transforming an Option into a Result
    ///
    /// # Example
    ///
    /// ```
    /// use nuhound::{Report, OptionExtension};
    ///
    /// fn oob() -> Report<u32> {
    ///    let list: Vec<u32> = vec![1, 2, 3, 4,];
    ///    let bad_val = *list.get(4).easy()?;
    ///    Ok(bad_val)
    /// }
    /// let bad = oob().unwrap_err();
    /// println!("{bad}");
    /// ```
    fn easy(self) -> Result<T, Nuhound>;
}

impl<T> OptionExtension<T> for Option<T> {
    fn report<O: FnOnce(Nuhound) -> Nuhound>(self, op: O) -> Result<T, Nuhound> {
        match self {
            Some(val) => Ok(val),
            None => Err(op(Nuhound::new("Option::None detected"))),
        }
    }

    fn easy(self) -> Result<T, Nuhound> {
        match self {
            Some(val) => Ok(val),
            None => Err(Nuhound::new("Option::None detected")),
        }
    }
}

/// Determines whether the value is of type `Nuhound`
///
/// # Example
///
/// ```
/// use nuhound::{Report, here, ResultExtension, is_nuhound};
///
/// fn generate_error() -> Report<u32> {
///     let text = "NaN";
///     let value = text.parse::<u32>().report(|e| here!(e, "Oh dear - '{}' could not be \
///     converted to an integer", text))?;
///     Ok(value)
/// }
///
/// let result = generate_error();
///
/// match result {
///     Ok(_) => unreachable!(),
///     Err(e) => {
///         println!("This is nuhound: {}", is_nuhound(&e));
///         // This will print 'true' to confirm the error is of type nuhound
///         #[cfg(feature = "disclose")]
///         eprintln!("{}", e.trace());
///         #[cfg(not(feature = "disclose"))]
///         eprintln!("{}", e);
///     },
/// }
/// ```
pub fn is_nuhound(val: &dyn Any) -> bool {
    val.is::<Nuhound>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_01() -> Report<()> {
        fn good_value() -> Report<u32> {
            let value = "999".parse::<u32>()
                .report(|_| here!())?;
            Ok(value)
        }
        fn bad_value() -> Report<u32> {
            let value = "NaN".parse::<u32>()
                .report(|_| here!())?;
            Ok(value)
        }
        assert_eq!(good_value()?, 999);
        let value = bad_value().unwrap_err().to_string(); 
        if cfg!(feature = "disclose") {
            let re = Regex::new(r"^src[\\/]lib\.rs:\d+:\d+: unspecified error$").unwrap();
            assert!(re.is_match(&value));
        } else {
            assert_eq!(value, "unspecified error");
        }
        Ok(())
    }

    #[test]
    fn test_02() {
        fn bad_value() -> Report<u32> {
            let value = "NaN".parse::<u32>()
                .report(|_| here!(Root))?;
            Ok(value)
        }
        let value = bad_value().unwrap_err().to_string(); 
        if cfg!(feature = "disclose") {
            let re = Regex::new(r"^src[\\/]lib\.rs:\d+:\d+: unspecified error$").unwrap();
            assert!(re.is_match(&value));
        } else {
            assert_eq!(value, "unspecified error");
        }
    }

    #[test]
    fn test_03() {
        fn bad_value() -> Report<u32> {
            let text = "error";
            let value = "NaN".parse::<u32>()
                .report(|_| here!(Root, "this is an {text}"))?;
            Ok(value)
        }
        let value = bad_value().unwrap_err().to_string(); 
        if cfg!(feature = "disclose") {
            let re = Regex::new(r"^src[\\/]lib\.rs:\d+:\d+: this is an error$").unwrap();
            assert!(re.is_match(&value));
        } else {
            assert_eq!(value, "this is an error");
        }
    }

    #[test]
    fn test_04() {
        fn bad_value() -> Report<u32> {
            let value = "NaN".parse::<u32>()
                .report(|e| here!(e))?;
            Ok(value)
        }
        let value = bad_value().unwrap_err().to_string(); 
        if cfg!(feature = "disclose") {
            let re = Regex::new(r"^src[\\/]lib\.rs:\d+:\d+: invalid digit found in string$").unwrap();
            assert!(re.is_match(&value));
        } else {
            assert_eq!(value, "invalid digit found in string");
        }
    }

    #[test]
    fn test_05() {
        fn bad_value() -> Report<u32> {
            let value = "NaN".parse::<u32>()
                .report(|e| here!(e, "cannot convert string to a number"))?;
            Ok(value)
        }
        let value = bad_value().unwrap_err().to_string(); 
        if cfg!(feature = "disclose") {
            let re = Regex::new(r"^src[\\/]lib\.rs:\d+:\d+: cannot convert string to a number$").unwrap();
            assert!(re.is_match(&value));
        } else {
            assert_eq!(value, "cannot convert string to a number");
        }
    }

    #[test]
    fn test_06() {
        fn bad_value() -> Report<u32> {
            let value = "NaN".parse::<u32>()
                .report(|e| here!(e, "cannot convert string to a number"))?;
            Ok(value)
        }
        let value = bad_value().unwrap_err().trace(); 
        let values: Vec<&str> = value.split('\n').collect();
        if cfg!(feature = "disclose") {
            let re0 = Regex::new(r"^ 0: src[\\/]lib\.rs:\d+:\d+: cannot convert string to a number$").unwrap();
            let re1 = Regex::new(r"^ 1: invalid digit found in string$").unwrap();
            assert!(re0.is_match(&values[0]));
            assert!(re1.is_match(&values[1]));
        } else {
            assert_eq!(values[0], " 0: cannot convert string to a number");
            assert_eq!(values[1], " 1: invalid digit found in string");
        }
    }

    #[test]
    fn test_07() {
        fn oob() -> Report<u32> {
            let list: Vec<u32> = vec![1, 2, 3, 4,];
            let good_val = *list.get(3).report(|e| here!(e, "Index out of bounds"))?;
            assert_eq!(good_val, 4);
            let bad_val = *list.get(4).report(|e| here!(e, "Index out of bounds"))?;
            Ok(bad_val)
        }
        let bad = oob().unwrap_err();
        let source = bad.source().unwrap();
        if cfg!(feature = "disclose") {
            let re = Regex::new(r"^src[\\/]lib\.rs:\d+:\d+: Index out of bounds$").unwrap();
            assert!(re.is_match(&bad.to_string()));
        } else {
            assert_eq!(bad.to_string(), "Index out of bounds");
        }
        assert_eq!(source.to_string(), "Option::None detected");
    }

    #[test]
    fn test_08() {
        fn bad_value() -> Report<u32> {
            let value = "NaN".parse::<u32>()
                .report(|e| here!(e, "cannot convert string to a number"))?;
            Ok(value)
        }
        fn easy_test() -> Report<u32> {
            let value = bad_value().easy()?;
            Ok(value)
        }
        let value = easy_test().unwrap_err().trace(); 
        let values: Vec<&str> = value.split('\n').collect();
        if cfg!(feature = "disclose") {
            let re0 = Regex::new(r"^ 0: src[\\/]lib\.rs:\d+:\d+: cannot convert string to a number$").unwrap();
            let re1 = Regex::new(r"^ 1: invalid digit found in string$").unwrap();
            assert!(re0.is_match(&values[0]));
            assert!(re1.is_match(&values[1]));
        } else {
            assert_eq!(values[0], " 0: cannot convert string to a number");
            assert_eq!(values[1], " 1: invalid digit found in string");
        }
    }

    #[test]
    fn test_09() {
        fn bad_value() -> Report<u32> {
            let value = "NaN".parse::<u32>()
                .easy()?;
            Ok(value)
        }
        let value = bad_value().unwrap_err().to_string(); 
        assert_eq!(value, "invalid digit found in string");
    }

    #[test]
    fn test_10() {
        fn oob() -> Report<u32> {
            let list: Vec<u32> = vec![1, 2, 3, 4,];
            let good_val = *list.get(3).easy()?;
            assert_eq!(good_val, 4);
            let bad_val = *list.get(4).easy()?;
            Ok(bad_val)
        }
        let value = oob().unwrap_err().to_string(); 
        assert_eq!(value, "Option::None detected");
    }
}
