fn is_terminating(d: u16, init_value: u16, mut terminating_outcome: bool) -> bool {
    let mut modulo_index: u16;

    for index in 2..d {
        if index % 2 == 0 || index % 5 == 0 {continue};
        modulo_index = d % index;

        // println!("Mid Loop: {}, {}, {}", d, terminating_outcome, index);

        if modulo_index == 0 {
            terminating_outcome = is_terminating(d/index, init_value, terminating_outcome)
        }

        if terminating_outcome == false {return false}
    }

    // println!("Loop Exit: {}, {}, {}", d, terminating_outcome, d);

    return d == init_value && terminating_outcome 

}

fn find_cycle_len(fraction_string: String) -> u16 {
    let mut cycle_length: u16 = 0;




    cycle_length
}

fn max_reciprocal_cycle(max_index: u16) -> u16 {
    let mut cycle_length: u16 = 0;
    let mut fraction: f64;
    let mut fraction_string: String;

    for d in 2..max_index {
        if is_terminating(d, d, true) {
            fraction = (1 as f64)/(d as f64);
            fraction_string = fraction.to_string();

            find_cycle_len(fraction_string);
        }
    }

    cycle_length
}

fn main() {
    // let max_d: u16 = max_reciprocal_cycle(1000);

    println!("{}", is_terminating(15, 15, true))
}
