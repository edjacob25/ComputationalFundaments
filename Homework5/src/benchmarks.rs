extern crate test;

use super::*;
use test::Bencher;

#[bench]
fn bench_quicksort_2(b: &mut Bencher) {
    let numbers = read_file("random_2".to_string()).unwrap();
    b.iter(|| quicksort(numbers.clone()));
}

#[bench]
fn bench_quicksort_s_2(b: &mut Bencher) {
    let numbers = read_file("sorted_2".to_string()).unwrap();
    b.iter(|| quicksort(numbers.clone()));
}

#[bench]
fn bench_quicksort_r_2(b: &mut Bencher) {
    let numbers = read_file("reverse_2".to_string()).unwrap();
    b.iter(|| quicksort(numbers.clone()));
}

#[bench]
fn bench_quicksort_3(b: &mut Bencher) {
    let numbers = read_file("random_3".to_string()).unwrap();
    b.iter(|| quicksort(numbers.clone()));
}

#[bench]
fn bench_quicksort_s_3(b: &mut Bencher) {
    let numbers = read_file("sorted_3".to_string()).unwrap();
    b.iter(|| quicksort(numbers.clone()));
}

#[bench]
fn bench_quicksort_r_3(b: &mut Bencher) {
    let numbers = read_file("reverse_3".to_string()).unwrap();
    b.iter(|| quicksort(numbers.clone()));
}

#[bench]
fn bench_quicksort_4(b: &mut Bencher) {
    let numbers = read_file("random_4".to_string()).unwrap();
    b.iter(|| quicksort(numbers.clone()));
}
#[bench]
fn bench_quicksort_s_4(b: &mut Bencher) {
    let numbers = read_file("sorted_4".to_string()).unwrap();
    b.iter(|| quicksort(numbers.clone()));
}

#[bench]
fn bench_quicksort_r_4(b: &mut Bencher) {
    let numbers = read_file("reverse_4".to_string()).unwrap();
    b.iter(|| quicksort(numbers.clone()));
}

#[bench]
fn bench_quicksort_5(b: &mut Bencher) {
    let numbers = read_file("random_5".to_string()).unwrap();
    b.iter(|| quicksort(numbers.clone()));
}

#[bench]
fn bench_quicksort_s_5(b: &mut Bencher) {
    let numbers = read_file("sorted_5".to_string()).unwrap();
    b.iter(|| quicksort(numbers.clone()));
}

#[bench]
fn bench_quicksort_r_5(b: &mut Bencher) {
    let numbers = read_file("reverse_5".to_string()).unwrap();
    b.iter(|| quicksort(numbers.clone()));
}
#[bench]
fn bench_quicksort_6(b: &mut Bencher) {
    let numbers = read_file("random_6".to_string()).unwrap();
    b.iter(|| quicksort(numbers.clone()));
}
#[bench]
fn bench_quicksort_s_6(b: &mut Bencher) {
    let numbers = read_file("sorted_6".to_string()).unwrap();
    b.iter(|| quicksort(numbers.clone()));
}

#[bench]
fn bench_quicksort_r_6(b: &mut Bencher) {
    let numbers = read_file("reverse_6".to_string()).unwrap();
    b.iter(|| quicksort(numbers.clone()));
}