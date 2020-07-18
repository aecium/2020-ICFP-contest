use std::env;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut min_x:i64 = i64::MAX;
    let mut min_y:i64 = i64::MAX;
    let mut max_x:i64 = i64::MIN;
    let mut max_y:i64 = i64::MIN;

    let mut points: Vec<Point> = Vec::new();

    for i in (1..args.len()-1).step_by(2) {
        let x: i64 = args[i].parse::<i64>().expect("NaN");
        let y: i64 = args[i+1].parse::<i64>().expect("NaN");
        points.push(Point{x: x, y: y});

        if x < min_x { min_x = x; }
        if x > max_x { max_x = x; }
        if y < min_y { min_y = y; }
        if y > max_y { max_y = y; }
    }

    //println!("{} {} {} {}", min_x, min_y, max_x, max_y);
    //println!("{:?}", points);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let mut c = '.';
            for point in &points {
                if point.x==x && point.y==y {
                    c = '#';
                }
            }
            print!("{}", c);
        }
        println!();
    }
}