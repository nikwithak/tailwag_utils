pub mod strings;

#[cfg(test)]
mod tests {
    use super::strings::to_camel_case;

    #[test]
    fn to_camel_case_converts() {
        assert_eq!(to_camel_case("FoodTruck"), "food_truck");
        assert_eq!(to_camel_case("foodTruck"), "food_truck");
        assert_eq!(to_camel_case("FOodTRUck"), "food_truck");
        assert_eq!(to_camel_case("foodTruck"), "food_truck");
        assert_eq!(to_camel_case("foodtruck"), "foodtruck");
        assert_eq!(to_camel_case("_food_truck"), "_food_truck");
    }
}
