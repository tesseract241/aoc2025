use std::fs::File;
use std::io::Read;
fn main() {
    let mut file = File::open("input").unwrap();
    let mut r = String::new();
    file.read_to_string(&mut r).unwrap();
    let mut output = 0;
    let columns = r.lines().next().unwrap().len();
    let mut splitters = Vec::<Vec::<(usize, usize)>>::with_capacity(columns);
    let mut s = (0, 0);
    for _ in 0..columns {
        splitters.push(Vec::<(usize, usize)>::new());
    }
    for (i, line) in r.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '^' { splitters[j].push((i, 1));}
            else if c == 'S' {s.0 = i; s.1 = j;}
        }
    }
    println!("{s:?}");
    println!("{:?}", splitters);
    let mut ray_spawns = Vec::<(usize, usize)>::new();
    ray_spawns.push(s);
    loop {
        let spawn = match ray_spawns.pop() {
            Some(r) => r,
            None => break
        };
        match splitters[spawn.1].iter().position(|x| x.0 >= spawn.0){
            Some(x) => {
                if splitters[spawn.1][x].1 == 1 {
                    println!("Found hit ({},{})", splitters[spawn.1][x].0, spawn.1); 
                    splitters[spawn.1][x].1 = 0;
                    ray_spawns.push((splitters[spawn.1][x].0, spawn.1-1));
                    ray_spawns.push((splitters[spawn.1][x].0, spawn.1+1));
                }
            },
            None => ()
        }
    }
    for column in splitters {
        output += column.iter().filter(|x| x.1 == 0).count();
    }
    println!("{output}");
    
}
