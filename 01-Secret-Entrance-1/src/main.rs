use std::fs::File;
use std::io::Read;
fn main() {
    let mut file = File::open("input").unwrap();
    let mut i = String::new();
    file.read_to_string(&mut i).unwrap();

    let instructions : Vec<&str> = i.split_terminator('\n').collect();
    let mut current_value = 50i32;
    let mut password = 0;
    for i in instructions {
        if i.starts_with('L') {
            current_value = (current_value - i[1..].parse::<i32>().unwrap()).rem_euclid(100); 
        } else {
            current_value = (current_value + i[1..].parse::<i32>().unwrap()).rem_euclid(100); 
        }
        if current_value == 0 {password+=1;}
    }
    println!("{}", password);



}
