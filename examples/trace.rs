// Error Tracing
// =============
//
// Try running this with and without the disclose feature enabled e.g.
// disclose feature enabled:
//
// $ cargo run --example=trace --features=disclose
//
// disclose feature not-enabled:
// $ cargo run --example=trace
//
// Notice that with the dislose feature enabled the trace method produces an error
// string that shows the source file name, line number and column number where the
// error occured

use std::fs::File;

use nuhound::{
    Report,
    here,
    ResultExtension,
};

// Attempt to open a file that doesn't exist
fn level2() -> Report<()> {
    // I assume there is no file in the current directory called this!
    let filename = "xuhgd56qhsl";
    let _file = File::open(filename).report(|e| here!(e, "Failed to open file '{}'", filename))?;
    Ok(())
}

fn level1() -> Report<()> {
    level2().report(|e| here!(e, "Well that's another fine mess"))?;
    Ok(())
}

fn level0() -> Report<()> {
    level1().report(|e| here!(e, "My user interface didn't work"))?;
    Ok(())
}

fn run() -> Report<()> {
    level0().report(|e| here!(e, "Better tell the end user"))?;
    Ok(())
}

fn main() {
    // Using the trace method in conjunction with the disclose feature helps
    // the debugging process by showing exactly where the error occured
    match run() {
        Err(e) => println!("{}", e.trace()),
        Ok(_) => unreachable!(),
    };
}
