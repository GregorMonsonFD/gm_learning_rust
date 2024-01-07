use std::io;
use std::time::{Duration, Instant};
// use std::env;
use std::collections::HashMap;

fn fibbonaci_seed(first_value: u128, second_value: u128) -> HashMap<u128, u128> {

    let mut fibbonaci_hashmap = HashMap::<u128, u128>::new();

    fibbonaci_hashmap.insert(0, first_value);
    fibbonaci_hashmap.insert(1, second_value);

    return fibbonaci_hashmap;

}

fn fibonacci_adder(mut fibbonaci_hashmap: HashMap<u128, u128>, requested_index: u128, count: u128) -> HashMap<u128, u128> {

    let previous_fib_number:u128;
    let curent_fib_number:u128;
    let next_fib_number:u128;
    
    previous_fib_number = *fibbonaci_hashmap.get(&(count)).unwrap();
    curent_fib_number = *fibbonaci_hashmap.get(&(count + 1)).unwrap();
    next_fib_number = previous_fib_number + curent_fib_number;

    println!("{}, {}, {}", previous_fib_number, curent_fib_number, next_fib_number);

    fibbonaci_hashmap.insert(count + 2, next_fib_number);

    if count + 2 < requested_index {

        fibbonaci_hashmap = fibonacci_adder(
            fibbonaci_hashmap
            , requested_index
            , count + 1
        )
    };

    return fibbonaci_hashmap;
}

fn fibonacci_call_index(index: u128) -> u128 {
    let fibbonaci_hashmap = fibbonaci_seed(0, 1);

    let fibbonaci_hashmap = fibonacci_adder(fibbonaci_hashmap, index, 0);

    return *fibbonaci_hashmap.get(&index).unwrap();
    
}

fn main() {
    // env::set_var("RUST_BACKTRACE", "full");
    println!("What fibonacci index would you like to return?: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let index: u128 = input.trim().parse().expect("Input not an integer");

    let start = Instant::now();
    println!("Index {} = {}", index, fibonacci_call_index(index));

    let duration = start.elapsed();

    println!("Time elapsed in fibonacci_call_index() is: {:?}", duration);
}