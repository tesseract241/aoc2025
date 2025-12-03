use std::fs::File;
use std::io::Read;
fn main() {
    let mut file = File::open("input").unwrap();
    let mut r = String::new();
    file.read_to_string(&mut r).unwrap();
    let banks : Vec<&str> = r.trim().split_terminator('\n').collect();
    let mut output = 0;
    for bank in banks {
        println!("{bank}");
        let bank : Vec<u32> = bank.chars().map(|x| x.to_digit(10).unwrap()).collect();
        let (mut index, mut value) = (0usize, bank[0]); 
        for (i, v) in bank.iter().enumerate().take(bank.len()-1) {
            if *v > value { value = *v; index = i;}
        }
        println!("{value}");
        output+=10*value;
        value = 0;
        for v in bank.iter().skip(index+1) {
            if *v > value {value = *v;}
        }
        println!("{value}");
        output+=value;
    }

    println!("{}", output);



}
