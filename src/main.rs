use std::{fmt::format, vec};

fn main() {
    let litros: i32 = litres(1787.);

    println!("{}", litros)
}

/*
Nathan loves cycling.

Because Nathan knows it is important to stay hydrated, he drinks 0.5 litres of water per hour of cycling.

You get given the time in hours and you need to return the number of litres Nathan will drink, rounded to the smallest value.

For example:

time = 2 ----> litres = 1

time = 6.7---> litres = 3

time = 11.8--> litres = 5
 */

fn litres(time: f64) -> i32 {
    // let number_i32 : i32 = time.trunc() as i32;

    let result: f64 = 0.5 * time;

    println!("{}", result);

    let return_result = result.trunc() as i32;

    return_result
}

/**
Write Number in Expanded Form
You will be given a number and you will need to return it as a string in Expanded Form. For example:
expanded_form(12);    // Should return "10 + 2"
expanded_form(42);    // Should return "40 + 2"
expanded_form(70304); // Should return "70000 + 300 + 4"
 */

fn expanded_form(n: u64) -> String {
    let mut final_string: String = String::from("");

    final_string.push_str("");

    final_string
}

fn set_alarm(employed: bool, vacation: bool) -> bool {
    if employed && !vacation {
        return true;
    }
    false
}

/*
In this simple exercise, you will build a program that takes a value, integer , and returns a list of its multiples up to another value, limit . If limit is a multiple of integer, it should be included as well. There will only ever be positive integers passed into the function, not consisting of 0. The limit will always be higher than the base.

For example, if the parameters passed are (2, 6), the function should return [2, 4, 6] as 2, 4, and 6 are the multiples of 2 up to 6.


*/
fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();

    if n + n > limit {
        v.push(n);
        return v;
    }

    let mut counter = 0;

    while counter < limit {
        if counter + n > limit {
            return v;
        }
        counter += n;
        v.push(counter);
    }
    v
}

// Implement a function which convert the given boolean value into its string representation.
fn boolean_to_string(b: bool) -> String {
    let mut final_string: String;
    if b {
        return true.to_string();
    }
    false.to_string()
}

fn switch_it_up(n: usize) -> &'static str {
    if n == 1 {
        return "One";
    }
    if n == 2 {
        return "Two";
    }
    if n == 3 {
        return "Three";
    }
    if n == 4 {
        return "Four";
    }
    if n == 5 {
        return "Five";
    }
    if n == 6 {
        return "Six";
    }
    if n == 7 {
        return "Seven";
    }
    if n == 8 {
        return "Eight";
    }
    if n == 9 {
        return "Nine";
    }
    if n == 10 {
        return "Ten";
    }
    "Zero"
}

fn merge_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut final_vec: Vec<i32> = Vec::new();

    final_vec.extend(arr1);

    final_vec.extend(arr2);

    final_vec.sort();

    final_vec.dedup();

    final_vec
}

fn reverse_words(str: &str) -> String {
    let mut final_vector: Vec<String> = Vec::new();
    let each_words: Vec<&str> = str.split(' ').collect();
    let mut final_string: String = String::from("");
    for i in each_words {
        final_vector.push(i.chars().rev().collect())
    }

    let len = final_vector.len();
    let mut counter = 0;
    for j in final_vector {
        counter = counter + 1;
        final_string.push_str(j.as_str());
        if counter < len {
            final_string.push(' ')
        }
    }
    final_string
}

fn bool_to_word(value: bool) -> &'static str {
    if !value {
        return "No";
    }

    "Yes"
}

fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;
    let vocals = ['a', 'e', 'i', 'o', 'u'];
    let vec_string: Vec<char> = string.chars().collect();
    for i in vocals {
        for &j in &vec_string {
            if i == j {
                vowels_count = vowels_count + 1
            }
        }
    }
    vowels_count
}

