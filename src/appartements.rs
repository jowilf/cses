fn main() {
    use std::io::stdin;
    let mut line = String::new();
    stdin().read_line(&mut line);
    let nmk: Vec<i32> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    let (n, m, k) = (nmk[0], nmk[1], nmk[2]);
    line = "".to_string();
    stdin().read_line(&mut line);
    let mut applicants: Vec<i32> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    line = "".to_string();
    stdin().read_line(&mut line);
    let mut apartements: Vec<i32> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    applicants.sort();
    apartements.sort();
    let (mut i, mut j, mut ans) = (0, 0, 0);
    while i < n && j < m {
        if (applicants[i as usize] - apartements[j as usize]).abs() <= k {
            ans += 1;
            j += 1;
            i += 1;
        } else if (applicants[i as usize] - k) > apartements[j as usize] {
            j += 1;
        } else {
            i += 1;
        }
    }
    // println!("{:?}", applicants);
    // println!("{:?}", apartements);
    println!("{}", ans);
}
