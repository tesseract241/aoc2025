use std::fs::File;
use std::io::Read;
fn main() {
    let mut file = File::open("input").unwrap();
    let mut r = String::new();
    file.read_to_string(&mut r).unwrap();
    let mut output = 0;
    let lines = r.lines().count();
    let columns = r.lines().next().unwrap().chars().count();
    let operations : Vec::<&str> = r.lines().last().unwrap().split_whitespace().rev().collect();
    //println!("{:?}", operations);
    let mut raw = Vec::<String>::with_capacity(columns);
    for _ in 0..columns {
        raw.push(String::with_capacity(lines-1));
    }
    for line in r.lines().take(lines-1) {
        for (i, e) in line.chars().rev().enumerate() {
            raw[i].push(e);
        }
    }
    let raw : Vec::<&str> = raw.iter().map(|x| x.trim()).collect();
    //println!("{:?}", raw);
    let mut operands = Vec::<Vec::<u64>>::new();
    let mut last_pos = 0;
    while let Some(pos) = raw.iter().skip(last_pos).position(|x| x.is_empty()) {
        operands.push(raw[last_pos..last_pos+pos].iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>());
        last_pos += pos+1;
    }
    operands.push(raw[last_pos..].iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>());
    //println!("{:?}", operands);
    for (i, operation) in operations.iter().enumerate() {
        if *operation=="+" {
            let result = operands[i].iter().sum::<u64>();
            //println!("{result}");
            output+=result;
        } else{
            let result = operands[i].iter().product::<u64>();
            //println!("{result}");
            output+=result;
        }
    }
    println!("{}", output);
}
