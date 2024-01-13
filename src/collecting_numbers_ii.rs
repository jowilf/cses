use std::{collections::HashSet, io::stdin};

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("-error-");
    let nm: Vec<i32> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    let (n, m) = (nm[0], nm[1]);
    line = "".to_string();
    stdin().read_line(&mut line).expect("-error-");
    let arr: Vec<i32> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    let mut values: [i32; 200002] = [-1; 200002];
    let mut position_of: [i32; 200002] = [0; 200002];
    for (i, v) in arr.into_iter().enumerate() {
        position_of[v as usize] = (i + 1) as i32;
        values[i + 1] = v;
    }
    let mut ans = 1;
    let mut previous_idx = -1;
    for i in 1..(n + 1) {
        if position_of[i as usize] < previous_idx {
            ans += 1;
        }
        previous_idx = position_of[i as usize];
    }

    let mut affected: HashSet<(i32, i32)> = HashSet::new();
    for _ in 0..m {
        line = "".to_string();
        stdin().read_line(&mut line).expect("-error-");
        let ab: Vec<i32> = line
            .split_whitespace()
            .filter_map(|w| w.parse().ok())
            .collect();
        let (l, r) = (ab[0], ab[1]);
        let (a, b) = (values[l as usize], values[r as usize]);
        // println!("\na,b {} {} {:?}\n", a, b, &values[0 .. 6]);
        for x in [a, b] {
            if x > 1 {
                affected.insert((x - 1, x));
            }
            if x < n {
                affected.insert((x, x + 1));
            }
        }
        for (i, j) in affected.clone() {
            if position_of[i as usize] > position_of[j as usize] {
                ans -= 1;
            }
        }
        values[l as usize] = b;
        values[r as usize] = a;
        position_of[a as usize] = r;
        position_of[b as usize] = l;

        for (i, j) in affected.clone() {
            if position_of[i as usize] > position_of[j as usize] {
                ans += 1;
            }
        }
        // println!("{:?}", affected);
        println!("{}", ans);
        affected.clear();
    }
}
