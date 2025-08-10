use rand::prelude::IndexedRandom;

use crate::talk::globals::{
    DEFAULT_RESPONSES, FACTS, GOODBYE_INPUTS, GOODBYE_RESPONSES, INTRO_RESPONSES, REFLECTED_WORDS,
    REGEX_RESPONSES,
};

pub fn preprocess(input: &str) -> String {
    input
        .trim_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace())
        .to_lowercase()
}

fn random_element_from(list: &[&'static str]) -> &'static str {
    list.choose(&mut rand::rng()).unwrap_or(&"")
}

fn reflect(fragment: &str) -> String {
    fragment
        .split_whitespace()
        .map(|word| REFLECTED_WORDS.get(word).cloned().unwrap_or(word))
        .collect::<Vec<_>>()
        .join(" ")
}

fn lookup_response(input: &str) -> String {
    for (re, responses) in REGEX_RESPONSES.iter() {
        if let Some(captures) = re.captures(input) {
            let response = random_element_from(responses);
            if response.contains("%s") {
                if let Some(fragment) = captures.get(1) {
                    let reflected = reflect(fragment.as_str());
                    return response.replace("%s", &reflected);
                }
            } else {
                return response.to_string();
            }
        }
    }

    random_element_from(&DEFAULT_RESPONSES).to_string()
}

pub fn reply(input: &str) -> (String, bool) {
    let input = preprocess(input);
    if GOODBYE_INPUTS.contains(input.as_str()) {
        (random_element_from(&GOODBYE_RESPONSES).to_string(), true)
    } else {
        (lookup_response(&input), false)
    }
}

pub fn get_intro_responses(name: &str) -> Vec<String> {
    let mut intros: Vec<String> = INTRO_RESPONSES
        .iter()
        .map(|r| r.replace("{}", name))
        .collect();
    intros.push(random_element_from(&FACTS).to_string());
    intros.push("How are you feeling today?".to_string());
    intros
}
