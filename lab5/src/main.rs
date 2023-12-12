use rand::Rng;
use std::io;

fn main() {
    let array: Vec<i32>;

    println!("Would you like to manually fill the array? (y/n): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().to_uppercase().as_str() {
        "Y" => {
            println!("How do you want to fill the array? (Default: 2)");
            println!("1: \" 4\n     a\n     b\n     c\n     d \"\n");
            println!("2: \"a b c d\"\n");
            println!("Choose (1/2): ");

            match read_line().parse::<i32>() {
                Ok(1) => {
                    println!("Enter the number of elements:");
                    array = input_array(read_line().parse().unwrap());
                    print_array(&array);
                }
                _ => {
                    array = input_array_inline();
                    print_array(&array);
                }
            }
        }
        _ => {
            println!("Enter the number of elements:");
            array = random_array(read_line().parse().unwrap());
            print_array(&array);
        }
    }
    println!("{}", solution(&array));
}

fn random_array(size: i32) -> Vec<i32> {
    let mut num_gen = rand::thread_rng();
    (0..size).map(|_| num_gen.gen_range(-100..100)).collect()
}

fn input_array(size: i32) -> Vec<i32> {
    println!("Enter {} elements:", size);
    (0..size).map(|_| read_line().parse().unwrap()).collect()
}

fn input_array_inline() -> Vec<i32> {
    println!("Enter elements:");
    read_line()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn print_array(array: &Vec<i32>) {
    println!(
        "Your array: \"{}\"",
        array
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn solution(array: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let max_index = array
        .iter()
        .position(|&x| x == *array.iter().max().unwrap())
        .unwrap();
    let min_index = array
        .iter()
        .position(|&x| x == *array.iter().min().unwrap())
        .unwrap();

    let (min_index, max_index) = if max_index > min_index {
        (min_index, max_index)
    } else {
        (max_index, min_index)
    };

    println!("Min element: {}", array[min_index]);
    println!("Max element: {}", array[max_index]);
    print!("Selected elements: \"");
    for i in min_index..=max_index {
        print!("{} ", array[i]);
        sum += array[i];
    }
    println!("\"");

    sum
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
