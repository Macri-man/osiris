#![feature(globs)]
use std::io::timer;
use std::time::*;
fn main() {
    println!("This is a game. Prepare to have fun.");
	timer::sleep(Duration::seconds(2));
	println!("You're having fun now. Goodbye!");
}