fn min_max(lst: &[i32]) -> (i32, i32) {
    let mut new_vec = lst.to_vec();

    new_vec.sort();

    let last_number: &i32 = new_vec.last().unwrap();

    let final_last_number: i32 = *last_number as i32;

    if lst.len() == 1 {
        return (lst[0], lst[0]);
    }

    (new_vec[0], final_last_number)
}
fn row_sum_odd_numbers(n: i64) -> i64 {
    let mut counter = 1;
    let mut result: i64 = 0;

    if n == 1 {
        return counter;
    }

    let mut iterator = 0;

    for i in 2..n {
        while iterator < i {
            counter = counter + 2;
            iterator = iterator + 1;
        }
    }

    result
}

fn make_upper_case(s: &str) -> String {
    let new_string: String = String::from(s);

    new_string.to_uppercase()
}

// done
fn find_short(s: &str) -> u32 {
    let each_word: Vec<&str> = s.split(' ').collect();

    let mut counter_vec: Vec<u32> = Vec::new();

    for i in each_word {
        counter_vec.push(i.len() as u32)
    }
    counter_vec.sort();

    counter_vec[0]
}

fn zero_fuel(distance_to_pump: u32, miles_per_galon: u32, gallons: u32) -> bool {
    if miles_per_galon * gallons >= distance_to_pump {
        return true;
    }
    false
}

fn is_square(n: i64) -> bool {
    if n < 0 {
        return false;
    }
    if n == 0 {
        return true;
    }

    for i in 0..n {
        if i * i == n {
            return true;
        }
    }
    false
}

// done
fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    if numbers.len() == 0 {
        return numbers.to_vec();
    }

    let mut new_numbers = numbers.to_vec();

    let mut tuple_numbers: Vec<(u32, usize)> = Vec::new();

    for i in 0..new_numbers.len() {
        tuple_numbers.push((new_numbers[i], i))
    }

    tuple_numbers.sort();

    new_numbers.remove(tuple_numbers[0].1);

    new_numbers
}

// done
fn fake_bin(s: &str) -> String {
    let mut new_string = String::from("");

    let char_vec: Vec<char> = s.chars().collect();

    for i in char_vec {
        let num: u8 = match i.to_digit(10) {
            Some(digit) => digit as u8,
            None => 0,
        };
        if num < 5 {
            new_string.push('0')
        } else {
            new_string.push('1')
        }
    }

    new_string
}

fn odd_or_even(numbers: Vec<i32>) -> String {
    let mut num: i32 = 0;

    for i in numbers {
        num += i;
    }

    if num % 2 == 0 {
        return "even".to_string();
    }
    "odd".to_string()
}

// done
fn solution(word: &str, ending: &str) -> bool {
    let word_ending = word.ends_with(ending);
    word_ending
}

// done
fn add_binary(a: u64, b: u64) -> String {
    let sum = a + b;

    let binary = format!("{:b}", sum);
    return binary;
}


// done
fn xo(string: &'static str) -> bool {
    let char_xo: Vec<char> = string.chars().collect();
    let mut x_count = 0;
    let mut o_count = 0;
    for i in char_xo {
        if i == 'x' || i == 'X' {
            x_count += 1
        } else if i == 'o' || i == 'O' {
            o_count += 1
        }
    }
    if x_count == 0 && o_count == 0 {
        return true;
    }
    if x_count != o_count {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    // fn examples() {
    //     assert_eq!(expanded_form(12), "10 + 2");
    //     assert_eq!(expanded_form(42), "40 + 2");
    //     assert_eq!(expanded_form(70304), "70000 + 300 + 4");
    // }

    #[test]
    fn test_set_alarm() {
        assert_eq!(
            set_alarm(true, true),
            false,
            "Fails when input is true, true"
        );
        assert_eq!(
            set_alarm(false, true),
            false,
            "Fails when input is false, true"
        );
        assert_eq!(
            set_alarm(false, false),
            false,
            "Fails when input is false, false"
        );
        assert_eq!(
            set_alarm(true, false),
            true,
            "Fails when input is true, false"
        );
    }
}
