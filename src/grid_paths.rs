fn main() {
    use std::io::stdin;
    let mut s = String::new();
    stdin().read_line(&mut s);
    let path: Vec<char> = s.chars().collect();
    let mut grid: [[bool; 7]; 7] = [[false; 7]; 7];
    println!("{}", search(&path, &mut grid, 0, 0, 0));
}

fn search(path: &Vec<char>, grid: &mut [[bool; 7]; 7], x: i8, y: i8, idx: i8) -> i32 {
    if x == 0 && y == 6 {
        if idx == 48 {
            return 1;
        }
        return 0;
    }
    if idx == 48 {
        return 1;
    }
    if ((x + 1) >= 7 || grid[(x + 1) as usize][y as usize])
        && ((x - 1) < 0 || grid[(x - 1) as usize][y as usize])
        && (0 <= (y - 1) && (y - 1) < 7 && !grid[x as usize][(y - 1) as usize])
        && (0 <= (y + 1) && (y + 1) < 7 && !grid[x as usize][(y + 1) as usize])
    {
        return 0;
    }

    if ((y + 1) >= 7 || grid[x as usize][(y + 1) as usize])
        && ((y - 1) < 0 || grid[x as usize][(y - 1) as usize])
        && (0 <= (x - 1) && (x - 1) < 7 && !grid[(x - 1) as usize][y as usize])
        && (0 <= (x + 1) && (x + 1) < 7 && !grid[(x + 1) as usize][y as usize])
    {
        return 0;
    }
    let d: [char; 4] = ['D', 'U', 'L', 'R'];
    let dxy = [(0, 1), (0, -1), (-1, 0), (1, 0)];
    let mut ans: i32 = 0;
    for i in 0..d.len() {
        let (dx, dy) = dxy[i];
        if path[idx as usize] == d[i] || path[idx as usize] == '?' {
            let (nx, ny) = (x + dx, y + dy);
            if 0 <= nx && nx < 7 && 0 <= ny && ny < 7 && !grid[nx as usize][ny as usize] {
                grid[x as usize][y as usize] = true;
                ans += search(path, grid, nx, ny, idx + 1);
                grid[x as usize][y as usize] = false;
            }
        }
    }
    return ans;
}
