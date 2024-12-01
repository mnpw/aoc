use aoc23::parse_input;

fn main() {
    let input = parse_input("01.a.input");
    part1(input);
    let input = parse_input("01.b.input");
    part2(input);
}

fn part1(input: String) {
    let mut sum = 0;

    for line in input.lines() {
        let numbers = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();

        let first_digit = numbers.first().expect("No digit in line");
        let last_digit = numbers.last().expect("No digit in line");

        let number = first_digit * 10 + last_digit;
        // dbg!(number);
        sum += number;
    }
    println!("part 1 - {sum}")
}

fn part2(input: String) {
    let mut sum = 0;

    let mut modified_input = String::new();

    for i in 0..input.len() {
        let t = string_to_num(&input[i..]);
        modified_input.push_str(t);
    }

    for line in modified_input.lines() {
        let numbers = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();

        let first_digit = numbers.first().expect("No digit in line");
        let last_digit = numbers.last().expect("No digit in line");

        let number = first_digit * 10 + last_digit;
        // dbg!(number);
        sum += number;
    }
    println!("part 2 - {sum}")
}

fn string_to_num(input: &str) -> &str {
    if input.starts_with("zero") {
        "0"
    } else if input.starts_with("one") {
        "1"
    } else if input.starts_with("two") {
        "2"
    } else if input.starts_with("three") {
        "3"
    } else if input.starts_with("four") {
        "4"
    } else if input.starts_with("five") {
        "5"
    } else if input.starts_with("six") {
        "6"
    } else if input.starts_with("seven") {
        "7"
    } else if input.starts_with("eight") {
        "8"
    } else if input.starts_with("nine") {
        "9"
    } else {
        // return the first character
        &input[0..1]
    }
}
