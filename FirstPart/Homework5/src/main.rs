use homework_5::*;
use std::ops::Sub;

fn main() {
    println!("Hello, world!");

    // let mut vec = create_numbers_array_base_10(2);
    // print_vec(&vec);
    // heapsort(&mut vec);
    // print_vec(&vec);
    let mut vec = create_numbers_array_base_10(9);
    println!("Created numbers");
    let time1 = time::now();
    mergesort(&mut vec);
    //print_vec(&vec);
    let time2 = time::now();
    let duration = time2.sub(time1);
    println!("The duration is {}", duration);

    //let vec = create_numbers_array_base_10(5);
    //let ordered = quicksort(vec);
    //print_vec(&ordered);

    //create_files();
}
