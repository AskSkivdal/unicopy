extern crate copypasta;

use dialog::DialogBox;
use copypasta::{ClipboardContext, ClipboardProvider};




fn main() {
    let input = dialog::Input::new("Please the hex of the unicode.")
        .title("Unicode")
        .show()
        .expect("Could not display dialog box");
    let code = match input {
        Some(name) => name,
        None => return,
    };


    let val = u32::from_str_radix(&code, 16).expect("Could not convert the input to u32");

    let character = char::from_u32(val).expect("Not a valid character");
    println!("{}", character);

    let mut ctx = ClipboardContext::new().unwrap();
    ctx.get_contents().unwrap();
    ctx.set_contents(character.to_string()).unwrap();

    let content = ctx.get_contents().unwrap();

    println!("{}", content);
}
