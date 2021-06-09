extern crate rand;
extern crate time;

use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn create_files() {
    for i in 2..7 {
        let mut arr = create_numbers_array_base_10(i);
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
}

pub fn print_vec(arr: &[u32]) {
    print!("Array is [");
    for i in arr {
        print!("{},", i);
    }
    println!("]");
}

pub fn create_numbers_array_base_10(power: u32) -> Vec<u32> {
    let size = 10_u32.pow(power);
    return create_numbers_array(size);
}

fn create_numbers_array(elements: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..elements {
        v.push(rng.gen());
    }
    v
}

fn write_file(v: &[u32], path: String) -> Result<(), Box<Error>> {
    let mut output = File::create(&path)?;
    for i in v {
        writeln!(output, "{}", i)?;
    }
    Ok(())
}

pub fn read_file(path: String) -> Result<Vec<u32>, Box<Error>> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut vec = Vec::new();
    for line in buffered.lines() {
        let int = line?.parse::<u32>()?;
        vec.push(int);
    }
    return Ok(vec);
}

pub fn quicksort(arr: Vec<u32>) -> Vec<u32> {
    if arr.len() <= 1 {
        return arr;
    }

    let mut rng = rand::thread_rng();
    let range = Uniform::from(0..arr.len() - 1);
    let pivot = arr[range.sample(&mut rng)];
    //println!("Pivot is {}", pivot);

    let mut less = Vec::new();
    let mut more = Vec::new();
    let mut same = Vec::new();

    for item in arr {
        if item < pivot {
            less.push(item);
        }
        if item == pivot {
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

pub fn mergesort(arr: &mut Vec<u32>) {
    let size: usize = arr.len();
    let mut worker: Vec<u32> = vec![0; size];

    merge_split(arr, 0, size, &mut worker);
}

fn merge_split(l1: &mut Vec<u32>, s: usize, e: usize, l2: &mut Vec<u32>) {
    if e - s > 1 {
        let m: usize = (e + s) / 2;

        merge_split(l1, s, m, l2);
        merge_split(l1, m, e, l2);
        merge(l1, s, m, e, l2);
        merge_copy(l1, s, e, l2);
    }
}

fn merge_copy(l1: &mut Vec<u32>, s: usize, e: usize, l2: &Vec<u32>) {
    (s..e).for_each(|i| l1[i] = l2[i]);
}

fn merge(l1: &Vec<u32>, s: usize, m: usize, e: usize, l2: &mut Vec<u32>) {
    let mut ptr1 = s;
    let mut ptr2 = m;

    for i in s..e {
        // Continue to compare elements within each sub-list until one sub-list
        // is exhausted. If a sub-list is exhausted, then the remaing elements
        // in the other list are copied over assuming they're already in order.
        if (ptr1 < m) && (ptr2 >= e || l1[ptr1] <= l1[ptr2]) {
            l2[i] = l1[ptr1];
            ptr1 += 1;
        } else {
            l2[i] = l1[ptr2];
            ptr2 += 1;
        }
    }
}

fn down_heap(a: &mut [u32], i: usize, n: usize) {
    let mut p = i;

    loop {
        let q = p;

        let left = (q << 1) + 1;
        if left < n && a[p] < a[left] {
            p = left
        }

        let right = (q << 1) + 2;
        if right < n && a[p] < a[right] {
            p = right
        }

        a.swap(q, p);

        if q == p {
            break;
        }
    }
}

pub fn heapsort(a: &mut [u32]) {
    let len = a.len();

    for i in (0..len / 2 + 1).rev() {
        down_heap(a, i, len);
    }

    for i in (1..len).rev() {
        a.swap(0, i);
        down_heap(a, 0, i - 1);
    }
}
