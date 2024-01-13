use std::{
    collections::{HashMap, LinkedList, VecDeque},
    io::stdin,
};

// fn dfs(
//     x: usize,
//     y: usize,
//     x2: usize,
//     y2: usize,
//     n: i32,
//     m: i32,
//     visited: &mut [[bool; 1001]; 1001],
//     paths: &mut VecDeque<char>,
// ) -> bool {
//     println!("dfs {:?} {:?}", (x, y, x2, y2), paths);
//     visited[x][y] = true;
//     if (x == x2 && y == y2) {
//         return true;
//     }
//     let d: [(i32, i32, char); 4] = [(0, 1, 'D'), (1, 0, 'R'), (-1, 0, 'L'), (0, -1, 'U')];
//     for k in 0..4 {
//         let (xx, yy) = ((x as i32) + d[k].0, (y as i32) + d[k].1);
//         if xx >= 0 && xx < n && yy >= 0 && yy < m && !visited[xx as usize][yy as usize] {
//             paths.push_back(d[k].2);
//             if dfs(xx as usize, yy as usize, x2, y2, n, m, visited, paths) {
//                 return true;
//             }
//             paths.pop_back();
//         }
//     }
//     return false;
// }

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("");
    let mut iter = line.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let m: i32 = iter.next().unwrap().parse().unwrap();
    // let mut grid: [[char; 1001]; 1001] = [['#'; 1001]; 1001];
    let mut visited: [[bool; 1001]; 1001] = [[false; 1001]; 1001];
    // let mut predecessors: HashMap<(i32, i32), (i32, i32, char)> = HashMap::new();
    // let paths: VecDeque<char> = VecDeque::new();
    let (mut x, mut x2, mut y, mut y2) = (0, 0, 0, 0);
    for i in 0..n as usize {
        line = "".to_string();
        stdin().read_line(&mut line).expect("");
        for (j, c) in line.chars().enumerate() {
            // grid[i][j] = c;
            visited[i][j] = c == '#';
            if c == 'A' {
                (x, y) = (i as i32, j as i32);
            } else if c == 'B' {
                (x2, y2) = (i as i32, j as i32);
            }
        }
    }
    let d: [(i32, i32, char); 4] = [(0, 1, 'R'), (1, 0, 'D'), (-1, 0, 'U'), (0, -1, 'L')];
    let mut queue: Vec<(i32, i32)> = Vec::new();
    queue.push((x, y));
    while !queue.is_empty() {
        let v = queue.first().unwrap();
        (x, y) = (v.0, v.1);
        queue.remove(0);
        for k in 0..4 {
            let (xx, yy) = (x + d[k].0, y + d[k].1);
            if xx >= 0 && xx < n && yy >= 0 && yy < m && !visited[xx as usize][yy as usize] {
                visited[x as usize][y as usize] = true;
                // predecessors[xx as usize][yy as usize] = (x, y, d[k].2);
                // if xx == x2 && yy == y2 {
                //     println!("YES");
                //     // println!("{:?}", predecessors);
                //     let mut paths: VecDeque<char> = VecDeque::new();
                //     // paths.push_back(d[k].2);
                //     // // print!("{}", d[k].2);
                //     // while (predecessors.contains_key(&(x, y))) {
                //     //     let v = predecessors.get(&(x, y)).unwrap();
                //     //     // println!("{:?}", v);
                //     //     // print!("{}", v.1);
                //     //     paths.push_front(v.2);
                //     //     (x, y) = (v.0, v.1);
                //     // }
                //     // println!("{}", paths.len());
                //     // for c in paths {
                //     //     print!("{}", c);
                //     // }
                //     // println!();
                //     return;
                // }
                
                
                queue.push((xx, yy));
                // predecessors.insert((xx, yy), (x, y, d[k].2));
            }
        }
    }
    println!("NO");
}
struct Scanner {
    tokens: std::collections::VecDeque<String>,
}
impl Scanner {
    pub fn new() -> Self {
        use std::io::BufRead;
 
        let stdin = std::io::stdin();
        let mut tokens = std::collections::VecDeque::new();
        for line in stdin.lock().lines() {
            for token in line.unwrap().split_ascii_whitespace() {
                tokens.push_back(token.to_owned());
            }
        }
        Self { tokens }
    }
 
    pub fn next<T: std::str::FromStr>(&mut self) -> T
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        T::from_str(&self.tokens.pop_front().unwrap()).unwrap()
    }
}