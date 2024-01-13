use std::io::stdin;

fn dfs(x: usize, y: usize, n: i32, m: i32, visited: &mut [[bool; 1001]; 1001]) {
    // println!("dfs {} {}", x, y);
    visited[x][y] = true;
    let d: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    for k in 0..4 {
        let (xx, yy) = ((x as i32) + d[k].0, (y as i32) + d[k].1);
        if xx >= 0 && xx < n && yy >= 0 && yy < m && !visited[xx as usize][yy as usize] {
            dfs(xx as usize, yy as usize, n, m, visited);
        }
    }
}
fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("");
    let mut iter = line.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let m: i32 = iter.next().unwrap().parse().unwrap();
    let mut grid: [[char; 1001]; 1001] = [['#'; 1001]; 1001];
    let mut visited: [[bool; 1001]; 1001] = [[false; 1001]; 1001];
    for i in 0..n as usize {
        line = "".to_string();
        stdin().read_line(&mut line).expect("");
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c;
            visited[i][j] = c == '#';
        }
    }
    let mut ans = 0;
    for i in 0..n as usize {
        for j in 0..m as usize {
            if !visited[i][j] {
                dfs(i, j, n, m, &mut visited);
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
