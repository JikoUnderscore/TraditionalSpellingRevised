use fltk::text::TextBuffer;
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

pub fn change_color(text: &str, style_buff_ref: &mut TextBuffer) {
    for word in text.split(" ") {
        match DICT.get(word) {
            None => {
                style_buff_ref.append(" ");
                style_buff_ref.append(&"B".repeat(word.len()));
            },
            Some(&reformed_word) => {
                if reformed_word == word {
                    debug_assert_eq!(reformed_word.len(), word.len());
                    style_buff_ref.append(" ");
                    style_buff_ref.append(&"A".repeat(reformed_word.len()));
                } else {
                    style_buff_ref.append(" ");
                    style_buff_ref.append(&"C".repeat(reformed_word.len()));
                }
            },
        }
    }
}


pub fn web_scpaker(string: &str) -> anyhow::Result<String> {
    let url = format!("http://cube.elte.hu/index.pl?s={}&fullw=on&t=&syllcount=&maxout=&wfreq=0-9&grammar=", string);
    let text = reqwest::blocking::get(url)?
        .text()?;


    let parsed_html = scraper::Html::parse_document(&text);

    let selector = match scraper::Selector::parse("span.ipa") {
        Ok(res) => res,
        Err(_) => {
            return Err(anyhow::anyhow!("Parse Error"));
        }
    };

    let span_text = parsed_html
        .select(&selector)
        .flat_map(|el| el.text())
        .collect::<Vec<_>>();

    return Ok(span_text.join("\n"));
}