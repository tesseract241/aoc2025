use std::fs::File;
use std::io::Read;
fn main() {
    let mut file = File::open("input").unwrap();
    let mut r = String::new();
    file.read_to_string(&mut r).unwrap();
    let raw : Vec<&str> = r.split("\n\n").collect();
    let mut output = 0;
    let mut ranges = Vec::<Option::<Vec::<u64>>>::with_capacity(raw[0].len());
    for line in raw[0].lines() {
        let bounds = line.split('-');
        let v : Vec::<u64> = bounds.map(|x| x.parse::<u64>().unwrap()).collect();
        ranges.push(Some(v));
    }
    ranges.sort_by_key(|a| match a { Some(o) => o[0], None => 0});
    //for range in &ranges {
    //    println!("{range:?}");
    //}
    for i in 0..ranges.len()-1 {
        if ranges[i].is_some() {
            for j in i+1..ranges.len() {
                if ranges[i].clone().unwrap()[1] >= ranges[j].clone().unwrap()[0] {
                    ranges[i] = Some(vec!(ranges[i].clone().unwrap()[0], std::cmp::max(ranges[i].clone().unwrap()[1], ranges[j].clone().unwrap()[1])));
                    ranges[j] = None;
                } else {
                    break
                }
            }
        }
    }
    //println!("After compactification:");
    for range in ranges {
    //    println!("{range:?}");
        match range{
            Some(o) => output+= o[1] - o[0] + 1, 
            None => ()
        }
    }
    println!("{output}");
}
