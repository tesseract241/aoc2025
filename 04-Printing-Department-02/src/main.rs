use std::fs::File;
use std::io::Read;
fn main() {
    let mut file = File::open("input").unwrap();
    let mut r = String::new();
    file.read_to_string(&mut r).unwrap();
    let mut lines = r.lines().peekable();
    let line_length = lines.peek().unwrap().len();
    let mut output = 0;
    let mut grid = Vec::<Vec::<u64>>::with_capacity(lines.clone().count());
    grid.push(vec![0u64; line_length+2]);
    for line in lines {
        let mut v : Vec::<u64> = line.chars().map(|x| {if x=='@' {return 1;} else {return 0;}}).collect();
        v.insert(0, 0);
        v.push(0);
        grid.push(v);
        //println!("{line}");
    }
    grid.push(vec![0u64; line_length+2]);
    loop{
        let old_output = output;
        for i in 1..grid.len()-1 {
            for j in 1..line_length+1 {
                if grid[i][j]==1 {
                    let rolls = 
                        grid[i-1][j-1] + 
                        grid[i-1][j  ] + 
                        grid[i-1][j+1] + 
                        grid[i  ][j-1] + 
                        grid[i  ][j+1] + 
                        grid[i+1][j-1] +
                        grid[i+1][j  ] +
                        grid[i+1][j+1];
                    //println!("({i} {j}) rolls: {rolls}");
                    if rolls < 4 {
                        output+=1;
                        grid[i][j] = 0;
                    }
                }
            }
        }
        if old_output==output {break}
    }
    println!("{}", output);
}
