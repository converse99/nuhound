use nuhound::{
    Report,
    here,
    ResultExtension,
};

fn get_user_input() -> Report<u32> {
    let text = "99s";
    let val = text.parse::<u32>()
        .report(|e| here!(e, "Unable to parse string: {text}"))?;
    // This can be read as report error here
    Ok(val)
}

fn ask_user_for_value() -> Report<u32> {
    let my_val = get_user_input()
        .report(|e| here!(e, "You've entered an incorrect text string"))?;
    Ok(my_val)
}

fn main() {
    match ask_user_for_value() {
        Ok(val) => println!("We shouldn't be able see this value: {val}"),
        Err(e) => {
            if cfg!(feature = "disclose") {
                eprintln!("{}", e.trace());
            } else {
                eprintln!("{e}");
            }
        },
    }
}
