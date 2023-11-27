use std::io;

fn main() {
    println!("Point A = (0;0)\nEnter point B:");
    let (a, b) = input_values();
    println!("Point B = ({};{})", a, b);
    let (area, point_c) = find_smallest_area(a, b);
    let (x, y) = point_c;
    println!("For B({a};{b}) smallest area is {area:0.01} with C({x};{y})");
}

fn find_smallest_area(a: i64, b: i64) -> (f64, (i64, i64)) {
    let mut min_area = i64::MAX;
    let mut point_c = (0,0);
    let ratio: f64 = (b as f64)/(a as f64);
    
    let end = a.abs();
    let step = end/a;

    let mut x: i64 = 0;
    while x.abs() < end {
        let y = ((x as f64 * ratio).floor()) as i64 + 1;
        let cur_area = (a * y - x * b).abs();

        if min_area > cur_area {
            point_c = (x,y);
            min_area = cur_area;
        }
        x += step;
    }
    (min_area as f64 / 2.0, point_c)
}

// fn input_values() -> (i64, i64) {
//     let mut input_line = String::new();
//     let _ = io::stdin().read_line(&mut input_line);
//     let values: Vec<i64> = input_line
//                                      .split_whitespace()
//                                      .map(|q| q.parse().unwrap())
//                                      .collect();
//     (values[0], values[1])
// }

fn input_values() -> (i64, i64) {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut iter = input_line
                                .trim()
                                .split_whitespace()
                                .map(|s| s.parse::<i64>().expect("Invalid input"));
    (iter.next().unwrap(), iter.next().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_smallest_area() {
        assert_eq!(find_smallest_area(3, -5), (0.5, (2, -3)));
        assert_eq!(find_smallest_area(19, 7), (0.5, (8, 3)));
        assert_eq!(find_smallest_area(40, 30), (5.0, (1, 1)));
        assert_eq!(find_smallest_area(-100, 100), (50.0, (0, 1)));
    }
}