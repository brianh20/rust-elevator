use std::io::{self, Write};
use std::str::FromStr;

/// Clear screen
pub fn clear_screen() {
    print!("{esc}c", esc = 27 as char);
}

/// Show a header
pub fn show_header(header: &str) {
    println!("{}", header);
    println!(
        "{}",
        std::iter::repeat("=")
            .take(header.len())
            .collect::<String>()
    )
}

/// Display main manu
pub fn show_main_menu() -> u8 {
    show_menu_with_options("Elevator Program", "Exit", "Process Elevator", "")
}

/// Display queue management menu
pub fn show_fill_or_process_menu() -> u8 {
    show_menu_with_options(
        "Elevator Queue",
        "Process Queue",
        "Add Person",
        "Add 10 random people",
    )
}

/// Show a menu with options
/// 
/// Will display a menu with up to three options, mostly leveraged by other functions. See: show_main_menu, show_fill_or_process_menu
/// 
/// OPTIONS:
/// - header: will display a header
/// - option0: will display option for value 0
/// - option1: will display option for value 1
/// - option2: will display option for value 2
pub fn show_menu_with_options(header: &str, option0: &str, option1: &str, option2: &str) -> u8 {
    // show GUI
    clear_screen();
    show_header(&header);
    if option2 != "" {
        println!("2) {}", &option2);
    }
    println!("1) {}", &option1);
    println!("0) {}", &option0);
    print!("Select an option: ");

    // ask until a valid value is received
    let option = get_input_by_type_from_user::<u8>(&false, &true, &"", &"");
    option
}

/// Get an input from a specific Type<T>
/// 
/// The purpose of this is that in some cases we will want a specific value
/// such as a number and not a string (for example)
/// 
/// OPTIONS:
/// - clear: will clear screen
/// - allow zero: will allow zero as a value
/// - header: will add a header to the prompt
/// - prompt: will add a description to the request
pub fn get_input_by_type_from_user<T: FromStr>(
    clear: &bool,
    allow_zero: &bool,
    header: &str,
    prompt: &str,
) -> T {
    // ask until a valid value is received according to type
    let value = get_value_from_user(&clear, &header, &prompt);
    if !allow_zero && value == "0" {
        println!("Zero value not allowed");
        return get_input_by_type_from_user::<T>(&clear, &allow_zero, &header, &prompt);
    }

    let value = value.parse::<T>();
    let value = match value {
        Ok(x) => x,
        Err(_error) => get_input_by_type_from_user::<T>(&clear, &allow_zero, &header, &prompt),
    };
    value
}

/// Request a value from the user
/// 
/// Mostly leveraged by all other functions. This is mainly a display, to cycle
/// while values are invalid in invoking functions which carry logic (See: get_input_by_type_from_user)
///
/// # OPTIONS:
/// - clear: will clear the screen
/// - header: will add a header to the input request
/// - prompt: will add a description to the value being requested
pub fn get_value_from_user(clear: &bool, header: &str, prompt: &str) -> String {
    if *clear {
        clear_screen();
    }
    if header != "" {
        show_header(&header);
    }
    if prompt != "" {
        print!("{} ", prompt);
    }

    get_input_from_user()
}


/// Read a line input.
/// 
/// This only reads a line, function is leveraged by previous GUI functions.
pub fn get_input_from_user() -> String {
    // variable declarations
    let mut line = String::new();
    // prompt user and return value
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();

    // return trimmed string
    String::from(line.trim())
}


