#[cfg(test)]
mod tests {
    //use std::fmt::Error;

    use buggy_rust_1::*;

    #[test]
    fn two_plus_two_is_four() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn string_length_excluding_whitespace() {
        let result = get_trimmed_string_length("  Hello  ");
        assert_eq!(result, 5);
    }

    #[test]
    fn get_second_two_elements_test() {
        let test_array: [usize; 4] = [10,11,12,13];
        let result = get_second_two_elements(&test_array);
        assert_eq!([11,12], result);

    }

    #[test]
    fn negate_the_array_test() {
        
        let mut test_array: [i32; 4] = [10,11,12,13];

        negate_the_array(&mut test_array);

        assert_eq!([-10, -11, -12, -13], test_array);

    }

    #[test]
    fn fizz_the_odds_test() {

        assert_eq!("fizz", fizz_the_odds(3));

    }


    #[test]
    fn point_quadrant_test() {

        let test_quadrant = get_point_quadrant(10, 10);
   
        assert_eq!(Quadrant::One, test_quadrant);
        
    }

    #[test]
    fn add_numbers_within_text_test() {

        let test_text = "h3r3's A s3nt3nc3 11";
   
        assert_eq!(17, add_numbers_within_text(test_text));
        
    }


    #[test]
    fn try_divide_test_one() {
   
        assert_eq!(None, try_divide_returns_option_type(10,0));
        
    }

    #[test]
    fn try_divide_test_two() {
   
        assert_eq!(Err(DivError::DivisionByZero), try_divide_returns_error_type(10,0));
        
    }

    #[test]
    fn read_name_from_file_test() {

        let name = read_name_from_file();
   
        match name {
            Ok(name) => assert_eq!(name, "Zackery Fleming"),
            Err(err) => panic!("Error reading file: {:?}", err),
        }
        
    }


    #[test]
    fn get_circle_area_test() {
   
        let radius = 2.0;

        let expected_area = std::f64::consts::PI * radius * radius;

        assert_eq!(expected_area, get_circle_area(radius) );
        
    }

}

//

