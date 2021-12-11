use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input!{
        n: usize,
        m: usize,
        ab: [(usize, usize); m]
    }
    let mut hashmap = HashMap::new();
    for i in 0..m {
        let l = ab[i].0;
        let r = ab[i].1;
        let l_obj = hashmap.entry(l).or_insert(vec![0, 0]);
        let r_obj = hashmap.entry(r).or_insert(vec![0, 0]);
        if l_obj[0] != 0 && l_obj[1] != 0 {
            println!("No");
            std::process::exit(0);
        }
        if r_obj[0] != 0 && r_obj[1] != 0 {
            println!("No");
            std::process::exit(0);
        }
       if l_obj[0] == 0 && r_obj[1] == 0 {
            l_obj[0] = l;
            r_obj[1] = r;
        } else if l_obj[1] == 0 && r_obj[0] == 0 {
            l_obj[1] = l;
            r_obj[0] = r;
        } else {
            println!("No");
            std::process::exit(0);
        }
    }
    println!("Yes");
}
