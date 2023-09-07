pub fn to_camel_case(input: &str) -> String {
    let chars = input.chars();
    let mut output = String::new();
    let mut prev: Option<char> = None;
    for char in chars {
        if char.is_uppercase() && prev.map(|prev_char| prev_char.is_lowercase()).unwrap_or(false) {
            output.push('_');
        }
        if char.is_uppercase() {
            for char in char.to_lowercase() {
                output.push(char);
            }
        } else {
            output.push(char);
        }
        prev.replace(char);
    }
    output
}
