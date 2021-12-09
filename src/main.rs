use lazy_static::lazy_static;
use regex::Regex;
use web_sys::HtmlInputElement;
use yew::prelude::*;

mod dictionary;

use dictionary::Entry;

use std::fmt::Write;

use crate::dictionary::PartOfSpeech;

enum Msg {
    Search,
}

struct Model {
    dictionary: Vec<Entry>,
    input_ref: NodeRef,
    search_results: Vec<Entry>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            dictionary: Self::parse_dictionary(include_str!("dictionary.txt")),
            input_ref: NodeRef::default(),
            search_results: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Search => {
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    self.search_results.clear();

                    let query = input.value().to_lowercase();

                    if query.is_empty() {
                        return true;
                    }

                    for entry in &self.dictionary {
                        for keyword in entry.get_keywords() {
                            if keyword.to_lowercase().contains(&query) {
                                self.search_results.push(entry.clone());
                                break;
                            }
                        }
                    }

                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|_| Msg::Search);

        html! {
            <>
                <h1>{ "Dwarven Dictionary" }</h1>
                <input ref={ self.input_ref.clone() } type="text" { oninput }/>
                { for self.search_results.iter().map(|result| {
                    html! {
                        <div class="entry">
                            { result.word.to_string() }
                            { " (" }
                            { result.part_of_speech.to_string() }
                            { ") - " }
                            { result.translation.to_string() }
                        </div>
                    }
                }) }
            </>
        }
    }
}

impl Model {
    fn parse_dictionary(dictionary: &str) -> Vec<Entry> {
        lazy_static! {
            static ref WORD: Regex = Regex::new(r"(.+) - (.+)").unwrap();
            static ref TRANSLATION: Regex =
                Regex::new(r"(.+?) \((?:(.+), )?(n.|v.)\)(?:; )?").unwrap();
        }

        let mut result = Vec::new();

        for line in dictionary.lines() {
            let captures = WORD.captures(line).unwrap();

            let word = captures[1].to_string();
            let definitions = captures[2].to_string();

            for captures in TRANSLATION.captures_iter(&definitions) {
                let mut translation = captures[1].to_string();

                if let Some(extra) = captures.get(2) {
                    write!(translation, " ({})", extra.as_str()).unwrap();
                }

                let part_of_speech = match &captures[3] {
                    "n." => PartOfSpeech::Noun,
                    "v." => PartOfSpeech::Verb,
                    _ => unreachable!(),
                };

                result.push(Entry {
                    word: word.to_string(),
                    translation,
                    part_of_speech,
                })
            }
        }

        result
    }
}

fn main() {
    yew::start_app::<Model>();
}
