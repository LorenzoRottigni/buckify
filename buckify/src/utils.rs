use regex::Regex;

pub fn slugify(input: &str) -> String {
    let re = Regex::new(r"[^\w\s-]").unwrap();
    let cleaned = re.replace_all(input, "");
    let re_whitespace = Regex::new(r"[\s-]+").unwrap();
    let hyphenated = re_whitespace.replace_all(&cleaned, "-");
    hyphenated.to_lowercase()
}

pub fn extract_file_name(path: &str) -> String {
    path.rsplit('/')
        .next()
        .unwrap_or("") // Provide a default empty string if no segment is found
        .to_string()
}
