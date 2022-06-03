use crate::gui;
use rand::{thread_rng, Rng};

/// Generate a random person for the queue
/// 
/// Used to test the application faster by generating a person within constraints
/// 
/// # Usage
/// 
/// ```
/// let random_person = get_random_person(10, 100);
/// ```
/// # DISCLAIMER: Doctests won't run if the entrypoint is main.rs
/// Open issue on github: <https://github.com/rust-lang/rust/issues/50784>
/// Writtent rust test at end of file.
pub fn get_random_person(max_floors: &u8, max_weight: &u16) -> Person {
    let mut rng = thread_rng();
    let weight = rng.gen_range(1..=*max_weight as u8);
    let floor = rng.gen_range(1..=*max_floors as u8);
    create_person(&weight, &floor)
}

/// Create a person from user input
pub fn get_person(max_floors: &u8, max_weight: &u16) -> Person {
    let weight = get_weight(&true, &max_weight);
    let floor = get_floor(&true, &max_floors);
    create_person(&weight, &floor)
}

/// Prompt to get a weight for a person
pub fn get_weight(clear: &bool, max_weight: &u16) -> u8 {
    let weight = gui::get_input_by_type_from_user::<u8>(
        &clear,
        &false,
        &"Config Person in line",
        &"Weight:",
    );

    let weight = if weight as u16 > *max_weight {
        println!("The person's weight exceeds the elevator's max weight tolerance.");
        get_weight(&false, &max_weight)
    } else {
        weight
    };
    weight
}

/// Prompt to get the floor destination for a person
pub fn get_floor(clear: &bool, max_floors: &u8) -> u8 {
    let floor = gui::get_input_by_type_from_user::<u8>(
        &clear,
        &false,
        &"Config Person in line",
        &"Floor destination:",
    );

    let floor = if floor > *max_floors {
        println!("Floor exceeds Maximum number of floors.");
        get_floor(&false, &max_floors)
    } else {
        floor
    };
    floor
}

/// Instantiate a Person
pub fn create_person(weight: &u8, floor: &u8) -> Person {
    Person {
        weight: *weight,
        floor: *floor,
    }
}

/// Person Struct
#[derive(Clone, Debug)]
pub struct Person {
    pub weight: u8,
    pub floor: u8,
}


#[test]
fn test_create_random_person() {
    let random_person= get_random_person(&10, &100);
    println!("Random Person: {:?}", random_person);
    assert!(random_person.floor > 0 && random_person.floor <= 10);
    assert!(random_person.weight > 0 && random_person.weight <= 100);
}

#[test]
fn test_create_person() {
    let person = create_person(&100, &10);
    assert_eq!(person.floor, 10);
    assert_eq!(person.weight, 100);
}
