/* 0. -----------------------------------------------------------------------*/
// This will run correctly, to show a passing test.

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/* 1. -----------------------------------------------------------------------*/
// Get the length of a string. 

pub fn get_trimmed_string_length(input: &str) -> usize {

    input.trim().len()

}

/* 2. -----------------------------------------------------------------------*/
// Get the second two elements in an array.

pub fn get_second_two_elements(input: &[usize]) -> &[usize] {

    if input.len() > 2 {
        &input[1..3]
    } else {
        panic!("Array too small.")
    }

}

/* 3. -----------------------------------------------------------------------*/
// Negate the array of i32s (positive to negative, negative to positive)

pub fn negate_the_array(input: &mut [i32]){

    input.iter_mut().for_each(|num| *num *= -1)

}

/* 4. -----------------------------------------------------------------------*/
// Return "fizz" for odd numbers.

pub fn fizz_the_odds(input: i32) -> &'static str {

    match input % 2 {
        0 => "buzz",
        _ => "fizz"
    }
}

/* 5. -----------------------------------------------------------------------*/
// Get the quadrant for a point (on a Cartesian plane).


#[derive(Debug, PartialEq, Eq)]
pub enum Quadrant {
    Origin,
    One,
    Two,
    Three,
    Four,
}

pub fn get_point_quadrant(x:i32, y:i32) -> Quadrant{
    
    let point = (x, y);

    match point {
        (x, y) if x > 0 && y > 0 => Quadrant::One,
        (x, y) if x < 0 && y > 0 => Quadrant::Two,
        (x, y) if x < 0 && y < 0 => Quadrant::Three,
        (x, y) if x > 0 && y < 0 => Quadrant::Four,
        _ => Quadrant::Origin,
    }

}

/* 6. -----------------------------------------------------------------------*/
// Add up all the numbers in a string of text.

pub fn add_numbers_within_text(number_text: &str) -> u32 {

    let mut x = 0;

    for c in number_text.chars()
        .filter(|s| s.is_numeric())
            {
                x += c.to_digit(10).unwrap_or(0);
            }

    x
}

/* 7. -----------------------------------------------------------------------*/
// The option type is one way of managing potential failures. 

pub fn try_divide_returns_option_type(numerator: i32, denominator: i32) -> Option<i32> {

    if denominator != 0 {
        Some(numerator / denominator)
    } else {
        None
    }
}

/* 8. -----------------------------------------------------------------------*/
// The error type is another way to manage potential failures. 

#[derive(Debug, PartialEq, Eq)]
pub enum DivError {
    DivisionByZero,
}

pub fn try_divide_returns_error_type(numerator: i32, denominator: i32) -> Result<i32, DivError> {

    if denominator != 0 {
        Ok(numerator / denominator)
    } else {
        Err(DivError::DivisionByZero)
    }
}

/* 9. -----------------------------------------------------------------------*/
// Get text from a test file.

use std::fs::File;
use std::io::{self, Read};

pub fn read_name_from_file() -> Result<String, io::Error> {
    let mut name = String::new();

    File::open("tests/file_read_test.txt")?.read_to_string(&mut name)?;

    Ok(name)
}

/* 10. -----------------------------------------------------------------------*/
// Get the area of a circle.

#[allow(dead_code)]
pub struct Shape {
    name: String,
    area: f64,
}

#[allow(dead_code)]
pub struct Circle {
    shape: Shape,
    radius: f64,
}

#[allow(dead_code)]
impl Shape {
    fn new(name: String, area: f64) -> Self {
        Self { name, area }
    }

    fn get_area(&self) -> f64 {
        self.area
    }
}

#[allow(dead_code)]
impl Circle {
    fn new(name: String, radius: f64) -> Self {
        Self {
            shape: Shape::new(name, 0.0), // poor design, but: default shape area to 0.0
                                                // we'll just use the calculation for area in Circles Impl
            radius,
        }
    }

    fn get_area(&self) -> f64{
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub fn get_circle_area(radius: f64) -> f64{

    let circle = Circle::new(String::from("Circle"), radius);

    circle.get_area() 

}
