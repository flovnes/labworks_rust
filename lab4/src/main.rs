use regex::Regex;
use std::io::{self};
use std::time::Instant;

fn main() {
    println!("1 Створення рядка. Введіть n: ");
    let num: u32 = read_line().trim().parse().unwrap();
    forward(num);
    backwards(num);
    println!("9 Форматування рядка. Рядок: ");
    format_string(read_line());
    println!("15 Перевірка формату. Перший рядок; Другий рядок: ");
    println!("{}", compare_spaces(read_line(), read_line()));
    println!("16 Анаграми. Перший рядок; Другий рядок: ");
    println!("{}", anagram(read_line(), read_line()));
    println!("17 Перевірка дужок. Рядок: ");
    println!("{}", validate_parentheses(read_line()));
    println!("18. Пошук. Введіть рядок та шаблон: ");
    for word in find_words(read_line().trim(), read_line().trim()) {
        print!("'{}' ", word)
    }
    println!("\n18(regex). Введіть рядок та шаблон: ");
    for match_value in find_words_regex(read_line().trim(), read_line().trim()) {
        print!("'{}' ", match_value)
    }
}

fn read_line() -> String {
    let mut input_line = String::new();
    let _ = io::stdin().read_line(&mut input_line);
    input_line
}

fn forward(num: u32) {
    let mut result = String::new();

    let start = Instant::now();
    for i in 1..=num {
        result.push_str(&format!("{} ", i));
    }
    let elapsed = start.elapsed();

    println!(
        "Completed \"forward()\" for N = {} in {}ms",
        num,
        elapsed.as_millis()
    );
}

fn backwards(num: u32) {
    let mut result = String::new();

    let start = Instant::now();
    for i in (1..=num).rev() {
        result.push_str(&format!("{} ", i));
    }
    let elapsed = start.elapsed();

    // println!("{}", result);
    println!(
        "Completed \"backwards()\" for N = {} in {}ms",
        num,
        elapsed.as_millis()
    );
}

fn format_string(input: String) {
    let result_letters: String = input
        .chars()
        .filter(|&char| char.is_ascii_alphabetic())
        .collect();
    let result_numbers: String = input
        .chars()
        .filter(|&char| char.is_ascii_digit())
        .collect();
    println!("{}{}", result_letters, result_numbers);
}

fn compare_spaces(input: String, example: String) -> &'static str {
    for (letter_example, letter_input) in example.chars().zip(input.chars()) {
        if letter_example.is_whitespace() && letter_example != letter_input {
            return "NO";
        }
    }
    "YES"
}

fn anagram(mut input: String, mut example: String) -> &'static str {
    input = input.to_lowercase();
    example = example.to_lowercase();
    let mut input_letters: String = input
        .chars()
        .filter(|&char| char.is_ascii_alphabetic())
        .collect();
    let mut example_letters: String = example
        .chars()
        .filter(|&char| char.is_ascii_alphabetic())
        .collect();
    input_letters = insertsort(&input_letters);
    example_letters = insertsort(&example_letters);
    if input_letters.eq(&example_letters) {
        "YES"
    } else {
        println!("{:?} != {:?}", input_letters, example_letters);
        "NO"
    }
}

fn insertsort(s: &str) -> String {
    let mut str = String::with_capacity(s.len());
    let mut str_iter = s.chars().into_iter();
    match str_iter.next() {
        Some(chr) => str.push(chr),
        None => return str,
    }
    while let Some(chr) = str_iter.next() {
        let mut found = None;
        for (index, value) in str.chars().enumerate() {
            if value >= chr {
                found = Some(index);
                break;
            }
        }
        if let Some(index) = found {
            str.insert(index, chr)
        } else {
            str.push(chr)
        }
    }
    str
}

fn validate_parentheses(input: String) -> &'static str {
    let mut count: i32 = 0;
    for symbol in input.chars() {
        if symbol == '(' {
            count += 1
        }
        if symbol == ')' {
            count -= 1
        }
        if count < 0 {
            return "NO";
        }
    }
    if count != 0 {
        return "NO";
    }
    "YES"
}

fn find_words(input: &str, pattern: &str) -> Vec<String> {
    let line = Regex::new(r#"[^a-zA-Z()<>/+:-;,. ]"#)
        .unwrap()
        .replace_all(input, "");

    let delimiters: Vec<char> = vec![
        ' ', '(', ')', '[', ']', '<', '>', '+', '-', ',', '.', '/', ':', ';', '\t',
    ];

    let words: Vec<&str> = line
        .trim()
        .split(&delimiters[..])
        .filter(|&s| !s.is_empty())
        .collect();
    let mut search = Vec::new();

    println!("Found {} words to match: {}", words.len(), words.join(", "));

    for (index, word) in words.iter().enumerate() {
        println!("[{}/{}] Word: {}", index + 1, words.len(), word);
        let pattern_index = 0;
        let word_index = 0;

        if check_word(
            word.chars().collect(),
            pattern.chars().collect(),
            word_index,
            pattern_index,
        ) {
            println!("+ '{}' added.", word);
            search.push(word.to_string());
        }
    }

    match search.len() {
        0 => println!("No matches found."),
        _ => println!("\nFound {} matches in:\n  \"{}\"", search.len(), input),
    }

    search
}

fn check_word(
    word: Vec<char>,
    pattern: Vec<char>,
    mut word_index: usize,
    mut pattern_index: usize, // mut temp: String,
) -> bool {
    while word_index < word.len() {
        println!(
            "    ({}/{}) Expected: {}, got: {}",
            pattern_index + 1,
            pattern.len(),
            &pattern[pattern_index],
            &word[word_index]
        );

        match pattern[pattern_index] {
            '?' => {
                println!("      :symbol skipped by '?'");
                pattern_index += 1;
                word_index += 1;
            }
            '*' => {
                println!("      :inside '*'");
                // if star is the last pattern character
                if pattern_index + 1 == pattern.len() {
                    println!("      :'*' is last character");
                    return true;
                } else {
                    if let Some(index_of_last_next) = find_last(&word, &pattern[pattern_index + 1])
                    {
                        println!(
                            "      :jumped to the last occurance of {}",
                            word[index_of_last_next]
                        );
                        word_index = index_of_last_next;
                        pattern_index += 1;
                    } else {
                        println!(
                            "      :no matches for {} found. skipping word",
                            pattern[pattern_index + 1]
                        );
                        break;
                    }
                }
            }
            _ => {
                if &pattern[pattern_index] == &word[word_index] {
                    pattern_index += 1;
                    word_index += 1;
                } else {
                    println!("- Unmatched symbol. Skipping word.");
                    return false;
                }
            }
        }

        if pattern_index > pattern.len() - 1 {
            return true;
        }
    }
    return false;
}

fn find_last(word: &Vec<char>, letter: &char) -> Option<usize> {
    println!("      :'find_last' -> looking for the last occurance of {letter}");
    let mut last: Option<usize> = None;
    for (letter_index, current_letter) in word.iter().enumerate() {
        println!("      'find_last': comparing {current_letter} to {letter}");
        if current_letter == letter {
            println!("      'find_last': last index of {letter} is {letter_index}");
            last = Some(letter_index);
        }
    }
    return last;
}

fn find_words_regex(input: &str, example: &str) -> Vec<String> {
    let search = Regex::new(&example.replace("?", ".").replace("*", "(\\w)*")).unwrap();
    let result: Vec<String> = search
        .find_iter(input)
        .map(|m| m.as_str().to_string())
        .collect();

    println!("\nFound {} matches in:\n  \"{}\"", result.len(), input);
    println!("Matched words: ");
    result
}
