// The 'easy' method will convert any error type into an Nuhound error with minimal
// coding required. It's a quick and dirty way to carry out this conversion.
// The question mark operator extracts the Err value and returns it to the calling context.
// Notice how we use 'Report' as the return value as a further simplification.
// You can, if you wish use Result as a return value in the normal way.
// i.e. Result<u32, Nuhound> (remembering to add Nuhound to the use clause)
// use nuhound::{
//    Report,
//    ResultExtension,
//    OptionExtension,
//    Nuhound
// };
//
// In addition the 'easy' method will convert an Option::None into an Nuhound error.
// An Option::Some will be converted to a Result::Ok

use nuhound::{
    here,
    Report,
    ResultExtension,
    OptionExtension,
};

fn get_some() -> Report<u32> {
    let short_vector = vec![9, 8 ,7];
    // The get returns an Option and easy converts it to a Result.
    // If the get returns None then an error is generated.
    let value = *short_vector.get(5).report(|e| here!(e, "Index out of bounds"))?;
    Ok(value)
}

fn input_data() -> Report<u32> {
    // This is Ok
    let value = "9090".parse::<u32>().easy()?;
    println!("Easily extract an Ok result using the question mark operator [value = {value}]");

    // This is also Ok
    let value = get_some()?;
    println!("Convert an Option to a Result using the easy method [value = {value}]");

    // This is an Err
    let value = "NaN".parse::<u32>().easy()?;
    Ok(value)
}

fn run() -> Report<u32> {
    let val = input_data().easy()?;
    Ok(val)
}

fn main() {
    println!("Any errors are returned by the main function. This example will always fail");
    match run() {
        Ok(val) => println!("This example should always fail so we shouldn't see this value: {val}"),
        Err(e) => {
            if cfg!(feature = "disclose") {
                eprintln!("{}", e.trace());
            } else {
                eprintln!("{e}");
            }
        },
    }
}
