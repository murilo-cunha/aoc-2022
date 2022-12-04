use std::{collections::HashSet, str::Split};

const INPUT: &str = include_str!("../input.txt");
const PRIO: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let input_vec: Vec<&str> = INPUT.split("\n").filter(|l| !l.is_empty()).collect();
    let prio_vec: Vec<&str> = PRIO.split("").collect();
    pt1(input_vec, prio_vec);
}

fn pt1(input_vec: Vec<&str>, prio_vec: Vec<&str>) {
    let mut total_prio = 0;
    for rucksack in input_vec {
        total_prio += get_prio_sack(rucksack, prio_vec.clone());
    }
    println!("{}", total_prio);
}

fn get_prio_sack(seq: &str, prio_vec: Vec<&str>) -> usize {
    let (p1, p2) = split_half(seq);
    let hash1: HashSet<&str> = p1.split("").collect();
    let hash2: HashSet<&str> = p2.split("").collect();
    let intersection = intersect(hash1, hash2);

    item2prio(prio_vec, &intersection)
}
fn item2prio(prio_vec: Vec<&str>, el: &str) -> usize {
    prio_vec.iter().position(|r| r.eq(&el)).unwrap()
}
fn split_half(seq: &str) -> (&str, &str) {
    let len = seq.len();
    assert!(len > 0);
    seq.split_at(len / 2)
}

fn intersect(hash1: HashSet<&str>, hash2: HashSet<&str>) -> String {
    hash1
        .intersection(&hash2)
        .map(|i| *i)
        .filter(|s| !s.is_empty())
        .nth(0)
        .unwrap()
        .to_string()
}
