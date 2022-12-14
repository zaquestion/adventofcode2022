use std::io;
use std::collections::HashSet;

fn main() {
    let mut count: u64 = 0;
    let lines: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();
    for line in lines {
        let sections = line.split(",").collect::<Vec<_>>();
        let s1 = sections[0].split("-").collect::<Vec<_>>();
        let s2 = sections[1].split("-").collect::<Vec<_>>();

        let mut hs1 = HashSet::new();
        for i in  (s1[0].parse::<i32>().unwrap()..s1[1].parse::<i32>().unwrap() + 1) {
            hs1.insert(i);
        }

        let mut hs2 = HashSet::new();
        for i in  (s2[0].parse::<i32>().unwrap()..s2[1].parse::<i32>().unwrap() + 1) {
            hs2.insert(i);
        }

        if !hs1.is_disjoint(&hs2) && !hs2.is_disjoint(&hs1) {
            count += 1;
        }
    }
    println!("{:?}", count);
}
