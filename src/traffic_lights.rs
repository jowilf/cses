use std::{
    collections::{BTreeMap, BTreeSet},
    io::stdin,
    ops::Bound,
};
fn get_lt<'a, T: Ord>(set: &'a BTreeSet<T>, value: &T) -> Option<&'a T> {
    set.range((Bound::Unbounded, Bound::Excluded(value)))
        .next_back()
}

fn get_gt<'a, T: Ord>(set: &'a BTreeSet<T>, value: &T) -> Option<&'a T> {
    set.range((Bound::Excluded(value), Bound::Unbounded)).next()
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("");
    let nx: Vec<i32> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    let (x, _n) = (nx[0], nx[1]);
    line = "".to_string();
    stdin().read_line(&mut line).expect("");
    let arr: Vec<i32> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();

    let mut lengths = BTreeMap::new();
    let mut lights: BTreeSet<i32> = BTreeSet::new();

    for light in arr {
        let lower = get_lt(&lights, &light).unwrap_or(&0);
        let higher = get_gt(&lights, &light).unwrap_or(&x);
        // println!("low hig: {} {}", lower, higher);
        // println!("lengths: {:?}", lengths);
        // println!("lights: {:?}", lights);

        for x in [light - lower, higher - light] {
            lengths.insert(x, lengths.get(&x).unwrap_or(&0) + 1);
        }
        let old_length = higher - lower;
        let entry = lengths.get_key_value(&old_length);
        if entry.is_some() {
            let (_key, value) = entry.unwrap();
            if value == &1 {
                lengths.remove(&old_length);
            } else {
                lengths.insert(old_length, value - 1);
            }
        }
        lights.insert(light);
        print!("{} ", lengths.iter().next_back().unwrap().0)
    }
}
