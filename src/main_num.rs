use std::io;

fn main() {
    let num = get_input_i64();
    let mut a = num[0];
    let mut b = num[1];
    let c = num[2];
    if c > 0 {
        b -= 1;
    } else {
        a -= 1;
    }
    if a > b {
        println!("Takahashi");
    } else if b > a {
        println!("Aoki");
    } else if c > 0 {
        println!("Aoki");
    } else {
        println!("Takahashi");
    }
}

fn get_input() -> Vec<String> {
    let mut word_line = String::new();
    io::stdin().read_line(&mut word_line).ok();
    let words: Vec<&str> = word_line.split_whitespace().collect();
    words.iter().map(|word| word.to_string()).collect()
}

fn get_input_i64() -> Vec<i64> {
    let words = get_input();
    words.iter().map(|word| word.parse().unwrap()).collect()
}

