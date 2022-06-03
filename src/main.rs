mod elevator;
mod gui;
mod person;
mod queue;

use elevator::Travel;

/// Common Elevator kata
fn main() {
    // play or exit
    let option = gui::show_main_menu();
    if option != 1 {
        println!("Exiting");
        return;
    }

    // setup elevator constraints
    let elevator = elevator::config_elevator();
    println!(
        "Set floors to: {} \nSet max weight to: {}\n",
        elevator.max_floors, elevator.max_weight
    );

    // setup people queue
    let queue = queue::fill_queue(&elevator);

    // process queue without cloning values in processing function
    // in order to keep a history - could be removed if
    // decided not to keep history and just bestow ownership
    let stops = elevator.get_stops(&queue);

    println!("Stops: {}\n\nPress Enter to continue", stops);
    gui::get_input_from_user();

    // replay or exit
    main();
}
