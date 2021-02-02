mod error;

use error::AlgoError;

pub fn linear_search(arr: &[i32], rand_num: i32) -> Result<i32, AlgoError> {
    let mut comparison_counter = 1;

    for num in arr.iter() {
        if &rand_num == num {
            return Ok(comparison_counter);
        } else {
            comparison_counter += 1;
        }
    }
    Err(AlgoError::InputError)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn linear_search_output_matches_input() {
        let arr = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];

        let result = linear_search(&arr, 7).unwrap();
        let expected_result = 7;
        assert_eq!(result, expected_result);
    }
    #[test]
    fn linear_search_input_invalid() {
        let arr = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];

        assert_eq!(linear_search(&arr, 37), Err(AlgoError::InputError));
    }
}
