#![crate_name = "fizzbuzz"]
use std::vec::Vec;

/// Print vector of strings, every string in new line
///
/// # Arguments
///
/// * `strvec` - vector of strings to print
///
/// # Examples
///
/// ```
/// use fizzbuzz::strvec_print
/// let mut test_strvec = Vec::with_capacity(10);
/// for i in 0..9 {
///     let num : i32 = i;
///     let s: String = num.to_string();
///     test_strvec.push(s);
/// }
/// strvec_print(&test_strvec);
/// 
///
/// ```
fn strvec_print(strvec: &[String]) {
    for sel_string in strvec {
        println!("{}", sel_string)
    }
}

/// FizzBuzz task: for numbers from 1 to 100
/// print FizzBuzz if number is divided entirely by 3 and 5,
/// print Fizz if number is devided entirely by 3
/// print Buzz if number is deviced entirely by 5
/// else print number
fn main() {
    const COUNT: usize = 100;
    let mut result = Vec::with_capacity(COUNT);
    for i in 1..=COUNT {
        let s = match (i % 3, i % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            (_, _) => format!("{}", i),
        };
        result.push(s);
    }
    strvec_print(&result);
}
