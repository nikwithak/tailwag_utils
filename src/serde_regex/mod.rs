use regex::Regex;

struct RegexExtractor {
    regex: Regex,
}

impl RegexExtractor {
    fn new(regex_str: &str) -> Result<Self, regex::Error> {
        let regex = Regex::new(regex_str)?;
        Ok(Self {
            regex,
        })
    }
}
