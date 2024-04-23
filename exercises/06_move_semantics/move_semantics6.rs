// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

/*In string_uppercase function, data is a String type and 
fow allowing you to take the ownership and mofidy it by calling 'to_uppercase' function / method ? 
*/

fn main() {
    let data = "Rust is great!".to_string();

    get_char(data.clone());

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}

