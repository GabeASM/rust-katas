use std::{fmt::format, vec};

fn main() {
    let litros: i32 = litres(1787.);

    println!("{}", litros);
    println!("holaaa" )

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


// Write a function to convert a name into initials. This kata strictly takes two words with one space in between them.
// The output should be two capital letters with a dot separating them.
// It should look like this:
// Sam Harris => S.H
// patrick feeney => P.F

// done
fn abbrev_name(name: &str) -> String {
    let mut final_string = String::from("");
    let vec_name : Vec<&str>  = name.split(' ').collect(); 
    let first_name_char : Vec<char> = vec_name[0].chars().collect();
    let last_name_char : Vec<char> =  vec_name[1].chars().collect();
    final_string.push(first_name_char[0]);
    final_string.push('.');
    final_string.push(last_name_char[0]);
    final_string.to_uppercase()
}

// Given a set of numbers, return the additive inverse of each. Each positive becomes negatives, and the negatives become positives.

// invert([1,2,3,4,5]) == [-1,-2,-3,-4,-5]
// invert([1,-2,3,-4,5]) == [-1,2,-3,4,-5]
// invert([]) == []
// done
fn invert(values: &[i32]) -> Vec<i32> {
    if values.len() == 0 {
        return values.to_vec()
    }
    let mut new_vector: Vec<i32> = Vec::new();
    for i in values{
        new_vector.push(i*-1)
    }
    new_vector
}


/*
The Western Suburbs Croquet Club has two categories of membership, Senior and Open. They would like your help with an application form that will tell prospective members which category they will be placed.

To be a senior, a member must be at least 55 years old and have a handicap greater than 7. In this croquet club, handicaps range from -2 to +26; the better the player the lower the handicap.

Input
Input will consist of a list of pairs. Each pair contains information for a single potential member. Information consists of an integer for the person's age and an integer for the person's handicap.

Output
Output will consist of a list of string values (in Haskell and C: Open or Senior) stating whether the respective member is to be placed in the senior or open category.

Example
input =  [[18, 20], [45, 2], [61, 12], [37, 6], [21, 21], [78, 9]]
output = ["Open", "Open", "Senior", "Open", "Open", "Senior"]

done
*/
fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    let mut final_vec : Vec<String> = Vec::new();

    for i in data{
        if i.0 >= 55 && i.1 > 7{
            final_vec.push("Senior".to_string())
        }
        else {
            final_vec.push("Open".to_string())
        } 
    }
    final_vec
}


/*
ATM machines allow 4 or 6 digit PIN codes 
and PIN codes cannot contain anything but exactly 4 digits or exactly 6 digits.

If the function is passed a valid PIN string, return true, else return false.

Examples (Input --> Output)
"1234"   -->  true
"12345"  -->  false
"a234"   -->  false

*/
// done
fn validate_pin(pin: &str) -> bool {
    let vec_char : Vec<char> = pin.chars().collect();
    for i in vec_char{
        if !i.is_digit(10){
            return false
        }
    }    
    pin.len() == 4 || pin.len() == 6
}

/*
They are good at taking orders, but they don't know how to capitalize words, or use a space bar!

All the orders they create look something like this:

"milkshakepizzachickenfriescokeburgerpizzasandwichmilkshakepizza"

The kitchen staff are threatening to quit, because of how difficult it is to read the orders.

Their preference is to get the orders as a nice clean string with spaces and capitals like so:

"Burger Fries Chicken Pizza Pizza Pizza Sandwich Milkshake Milkshake Coke"

The kitchen staff expect the items to be in the same order as they appear in the menu.

The menu items are fairly simple, there is no overlap in the names of the items:
1. Burger
2. Fries
3. Chicken
4. Pizza
5. Sandwich
6. Onionrings
7. Milkshake
8. Coke
done
*/
fn get_order( input: String ) -> String {
    let  buger_vec : Vec<&str> = input.matches("burger").collect();
    let  fries_vec : Vec<&str> = input.matches("fries").collect();
    let  chicken_vec : Vec<&str> = input.matches("chicken").collect();
    let  pizza_vec : Vec<&str> = input.matches("pizza").collect();
    let  sandwich_vec : Vec<&str> = input.matches("sandwich").collect();
    let  onionrings_vec : Vec<&str> = input.matches("onionrings").collect();
    let  milkshake_vec : Vec<&str> = input.matches("milkshake").collect();
    let  coke_vec : Vec<&str> = input.matches("coke").collect();


    let mut final_string : String = String::from("");

    fill_order(&mut final_string, buger_vec);
    fill_order(&mut final_string, fries_vec);
    fill_order(&mut final_string, chicken_vec);
    fill_order(&mut final_string, pizza_vec);
    fill_order(&mut final_string, sandwich_vec);
    fill_order(&mut final_string, onionrings_vec);
    fill_order(&mut final_string, milkshake_vec);
    fill_order(&mut final_string, coke_vec);

    final_string.pop();
    final_string
}

fn fill_order(input : &mut String , vector : Vec<&str>){
    for i in vector{
        input.push_str(capitalize_first_letter(i).as_str());
        input.push(' ')
    }
}
fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}


fn string_to_array(s: &str) -> Vec<String> {
    let str_vec : Vec<&str> = s.split(' ').collect();
    let mut final_vec : Vec<String> = Vec::new();

    for i in str_vec{
        final_vec.push(i.to_string())
    }

    final_vec
}


fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    let volume_a = volumes(a);
    let volume_b = volumes(b);
    
    (volume_a - volume_b).abs()
}

fn volumes ( cuboids : &[i32 ; 3]) -> i32 {
    let mut  volume : i32 = 1;
    for i in cuboids {
        volume = volume * i 
    }
    volume
}


fn is_valid_walk(walk: &[char]) -> bool {

    if walk.len() != 10{
        return false
    }

    let mut counter_n = 0; 
    let mut counter_w = 0; 
    let mut counter_s = 0; 
    let mut counter_e = 0;
    
    for i in walk{
        match i {
            'n' => counter_n += 1,
            'w' => counter_w += 1,
            's' => counter_s += 1,
            'e' => counter_e += 1,
            _ => panic!()
        }
    }
    counter_n == counter_s && counter_e == counter_w    
}








/**
 * hay que encontrar el producto max y el min de todos los numeros de un array
 * El array debe tener la suma de los numeros del array debe dar n
 * Cada debe tener k elementos.  
 * find_spec_partition(10, 4, 'max') == [3, 3, 2, 2]
 * find_spec_partition(10, 4, 'min') == [7, 1, 1, 1]
 */

/**
 * los numeros van desde el 1 hasta el n-k+1 debido a que debe haber
 * k slots que deben sumar n
 * En ese caso, si n = 12 y k = 3
 * [10 ,1 , 1]
 * 
 * si n = 5 y k = 2 
 * [4 , 1]
 */

fn find_spec_partition(n: u32, k: u32, com: &str) -> Vec<u32> {
    let mut final_vec : Vec<u32> = Vec::new();

    let mut vec_numbers : Vec<u32> = Vec::new();

    for i in 1..n-k+1{
        
    }
    final_vec
}


