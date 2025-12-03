use std::fs::File;
use std::io::Read;
fn main() {
    let mut file = File::open("input").unwrap();
    let mut r = String::new();
    file.read_to_string(&mut r).unwrap();
    let banks : Vec<&str> = r.trim().split_terminator('\n').collect();
    let mut output = 0u64;
    for bank in banks {
        let mut bank : Vec<u32> = bank.chars().map(|x| x.to_digit(10).unwrap()).collect();
        let mut index : usize;
        let mut value : u32; 
        for iteration in 0u32..12u32 {
            value = 0;
            index = 0;
            //println!("bank : {bank:?}");
            for (i, v) in bank.iter().enumerate().take(bank.len()-(11 - iteration) as usize) {
                if *v > value { value = *v; index = i;}
            }
            //println!("value : {value} index : {index}");
            output+=10_u64.pow(11-iteration)*(value as u64);
            //println!("output : {output}");
            bank  = bank.clone().into_iter().skip(index+1).collect();
        }
        //println!("{value}");
    }

    println!("{}", output);



}
