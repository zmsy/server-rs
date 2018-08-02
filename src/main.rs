
extern crate rand;
use std::io::{self, Write};
use rand::Rng;


fn print_vector(array: &Vec<u16>) {
    let mut i: usize = 0;
    while i < array.len() {
        print!("{}", &array[i]);
        print!(", ");
        i += 1;
    }
    io::stdout().flush().unwrap();
    println!();
    println!();
}


// applies a bubble sort to the vector
fn bubble_sort(sortvec: &mut Vec<u16>) {
    let vec_length = sortvec.len();

    // i loop
    let mut i = 0;
    while i < vec_length {

        let mut j = 0;
        while j < vec_length {
            if sortvec[i] < sortvec[j] {
                let tmp = sortvec[i];
                sortvec[i] = sortvec[j];
                sortvec[j] = tmp;
            }
            j += 1;
        }
        i += 1;
    }
}


// applies a merge sort to the array
fn merge_sort(sortvec: &Vec<u16>) {
    if sortvec.len() == 1 {
        return;
    }

    else {
        let mid = sortvec.len() / 2;
        let mut l1 = vec![0; sortvec[..mid].len()];
        let mut l2 = vec![0; sortvec[mid..].len()];
        l1.clone_from_slice(&sortvec[..mid]);
        l2.clone_from_slice(&sortvec[mid..]);
        println!("Left side: ");
        print_vector(&l1);
        println!("Right side: ");
        print_vector(&l2);
    }
}

// takes two sorted arrays and returns a merged version of them
fn merge(v1: &mut Vec<u16>, v2: &mut Vec<u16>) -> Vec<u16> {
    let mut v_combined: Vec<u16> = Vec::new();

    while v1.len() > 0 && v2.len() > 0 {
        if v1.get(v1.len() - 1) <= v2.get(v2.len() - 1) {
            v_combined.push(v1.pop().unwrap());
        } else {
            v_combined.push(v2.pop().unwrap());
        }
    }

    while v1.len() > 0 {
        v_combined.push(v1.pop().unwrap());
    }

    while v2.len() > 0 {
        v_combined.push(v2.pop().unwrap());
    }

    return v_combined;
}


// create a random array and invoke bubble sort
fn main() {

    // generate and populate an array with random integers
    let mut vec1: Vec<u16> = Vec::new(); 
    let mut vec2: Vec<u16> = Vec::new(); 
    let mut y = 0;
    while y < 20 {
        let num1 = rand::thread_rng().gen_range(0, 100);
        let num2 = rand::thread_rng().gen_range(0, 100);
        vec1.push(num1);
        vec2.push(num2);
        y += 1;
    }

    println!("Prior to sorting: ");
    println!("---- VECTOR 1 ----");
    print_vector(&vec1);
    merge_sort(&vec1);
    println!("---- VECTOR 2 ----");
    print_vector(&vec2);
    merge_sort(&vec2);

    println!("After sorting: ");
    bubble_sort(&mut vec1);
    bubble_sort(&mut vec2);
    println!("---- VECTOR 1 ----");
    print_vector(&vec1);
    println!("---- VECTOR 2 ----");
    print_vector(&vec2);

    let vec_c = merge(&mut vec1, &mut vec2);

    println!("Combined vector: ");
    print_vector(&vec_c);
}
