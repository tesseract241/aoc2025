use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::ops::Shl;
use std::cmp::min;


fn main() {
    let mut file = File::open("input").unwrap();
    let mut r = String::new();
    file.read_to_string(&mut r).unwrap();
    let mut output = 0u64;
    let mut lights = Vec::<u64>::with_capacity(r.lines().count());
    let mut buttons = VecDeque::<Vec::<u64>>::with_capacity(r.lines().count());
    for line in r.lines() {
        //println!("{line}");
        let pos = line.find(']').unwrap();
        let mut light = 0u64;
        for c in line[1..pos].chars() {
            light = light.shl(1);
            match c{
                '#' => light = light | 1u64.shl(63-pos+2),
                _   => ()
            }
        }
        lights.push(light);
        //println!("Light:  {light:064b}");
        let other_pos = line.find('{').unwrap();
        let mut button_line = Vec::<u64>::new();
        for button in line[pos+1..other_pos].split_whitespace() {
            let mut binary_button = 0u64;
            for b in button[1..button.len()-1].split(',').map(|x| x.parse::<u64>().unwrap()){
                binary_button |= 1u64.shl(63-b);
            }
            //println!("Button: {binary_button:064b}");
            button_line.push(binary_button);
        }
        buttons.push_back(button_line);
        //println!();
    }
    //let mut light_num = -1;
    for light in lights.iter() {
        //light_num+=1;
        let mut minimum_buttons = u64::MAX;
        let button_line = buttons.pop_front().unwrap();
        let mut indices = Vec::<usize>::new();
        'outer: loop {
            let mut light = light.clone();
            for index in indices.iter() {
                light ^= button_line[*index];
            }
            let index = match indices.last() {
                Some(i) => *i+1,
                None    => 0
            };
            for i in index..button_line.len() {
                if light == button_line[i] {
                    minimum_buttons = min(minimum_buttons, (indices.len()+1) as u64);
                    if minimum_buttons == 1 {
                        //println!("Found solution: button {i} turns off all lights alone");
                        break 'outer;
                    }
                    //We are only interested in a shortest valid sequence, so no point looking for
                    //a different one or longer one this is already valid.
                    let mut solution = Vec::from(indices);
                    solution.push(i);
                    //println!("Found solution: buttons {solution:?} turn off all the lights.");
                    break 'outer; 
                }
            }
            for index in indices.iter_mut().rev() {
                if *index == button_line.len()-1 {
                    *index = 0usize;
                } else {
                    *index += 1;
                    break;
                }
            }
            //If this assert ever fails it means we failed to find a solution
            assert!(indices.len() < button_line.len());
            //We have exhausted all sequences of this length, we have to go longer
            if indices.len() != 0 {
                if indices.iter().sum::<usize>() == 0 {
                    for index in 1..indices[indices.len()-1] {
                        indices[index] = indices[index-1] + 1;
                    }
                    indices.push(indices[indices.len()-1]+1);
                    //println!("Exhausted all sequences of length {}, going deeper with starting buttons {indices:?}", indices.len());
                } else {
                    //println!("Going on to check with starting buttons {indices:?}");
                }
            } else {
                indices.push(0);
                //println!("Exhausted all sequences of length 1, going deeper with starting buttons {indices:?}");
            }
        }
        assert_ne!(minimum_buttons, u64::MAX);
        assert_ne!(minimum_buttons,0);
        if minimum_buttons == 1 {
            //println!("For light {light_num} found a solution needing 1 button.");
        } else {
            //println!("For light {light_num} found a solution needing {minimum_buttons} buttons.");
        }
        //println!();
        output+=minimum_buttons;
    }
    println!("{output}");
}    

