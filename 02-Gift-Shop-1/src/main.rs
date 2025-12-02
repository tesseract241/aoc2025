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
            if snumber.len()%2==0 && snumber[0..snumber.len()/2] == snumber[snumber.len()/2..] {
                println!("{number}");
                output+=number;
            }
            /*for n in 2..=snumber.len()/2 {
                if snumber.len()%n==0 {
                    let mut silly = true;
                    for i in 0..(snumber.len()/n)-1 {
                        if snumber[i*n..(i+1)*n] != snumber[(i+1)*n..(i+2)*n] {
                            silly = false;
                            break
                        }
                    }
                    if silly {
                        println!("{number} {n}");
                        output+=number;
                        break
                    }
                }
            }*/
        }
    }

    println!("{}", output);



}
