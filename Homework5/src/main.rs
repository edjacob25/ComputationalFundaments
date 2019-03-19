#![feature(test)]
extern crate rand;

use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};


fn main() {
    println!("Hello, world!");

    for i in 2..7 {
        let mut arr = create_numbers_array_base_10(i);
    //print_vec(&arr);
        let mut random = "random_".to_string();
        random.push_str(&i.to_string());
        write_file(&arr, random).unwrap();

        let mut sorted = "sorted_".to_string();
        sorted.push_str(&i.to_string());
        arr.sort();
        write_file(&arr, sorted).unwrap();

        let mut reverse = "reverse_".to_string();
        reverse.push_str(&i.to_string());
        arr.reverse();
        write_file(&arr, reverse).unwrap();
    }
    
    //let sorted_arr = quicksort(arr);
    //println!("Sorted array first position is {}", &sorted_arr[0])
    //print_vec(&sorted_arr);
}

fn print_vec(arr: &Vec<u32>){
    print!("Array is [");
    for i in arr {
        print!("{},", i);
    }
    print!("]\n");
}

fn create_numbers_array_base_10(power: u32) -> Vec<u32>{
    let size = 10_u32.pow(power);
    return create_numbers_array(size);
}

fn create_numbers_array(elements: u32) -> Vec<u32>{
    let mut v : Vec<u32> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..elements {
        v.push(rng.gen());
    } 
    v
}

fn write_file(v : &Vec<u32>, path: String) -> Result<(),Box<Error>> {

    let mut output = File::create(&path)?;
    for i in v {
        write!(output, "{}\n", i)?;
    }
    Ok(())
}

fn read_file(path: String) -> Result<Vec<u32>, Box<Error>>{
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    
    let mut vec = Vec::new();
    for line in buffered.lines() {
        let int = line?.parse::<u32>()?;
        vec.push(int);
    }
    return Ok(vec);
}


fn quicksort(arr: Vec<u32>) -> Vec<u32>{
    if arr.len() <= 1
    {
        return arr;
    }

    let mut rng = rand::thread_rng();
    let range = Uniform::from(0..arr.len() -1);
    let pivot = arr[range.sample(&mut rng)];
    //println!("Pivot is {}", pivot);

    let mut less = Vec::new();
    let mut more = Vec::new();
    let mut same = Vec::new();

    for item in arr {
        if item < pivot {
            less.push(item);
        }
        if item == pivot{
            same.push(item);
        }
        if item > pivot {
            more.push(item);
        }
    }

    let mut less = quicksort(less);
    let mut more = quicksort(more);
    
    less.append(&mut same);
    less.append(&mut more);
    return less;
}

#[cfg(test)]
mod benchmarks;