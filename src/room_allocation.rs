use std::{collections::VecDeque, io::stdin};

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("");
    // println!("{}", line);
    let n: i32 = line.trim().parse().unwrap();
    let mut dates: Vec<(i32, i32, i32)> = vec![];
    for i in 0..n {
        line = "".to_string();
        stdin().read_line(&mut line).expect("");
        let mut iter = line.split_whitespace().into_iter();
        // let (a:i32, b:i32, idx:i32) =
        dates.push((iter.next().unwrap().parse::<i32>().unwrap(), -1, i));
        dates.push((iter.next().unwrap().parse::<i32>().unwrap(), 1, i));
    }
    dates.sort();
    let mut free_rooms: VecDeque<u32> = VecDeque::new();
    let mut max_room = 0;
    let mut ans: Vec<u32> = vec![0; n as usize];
    for (d, act, idx) in dates {
        if act == -1 {
            if free_rooms.len() == 0 {
                max_room += 1;
                free_rooms.push_front(max_room);
            }
            let room = free_rooms.pop_front().unwrap();
            ans[idx as usize] = room;
        } else {
            let room = ans[idx as usize];
            free_rooms.push_front(room);
        }
    }
    println!("{}", max_room);
    println!(
        "{}",
        ans.iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
