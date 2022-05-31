use crate::static_map::DICT;

pub fn convert(input_text: &str) -> String {
    if input_text.is_empty() { return "".to_string(); }
    let mut result = String::new();

    for word in input_text.split(' ') {
        match DICT.get(word) {
            None => {
                result.push_str(" ");
                result.push_str(word);
                // result.push_str("#");
            },
            Some(t) => {
                result.push_str(" ");
                result.push_str(t);
            },
        }
    }

    return result;
}

