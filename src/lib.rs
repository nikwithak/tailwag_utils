pub mod strings;

#[cfg(test)]
mod tests {
    use super::strings::to_snake_case;

    #[test]
    fn to_camel_case_converts() {
        assert_eq!(to_snake_case("FoodTruck"), "food_truck");
        assert_eq!(to_snake_case("foodTruck"), "food_truck");
        assert_eq!(to_snake_case("FOodTRUck"), "food_truck");
        assert_eq!(to_snake_case("foodTruck"), "food_truck");
        assert_eq!(to_snake_case("foodtruck"), "foodtruck");
        assert_eq!(to_snake_case("_food_truck"), "_food_truck");
        assert_eq!(to_snake_case("FOOD_TRUCK"), "food_truck");
        assert_eq!(to_snake_case("_FOOD_TRUCK"), "_food_truck");
    }
}
