use std::fs::File;
use std::io::Read;
fn main() {
    let mut file = File::open("input").unwrap();
    let mut r = String::new();
    file.read_to_string(&mut r).unwrap();
    let ranges : Vec<&str> = r.trim().split(',').collect();
    let mut output = 0;
    for range in ranges {
        //println!("{range}");
        let extremes : Vec<&str> = range.split('-').collect();
        //println!("{:?}", extremes);
        let lower = extremes[0].parse::<u64>().unwrap();
        let upper = extremes[1].parse::<u64>().unwrap();
        //println!("{lower}-{upper}");
        for number in lower..=upper {
            //println!("{number}");       
            let snumber = format!("{number}");
            for n in 2..=snumber.len() {
                if snumber.len()%n==0 {
                    let mut silly = true;
                    let m = snumber.len()/n;
                    for i in 0..=n-2 {
                        if snumber[i*m..(i+1)*m] != snumber[(i+1)*m..(i+2)*m] {
                            silly = false;
                            break
                        }
                    }
                    if silly {
                        //println!("{number} {n}");
                        output+=number;
                        break
                    }
                }
            }
        }
    }

    println!("{}", output);



}
