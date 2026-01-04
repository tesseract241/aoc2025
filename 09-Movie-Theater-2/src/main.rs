use std::collections::HashMap;
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
    let mut extremes_x = HashMap::<i64, (i64, i64)>::new();
    let mut extremes_y = HashMap::<i64, (i64, i64)>::new();
    for line in r.lines(){
        // println!("{line}");
        let v = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec::<i64>>();
        points.push(Point{x : v[0], y : v[1]});
        extremes_x.entry(v[1]).and_modify(|e| *e=(std::cmp::min(e.0, v[0]), std::cmp::max(e.1, v[0]))).or_insert((v[0], v[0]));
        extremes_y.entry(v[0]).and_modify(|e| *e=(std::cmp::min(e.0, v[1]), std::cmp::max(e.1, v[1]))).or_insert((v[1], v[1]));
    }
    // for y in &extremes_x {
    //     println!("Extremes for y {} are: ({}, {})", y.0, y.1.0, y.1.1);
    // }
    // for x in &extremes_y {
    //     println!("Extremes for x {} are: ({}, {})", x.0, x.1.0, x.1.1);
    // }
    let mut boundaries_x = HashMap::<i64, (i64, i64)>::new();
    let mut boundaries_y = HashMap::<i64, (i64, i64)>::new();
    for (y, (min_x, max_x)) in &extremes_x {
        for x in *min_x..=*max_x {
            boundaries_y.entry(x).and_modify(|e| *e = (std::cmp::min(e.0, *y), std::cmp::max(e.1, *y))).or_insert((*y, *y));
        }
    }
    for (x, (min_y, max_y)) in &extremes_y {
        for y in *min_y..=*max_y {
            boundaries_x.entry(y).and_modify(|e| *e = (std::cmp::min(e.0, *x), std::cmp::max(e.1, *x))).or_insert((*x, *x));
        }
    }
    // for y in &boundaries_x {
    //     println!("Boundaries for y {} are: ({}, {})", y.0, y.1.0, y.1.1);
    // }
    // for x in &boundaries_y {
    //     println!("Boundaries for x {} are: ({}, {})", x.0, x.1.0, x.1.1);
    // }
    for i in 0..points.len()-1 {
        'label: for j in i+1..points.len(){
            if  area(&points[i], &points[j]) > output && //This is the easiest check to calculate
                points[i].x >= boundaries_x[&points[j].y].0 && points[i].x <= boundaries_x[&points[j].y].1 &&
                points[i].y >= boundaries_y[&points[j].x].0 && points[i].y <= boundaries_y[&points[j].x].1 &&
                points[j].x >= boundaries_x[&points[i].y].0 && points[j].x <= boundaries_x[&points[i].y].1 &&
                points[j].y >= boundaries_y[&points[i].x].0 && points[j].y <= boundaries_y[&points[i].x].1 {
                    let (min_x, max_x) = (std::cmp::min(points[i].x, points[j].x), std::cmp::max(points[i].x, points[j].x));
                    let (min_y, max_y) = (std::cmp::min(points[i].y, points[j].y), std::cmp::max(points[i].y, points[j].y));
                    for x in min_x..=max_x {
                        if min_y < boundaries_y[&x].0 || max_y > boundaries_y[&x].1 {
                            continue 'label;
                        }
                    }
                    for y in min_y..=max_y {
                        if min_x < boundaries_x[&y].0 || max_x > boundaries_x[&y].1 {
                            continue 'label;
                        }
                    }
                    // println!("Point ({},{}) and ({},{}) produce area {}", points[i].x, points[i].y, points[j].x, points[j].y, area(&points[i], &points[j]));
                    output = area(&points[i], &points[j]);
            }
        }
    }
    println!("{output}");
    
}
