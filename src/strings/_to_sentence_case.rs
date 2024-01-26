// Converts an input to sentence_CASE. Does not filter out any special characters.
pub fn to_sentence_case(input: &str) -> String {
    let chars = input.chars();
    let mut output = String::new();
    let mut prev: Option<char> = None;
    for char in chars {
        todo!()
    }
    output
}

pub trait ToSentenceCase {
    fn to_sentence_case(&self) -> String;
}

impl ToSentenceCase for &str {
    fn to_sentence_case(&self) -> String {
        to_sentence_case(self)
    }
}

#[cfg(test)]
mod tests {
    use super::to_sentence_case;

    macro_rules! test_case {
        ($input:expr, $expected:expr) => {
            assert_eq!(to_sentence_case($input), $expected)
        };
    }
    #[test]
    fn to_camel_case_converts() {
        test_case!("FoodTruck", "Food truck");
        test_case!("foodTruck", "Food truck");
        test_case!("FOodTRUck", "Food truck");
        test_case!("foodTruck", "Food truck");
        test_case!("FOOD_TRUCK", "Food truck");
        test_case!("foodtruck", "Food truck");
        test_case!("_food_truck", "Food truck");
        test_case!("_FOOD_TRUCK", "Food truck");
        test_case!("food_truck", "Food truck");
    }
}
