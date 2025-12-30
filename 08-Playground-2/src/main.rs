use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone)]
struct Jbox{
    x : i64,
    y : i64,
    z : i64
}

struct JboxCouple{
    distance : u64,
    a : Jbox,
    b : Jbox
}
fn squared_distance(a : &Jbox, b : &Jbox) -> u64 {
    ((a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)) as u64
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut r = String::new();
    file.read_to_string(&mut r).unwrap();
    let mut output = 0i64;
    let mut jboxes = Vec::<Jbox>::with_capacity(r.lines().count());
    let mut circuits = Vec::<HashSet::<Jbox>>::with_capacity(r.lines().count());
    for line in r.lines(){
        let v = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec::<i64>>();
        jboxes.push(Jbox{x : v[0], y : v[1], z : v[2]});
        let mut h = HashSet::new();
        h.insert(Jbox{x : v[0], y : v[1], z : v[2]});
        circuits.push(h);
    }
    let mut couples = Vec::<JboxCouple>::with_capacity(jboxes.len()*(jboxes.len()-1)/2);
    for i in 0..jboxes.len()-1 {
        for j in i+1..jboxes.len(){
            couples.push(JboxCouple { distance: squared_distance(&jboxes[i], &jboxes[j]), a: jboxes[i].clone(), b: jboxes[j].clone() });
        }
    }
    couples.sort_unstable_by_key(|x| x.distance);
    for couple in couples {
        let (mut i, mut j) = (usize::MAX, usize::MAX);
        for (k, h) in circuits.iter().enumerate() {
            if h.contains(&couple.a) { i = k;}
            if h.contains(&couple.b) { j = k;}
            if i!=usize::MAX && j!=usize::MAX {break}
        }
        if i!=j { //If the junction boxes are already part of the same circuit we do nothing
            if circuits[i].len() < circuits[j].len(){
                if i>j {
                    let c = circuits.remove(i);
                    circuits[j].extend(c);
                } else {
                    let c = circuits.remove(i);
                    circuits[j-1].extend(c);
                }
            } else {
                if j > i {
                    let c = circuits.remove(j);
                    circuits[i].extend(c);
                } else {
                    let c = circuits.remove(j);
                    circuits[i-1].extend(c);
                }
            }
        }
        if circuits.len()==1 {
            output = couple.a.x * couple.b.x;
            break;
        }
    }
    println!("{output}");
    
}
