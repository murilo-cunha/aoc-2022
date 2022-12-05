const INPUT: &str = include_str!("../input.txt");

fn main() {
    let input_vec: Vec<&str> = INPUT.split("\n").filter(|s| !s.is_empty()).collect();
    pt1(input_vec.clone());
    pt2(input_vec.clone());
}

fn pt2(input_vec: Vec<&str>) {
    let mut total_overlap = 0;
    for line in input_vec {
        let (first, second) = get_pairs(line);
        let fvec = expand(first);
        let svec = expand(second);
        let ab = a_overlap_b(fvec.clone(), svec.clone());
        let ba = a_overlap_b(svec.clone(), fvec.clone());
        total_overlap += (ab | ba) as i32;
    }
    println!("{}", total_overlap);
}

fn pt1(input_vec: Vec<&str>) {
    let mut total_overlap = 0;
    for line in input_vec {
        let (first, second) = get_pairs(line);
        let fvec = expand(first);
        let svec = expand(second);
        let ab = a_subset_b(fvec.clone(), svec.clone());
        let ba = a_subset_b(svec.clone(), fvec.clone());
        total_overlap += (ab | ba) as i32;
    }
    println!("{}", total_overlap);
}

fn a_overlap_b(a: Vec<i32>, b: Vec<i32>) -> bool {
    b.iter().any(|item| a.contains(item))
}

#[test]
fn test_a_overlap_b() {
    assert_eq!(a_overlap_b(vec![0, 1, 2], vec![2, 3]), true);
    assert_eq!(a_overlap_b(vec![0, 1, 2], vec![3, 4]), false);
}

fn a_subset_b(a: Vec<i32>, b: Vec<i32>) -> bool {
    b.iter().all(|item| a.contains(item));
}

#[test]
fn test_a_subset_b() {
    assert_eq!(a_subset_b(vec![0, 1, 2], vec![1, 2]), true);
    assert_eq!(a_subset_b(vec![1, 2], vec![1, 2, 3]), false);
}

fn get_pairs(s: &str) -> (&str, &str) {
    let first = s.split(",").nth(0).expect("empty");
    let second = s.split(",").nth(1).expect("one element");
    (first, second)
}

#[test]
fn test_get_pairs() {
    assert_eq!(get_pairs("2-3,4-5"), ("2-3", "4-5"));
}

fn expand(s: &str) -> Vec<i32> {
    let start = s.split("-").nth(0).expect("empty").parse::<i32>().unwrap();
    let end = s
        .split("-")
        .nth(1)
        .expect("one element")
        .parse::<i32>()
        .unwrap();
    (start..end + 1).collect()
}

#[test]
fn test_expand() {
    assert_eq!(expand("2-4"), vec![2, 3, 4]);
}
