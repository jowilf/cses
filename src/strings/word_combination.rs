fn idx(v: char) -> usize {
    (v as u8 - b'a') as usize
}

struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    counter: [i32; 26],
    end: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
            counter: [0; 26],
            end: false,
        }
    }

    fn print_tree(&self, spaces: &str) {
        println!("{}{}", spaces, self);
        for i in 0..26 {
            if let Some(child) = &self.children[i] {
                child.print_tree(&format!("{}\t", spaces));
            }
        }
    }
}

impl std::fmt::Display for TrieNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let items: Vec<_> = (0..26)
            .filter_map(|i| {
                if let Some(child) = &self.children[i] {
                    Some((char::from(b'a' + i as u8), self.counter[i], child.end))
                } else {
                    None
                }
            })
            .collect();

        write!(f, "{:?}", items)
    }
}

struct Trie {
    root: Box<TrieNode>,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: Box::new(TrieNode::new()),
        }
    }

    fn insert(&mut self, text: &str) {
        let mut curr_node = &mut self.root;
        for letter in text.chars() {
            let index = idx(letter);
            if curr_node.children[index].is_none() {
                curr_node.children[index] = Some(Box::new(TrieNode::new()));
            }
            curr_node.counter[index] += 1;
            curr_node = curr_node.children[index].as_mut().unwrap();
        }
        curr_node.end = true;
    }

    fn count(&self, text: &str, start: usize) -> i32 {
        let mut curr_node = &self.root;
        let mut ans = 0;
        for (i, c) in text.chars().enumerate().skip(start) {
            let index = idx(c);
            if curr_node.end {
                ans += self.count(text, i);
            }
            if curr_node.children[index].is_none() {
                return ans;
            }
            curr_node = curr_node.children[index].as_ref().unwrap();
        }
        if curr_node.end {
            ans += 1;
        }
        ans
    }
}

fn main() {
    let mut text = String::new();
    std::io::stdin().read_line(&mut text).expect("Failed to read input");
    let text = text.trim().to_string();

    let mut input_k = String::new();
    std::io::stdin().read_line(&mut input_k).expect("Failed to read input");
    let k: i32 = input_k.trim().parse().expect("Invalid input");

    let mut root = Trie::new();

    for _ in 0..k {
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).expect("Failed to read input");
        root.insert(word.trim());
    }

    // root.print_tree();

    println!("{}", root.count(&text, 0));
}
