# Elevator Excercise

The Elevator excercise is a well-known coding kata. The excercise consists of building a queue of passengers for an elevator in order to determine how many stops the elevator will make until the queue is empty. 

## Running
- `cargo run` to run
- `cargo test` to run tests
- `cargo doc --open` to see docs

## Rules and Constraints
- All passengers will board in the order they arrive to the elevator, there is no optimization involved as in real life this does not happen.
- The elevator has a maximum floor, as a building has a top floor (u8).
- The elevator has a maximum weight tolerance, as an elevator would have in real life (u16).
- Every passenger in the queue must go to a floor equal or greater than 1 and have the top floor as a maximum (u8).
- Every passenger in the queue must weight at least 1kg and a maximum of the elevator maximum weight tolerance (u16).
- The elevator, as in real life, will only make stops while going up. There is no scenario where people board on the way down.
- The stop at the ground floor to pick up more passengers (including the return to ground floor once the queue is empty) must also be counted.

## Result

The output must conain:

- The list of passengers with their according weight and destination.
- The amount of stops the elevator made.

# As a Rust excercise

This repository aims to display the implementation of a number of Rust features/paradigms. This means that perhaps some things could have been different, but there were some "artistic liberties" taken in order to make use of these features/paradigms.

## From the rust book
Topics taken from "Programming Rust 2nd Edition".
https://www.amazon.es/Programming-Rust-Fast-Systems-Development/dp/1492052590

* Types
  * Integer (u8, u16)
  * Bool
  * Vectors
  * String, &str
* Owning, borrowing and dereferencing
  * using &
  * using &mut
  * using mut
  * using * dereferencing
* Expressions
  * type casts
  * if let
  * match
  * loop
* Lifetimes
  * 'running lifetime
* Error Handling
  * Result type -> Ok() / Err()
* Crates and Modules
  * Use of external for random number generation
  * Use of internal for structs and traits
* Structs
  * Person
  * Elevator
* Traits and Generics
  * get_input_by_type_from_user<T>
  * Travel trait for Elevator
* Utility Traits
  * Clone -> #[derive(Clone, Debug)]
* Collections
  * Queue as a Vec<Person>
* Inputs and Outputs
  * Reading input from the user
  * Writing lines
* Rustdocs 
  * Tests in rustdocs
  * Tests with macro-attribute

Pending for a follow-up excercise:
- Generate a thread/channel
- Read/write from external sources (file, db)
- Concurrency
- Asynchronous programming
- Macros


