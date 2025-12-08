use std::fs::File;
use std::io::Read;
fn main() {
    let mut file = File::open("input").unwrap();
    let mut r = String::new();
    file.read_to_string(&mut r).unwrap();
    let mut output = 0;
    let operands = r.lines().count();
    let operations = r.lines().next().unwrap().split_whitespace().count();
    let mut raw = Vec::<Vec::<&str>>::with_capacity(operations);
    for _ in 0..operations {
        raw.push(Vec::<&str>::with_capacity(operands));
    }
    for line in r.lines() {
        let v = line.split_whitespace();
        for (i, e) in v.enumerate() {
            raw[i].push(e);
        }
    }
    //println!("{:?}", raw);
    for e in raw {
        if e[e.len()-1] == "*" {
            output += e.iter().take(e.len()-1).map(|x| x.parse::<u64>().unwrap()).product::<u64>();
        } else {
            output += e.iter().take(e.len()-1).map(|x| x.parse::<u64>().unwrap()).sum::<u64>();
        }
    }
    println!("{}", output);
}
