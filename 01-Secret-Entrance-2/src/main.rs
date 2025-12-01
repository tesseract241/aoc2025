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
        let v = i[1..].parse::<i32>().unwrap();
        if i.starts_with('L') {
            for _ in 0..v {
                if current_value == 1 {password+=1;}
                if current_value == 0 {current_value = 100}
                current_value-=1;
            }
        } else {
            for _ in 0..v {
                if current_value == 99 {password+=1;}
                if current_value == 100 {current_value = 0}
                current_value+=1;
            }

        }
        println!("{} {} {}", current_value, i, password);
    }
    println!("{}", password);
}
