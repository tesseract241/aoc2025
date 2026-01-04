use std::fs::File;
use std::io::Read;

#[derive(Hash, PartialEq, Eq, Clone)]
struct Point{
    x : i64,
    y : i64,
}

fn area(a : &Point, b : &Point) -> i64 {
    ((a.x - b.x).abs() + 1)*((a.y - b.y).abs() + 1)
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut r = String::new();
    file.read_to_string(&mut r).unwrap();
    let mut output = 0i64;
    let mut points = Vec::<Point>::with_capacity(r.lines().count());
    for line in r.lines(){
        let v = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec::<i64>>();
        points.push(Point{x : v[0], y : v[1]});
    }
    for i in 0..points.len()-1 {
        for j in i+1..points.len(){
            // println!("Point ({},{}) and ({},{}) produce area {}", points[i].x, points[i].y, points[j].x, points[j].y, area(&points[i], &points[j]));
        output = std::cmp::max(output, area(&points[i], &points[j]));
        }
    }
    println!("{output}");
    
}
