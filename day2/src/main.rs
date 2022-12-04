use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    pt1();
}

fn pt1() {
    // misunderstood the problem - thought XYZ could be anything 😅
    let second_keys = vec!["X", "Y", "Z"];
    const VAL_COMB: [[&str; 3]; 6] = [
        ["r", "p", "s"],
        ["p", "s", "r"],
        ["s", "r", "p"],
        ["p", "r", "s"],
        ["s", "p", "r"],
        ["r", "s", "p"],
    ];

    for i in 0..6 {
        let first_map = HashMap::from([("A", "r"), ("B", "p"), ("C", "s")]);
        let second_map: HashMap<&str, &str> = second_keys
            .iter()
            .map(|s| s.as_ref())
            .zip(VAL_COMB[i])
            .into_iter()
            .collect();
        println!("{:?}", first_map);
        println!("{:?}", second_map);

        let map = first_map.into_iter().chain(second_map).collect();
        let decoded = decode_games(INPUT, map);

        let mut pts = 0;
        for game in decoded.split("\n") {
            if !game.is_empty() {
                let first: &str = game.clone().split(" ").nth(0).unwrap();
                let second: &str = game.clone().split(" ").nth(1).unwrap();
                pts += pts_second(first, second);
                pts += pts_played(second);
            }
        }
        println!("{}", pts);
    }
}

fn decode_games(txt: &str, map: HashMap<&str, &str>) -> String {
    let mut exp = txt.to_string();
    for (k, v) in map {
        exp = exp.replace(k, v)
    }
    exp
}

fn pts_second(first: &str, second: &str) -> u32 {
    assert!(["r","p","s"].contains(&first) && ["r","p","s"].contains(&second));
    if first.eq(second) {
        return 3; // draw
    }
    if [("r", "s"), ("s", "p"), ("p", "r")].contains(&(&first, &second)) {
        // first wins
        return 0;
    }
    6 // second wins
}

fn pts_played(s: &str) -> u32 {
    assert!(["r","p","s"].contains(&s));
    if s.eq("r") {
        return 1;
    }
    if s.eq("p") {
        return 2;
    }
    3 // scissors
}
