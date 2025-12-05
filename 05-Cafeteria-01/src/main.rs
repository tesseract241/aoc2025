use std::fs::File;
use std::io::Read;
fn main() {
    let mut file = File::open("input").unwrap();
    let mut r = String::new();
    file.read_to_string(&mut r).unwrap();
    let mut output = 0;
    let raw : Vec<&str> = r.split("\n\n").collect();
    let mut ranges = Vec::<Vec::<u64>>::with_capacity(raw[0].len());
    for line in raw[0].lines() {
        let bounds = line.split('-');
        let v : Vec::<u64> = bounds.map(|x| x.parse::<u64>().unwrap()).collect();
        ranges.push(v);
    }
    let mut ids = Vec::<u64>::with_capacity(raw[1].len());
    for line in raw[1].lines() {
        ids.push(line.parse::<u64>().unwrap());
    }
    println!("{ranges:?}");
    println!("{ids:?}");
    for id in ids {
        for range in &ranges {
            if id >= range[0] && id <= range[1] {
                output+=1; 
                println!("{id} is within {} and {}", range[0], range[1]);
                break
            }
        }
    }
    println!("{}", output);
}
