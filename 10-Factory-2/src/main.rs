use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::cmp::min;

fn ew_sub(lhs : &Vec<u64>, rhs : &Vec<u64>) -> Option<Vec<u64>> {
    //println!("SUBbing {lhs:?} and {rhs:?}");
    assert_eq!(lhs.len(), rhs.len());
    let mut output = Vec::<u64>::with_capacity(lhs.len());
    for i in 0..lhs.len() {
        if lhs[i] < rhs[i] {
            return None;
        }
        output.push(lhs[i] - rhs[i]);        
    }
    return Some(output);
}

fn ew_xor(lhs: &Vec<u64>, rhs : &Vec<u64>) -> Vec<u64> {
    //println!("XORring {lhs:?} and {rhs:?}");
    assert_eq!(lhs.len(), rhs.len());
    let mut output = Vec::<u64>::with_capacity(lhs.len());
    for i in 0..lhs.len() {
        output.push(((lhs[i]>0) ^ (rhs[i]>0)) as u64);        
    }
    return output;
}

fn ew_product(lhs: &Vec<u64>, rhs : &Vec<u64>) -> Vec<u64> {
    //println!("MULtiplying {lhs:?} and {rhs:?}");
    assert_eq!(lhs.len(), rhs.len());
    let mut output = Vec::<u64>::with_capacity(lhs.len());
    for i in 0..lhs.len() {
        output.push(lhs[i] * rhs[i]);        
    }
    return output;
}

fn vec_dep(lhs : &Vec<u64>, rhs : &Vec<u64>) -> bool {
    assert_eq!(lhs.len(), rhs.len());
    let l = lhs.iter().find(|&&x| x!=0).unwrap();
    let r = rhs.iter().find(|&&x| x!=0).unwrap();
    for i in 0..lhs.len() {
        if r * lhs[i] != l * rhs[i] {
            return false;
        }
    }
    return true;
}

fn dot_product(lhs : &Vec<u64>, rhs : &Vec<u64>) -> u64{
    assert_eq!(lhs.len(), rhs.len());
    let mut output = 0u64;
    for i in 0..lhs.len() {
        output += lhs[i]*rhs[i];
    }
    return output;
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut r = String::new();
    file.read_to_string(&mut r).unwrap();
    let mut output = 0u64;
    let mut buttons = VecDeque::<Vec::<Vec<u64>>>::with_capacity(r.lines().count());
    let mut joltages = Vec::<Vec::<u64>>::with_capacity(r.lines().count());
    for line in r.lines() {
        // println!("{line}");
        let size = line.find(']').unwrap() - line.find('[').unwrap()-1;
        // println!("We have {size} lights.");
        let other_pos_left = line.find('{').unwrap();
        let other_pos_right = line.find('}').unwrap();
        let mut button_line = Vec::<Vec::<u64>>::new();
        for button_text in line[line.find(']').unwrap()+1..other_pos_left].split_whitespace() {
            let mut button = vec!(0u64; size);
            for i in button_text[1..button_text.len()-1].split(',').map(|x| x.parse::<usize>().unwrap()) {
                button[i] = 1;
            }
            // println!("Button: {button:?}");
            button_line.push(button);
        }
        // println!("Buttons: {button_line:?}");
        buttons.push_back(button_line);
        let mut joltage = Vec::<u64>::with_capacity(size);
        for x in line[other_pos_left+1..other_pos_right].split(',').map(|x| x.parse::<u64>().unwrap()) {
            joltage.push(x);
        }
        // println!("Joltage: {joltage:?}");
        joltages.push(joltage);
        // println!();
    }

    println!("{} problems to solve", joltages.len());
    let mut solved = 0;

    for joltage in joltages.iter() {
        //These are buttons that energise lights that are not needed, so they cannot be part of the
        //solution
        // println!("Joltages: {joltage:?}");
        let mut button_line = buttons.pop_front().unwrap();
        println!("Problem {solved}: {} lights, {} total joltage required, {} buttons", joltage.len(), joltage.iter().sum::<u64>(), button_line.len());
        // println!("Buttons:                                 {button_line:?}");
        button_line = button_line.into_iter().filter(|x| (ew_product(&x, &ew_xor(&x, joltage))).iter().all(|&y| y==0)).collect::<Vec::<Vec::<u64>>>();
        //Each button can only give one jolt to a light, so the only way a solution can require less
        //is for a button to require a jolt to as many of the needed lights as possible: this should
        //speed up the search.
        // println!("Buttons after filtering:                 {button_line:?}");
        button_line.sort_by_key(|x| -(dot_product(x, joltage) as i64));
        // println!("Buttons sorted by similarity to joltage: {button_line:?}");
        let button_line_sorted = button_line.to_vec();
        let mut indices = Vec::<usize>::new();
        let mut minimum_buttons = u64::MAX;
        'outer: loop{
            let mut remaining_joltages = joltage.clone();
            'inner: loop{
                for i in indices.iter() {
                    match ew_sub(&remaining_joltages, &button_line_sorted[*i]) {
                        Some(x) => remaining_joltages = x,
                        None => break 'inner,
                    }
                }
                for (i, button) in button_line_sorted.iter().enumerate() {
                    if vec_dep(button, &remaining_joltages) {
                        let mut solution = indices.clone();
                        let button_multiplier = remaining_joltages.iter().sum::<u64>() / button.iter().sum::<u64>(); 
                        for _ in 0..button_multiplier{
                            solution.push(i as usize);
                        }
                        //solution.sort();
                        //println!("Found solution using buttons {solution:?} ({} presses, remaining_joltages were {remaining_joltages:?})", solution.len());
                        if indices.len() == 0 {
                            minimum_buttons = button_multiplier;
                            break 'outer;
                        }
                        minimum_buttons = min(minimum_buttons, indices.len() as u64 + button_multiplier);
                        break;
                    }
                }
                //If this assert ever fails it means we failed to find a solution
                assert!((indices.len() as u64) < joltage.iter().sum::<u64>());
                break 'inner;
            }
            let mut indices_iter = indices.iter_mut().rev().peekable();
            loop {
                if let Some(index) = indices_iter.next() {
                    if let Some(&&mut next_index) = indices_iter.peek() {
                        if *index < next_index {
                            *index += 1;
                            break;
                        } else {
                            *index = 0;
                        }
                    } else {
                        if *index < button_line_sorted.len() - 1 {
                                *index +=1;
                            } else {
                                *index = 0;
                        }
                        break;
                    }
                } else {
                    if indices.len() != 0 {
                        panic!("Faulty logic in the code for increasing indices!");
                    } else {
                        break;
                    }
                }
            }

            //We have exhausted all sequences of this length, we have to go longer
            if indices.len() != 0 {
                if indices.len() >= minimum_buttons as usize {
                    break 'outer;
                }
                if indices.iter().all(|&x| x==0) {
                    if indices.len() as u64 == joltage.iter().sum::<u64>() {
                        break 'outer;
                    }
                    indices.push(1);
                    println!("Exhausted all sequences of length {}, so far the best solution is {minimum_buttons} presses", indices.len());
                } else {
                    //println!("Going on to check with starting buttons {indices:?}");
                }
            } else {
                indices.push(0);
                println!("Exhausted all sequences of length 1, going deeper with starting buttons {indices:?}");
            }
        }
        assert_ne!(minimum_buttons, u64::MAX);
        assert_ne!(minimum_buttons, 0);
        solved+=1;
        println!("Problem {solved} out of {} solved: found a solution with {minimum_buttons} presses\n", joltages.len());
        output += minimum_buttons;
    }


    println!("{output}");
}    

