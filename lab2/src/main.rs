use std::io;
use std::str::FromStr;
use std::fmt::Debug;

fn main() {
    loop {
        println!("Введіть команду: ");
        println!("Для виконання блоку 1 (Варіант 17) введіть 1.");
        println!("Для виконання блоку 2 (Варіант 29) введіть 2.");
        println!("Для виконання блоку 3 (Варіант 61) введіть 3.");
        println!("Для виходу з програми введіть 0.");
  
        let mut input = String::new();
        let _ = io::stdin()
                           .read_line(&mut input)
                           .expect( "[main] не вдалося прочитати рядок\n" );

        let choice: i32 = input
                               .trim()
                               .parse()
                               .expect( "[main] несмішне число\n" );

        match choice {
            0 => break,
            1 => block_1(),
            2 => block_2(),
            3 => block_3(),
            _ => println!("Команда ``{}`` не розпізнана.", choice),
        }
    }
}

fn block_1() {
    println!("[1 in ] Кількість елементів");
    let n: i32 = input_num(1);
    println!("[1 in ] Введення елементів");
    
    let mut count = 0;
    for _ in 0..n {
        let num: i32 = input_num(1);
        count += num.abs()/num;
    }

    match count {
        x if x > 0 => println!("[1 out] Додатних чисел на {} більше.\n", count),
        x if x < 0 => println!("[1 out] Від'ємних чисел на {} більше.\n", -count),
        _ => println!("[1 out] Додатних і від'ємних чисел порівну.\n")
    }
}

fn block_2() {
    println!("[2 in ] Введення елементів");

    let mut count = 0;
    loop {
        let num: i32 = input_num(2);
        match num {
            0 => break,
            x  => { if x % 2 == 1 { count += 1 } }
        }
    }
    println!("[2 out] Було введено {} непарних чисел.\n", count);
}

fn block_3() {
    println!("[3 in ] Введення X");
    let x: f64 = input_num(3);

    println!("[3 in ] Введення N");
    let n: i32 = input_num(3);

    let negative_if_divisible = (-1_f64).powi(2 - ((n + 2) % 3 / 2));
    let mut current_sin = ((n - 1) as f64 * x + negative_if_divisible * (x * n as f64).sin()).sin();

    if n > 1 {
        for i in (0..n - 2).rev() {
            let negative_if_divisible = (-1_f64).powi(2 - (i % 3 / 2));
            current_sin = (i as f64 * x + negative_if_divisible * current_sin.sin()).sin();
        }
    }

    println!("[3 out] Результат виразу: {}\n", current_sin);
}

fn input_num<T>(block: u8) -> T 
where T: FromStr, T::Err: Debug, 
{
    let mut input = String::new();
    io::stdin()
            .read_line(&mut input)
            .expect( format!("[{block}] не вдалося прочитати рядок\n").as_str() );
    input
         .trim()
         .parse()
         .expect( format!("[{block}] несмішне число\n").as_str() )
}