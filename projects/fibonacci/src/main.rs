extern crate num;

use std::io;
use std::time::{Instant};
use num::BigUint;
use num::{Zero, One};
// use std::env;
use std::collections::HashMap;

fn fibbonaci_seed(first_value: BigUint, second_value: BigUint) -> HashMap<u32, BigUint> {

    let mut fibbonaci_hashmap = HashMap::<u32, BigUint>::new();

    fibbonaci_hashmap.insert(0, first_value);
    fibbonaci_hashmap.insert(1, second_value);

    fibbonaci_hashmap
}

fn fibonacci_adder(mut fibbonaci_hashmap: HashMap<u32, BigUint>, requested_index: u32, count: u32) -> HashMap<u32, BigUint> {
    
    let previous_fib_number = fibbonaci_hashmap.get(&(count)).unwrap().clone();
    let curent_fib_number = fibbonaci_hashmap.get(&(count + 1)).unwrap().clone();
    let next_fib_number = previous_fib_number.clone() + curent_fib_number.clone();

    // println!("{}, {}, {}", previous_fib_number, curent_fib_number, next_fib_number);

    fibbonaci_hashmap.insert(count + 2, next_fib_number);

    if count + 2 < requested_index {

        fibbonaci_hashmap = fibonacci_adder(
            fibbonaci_hashmap
            , requested_index
            , count + 1
        )
    };

    fibbonaci_hashmap
}

fn fibonacci_call_index(index: u32) -> BigUint {
    let f0: BigUint = Zero::zero();
    let f1: BigUint = One::one();

    let fibbonaci_hashmap = fibbonaci_seed( f0, f1 );

    let fibbonaci_hashmap = fibonacci_adder(fibbonaci_hashmap, index, 0);

    return fibbonaci_hashmap.get(&index).unwrap().clone();
    
}

fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
    println!("What fibonacci index would you like to return?: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let index: u32 = input.trim().parse().expect("Input not an integer");

    let start = Instant::now();
    println!("Index {} = {}", index, fibonacci_call_index(index));

    let duration = start.elapsed();

    println!("Time elapsed in fibonacci_call_index() is: {:?}", duration);
}