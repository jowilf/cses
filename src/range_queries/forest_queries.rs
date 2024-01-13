use std::io::{self, BufRead, BufReader};

fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut input_line = String::new();
    reader.read_line(&mut input_line).unwrap();
    let mut input_iter = input_line.split_whitespace();
    let n: usize = input_iter.next().unwrap().parse().unwrap();
    let q: usize = input_iter.next().unwrap().parse().unwrap();

    let mut grid = vec![vec![0; n]; n];
    let mut acc = vec![vec![0; n]; n];
 
    for i in 0..n {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let chars: Vec<char> = line.trim().chars().collect();
        for j in 0..n {
            grid[i][j] = if chars[j] == '*' { 1 } else { 0 };
        }
    }

    for i in 0..n {
        for j in 0..n {
            acc[i][j] = grid[i][j];
            if i > 0 {
                acc[i][j] += acc[i - 1][j];
            }
            if j > 0 {
                acc[i][j] += acc[i][j - 1];
            }
            if i > 0 && j > 0 {
                acc[i][j] -= acc[i - 1][j - 1];
            }
        }
    }

    for _ in 0..q {
        let mut query_line = String::new();
        reader.read_line(&mut query_line).unwrap();
        let mut query_iter = query_line.split_whitespace();
        let i1: usize = query_iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let j1: usize = query_iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let i2: usize = query_iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let j2: usize = query_iter.next().unwrap().parse::<usize>().unwrap() - 1;

        let mut ans = acc[i2][j2];
        if j1 > 0 {
            ans -= acc[i2][j1 - 1];
        }
        if i1 > 0 {
            ans -= acc[i1 - 1][j2];
        }
        if i1 > 0 && j1 > 0 {
            ans += acc[i1 - 1][j1 - 1];
        }
        println!("{}", ans);
    }
}
