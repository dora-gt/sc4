extern crate alfred;
extern crate clap;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use clap::App;
use core::borrow::Borrow;
use regex::Regex;
use std::io;

lazy_static! {
  static ref REGEX_SNAKE_CASE: Regex = { Regex::new("^[a-z]+((_[a-z]+)+$|$)").unwrap() };
  static ref REGEX_CAMEL_CASE: Regex = { Regex::new("^([A-Z][a-z]*)+$").unwrap() };
  static ref REGEX_KEBAB_CASE: Regex = { Regex::new("^[a-z]+((-[a-z]+)+$|$)").unwrap() };
}

fn main() {
    let matches = App::new("sc4...snake-case camel-case converter")
        .version("0.1")
        .author("Taiga Nakayama <dora@dora-gt.jp>")
        .about("Converts snake-case to camel-case and vice versa.")
        .args_from_usage(
            "
            -c, --case=[CASE] 'snake, camel, kebab'
            -a                'Output for Script Filter for Alfred'
            <INPUT>           'text you want to convert'
        ",
        )
        .get_matches();

    let input: &str = matches.value_of("INPUT").unwrap();
    let mut input = input.to_string();
    let mut cm = CaseManipulator::new(&mut input);

    let into_case = matches.value_of("case");
    let into_case: Option<Cases> = match into_case {
        Some(case) => match case {
            "snake" => Some(Cases::SnakeCase),
            "camel" => Some(Cases::CamelCase),
            "kebab" => Some(Cases::KebabCase),
            _ => None,
        },
        None => None,
    };
    let into_case = into_case.unwrap_or(cm.get_default_conversion());
    let converted = cm.convert_into(into_case);

    if matches.is_present("a") {
        // output for Alfred
        alfred::json::Builder::with_items(&[alfred::ItemBuilder::new(
            "Copy ".to_owned() + &converted + " to Clipboard",
        )
        .subtitle("converted from ".to_owned() + &input)
        .arg(&converted)
        .into_item()])
        .write(io::stdout())
        .ok();
    } else {
        // output just the converted
        println!("{}", converted);
    }
}

#[derive(Debug)]
enum Cases {
    SnakeCase,
    CamelCase,
    KebabCase,
}

struct CaseManipulator<'a> {
    text: &'a mut str,
}

impl<'a> CaseManipulator<'a> {
    pub fn new(text: &'a mut str) -> Self {
        CaseManipulator { text }
    }

    /// snake_case
    pub fn is_snake_case(&self) -> bool {
        REGEX_SNAKE_CASE.is_match(self.text)
    }

    /// CamelCase
    pub fn is_camel_case(&self) -> bool {
        REGEX_CAMEL_CASE.is_match(self.text)
    }

    /// kebab-case
    pub fn is_kebab_case(&self) -> bool {
        REGEX_KEBAB_CASE.is_match(self.text)
    }

    /// what case the text is
    pub fn get_case(&self) -> Option<Cases> {
        if self.is_snake_case() {
            return Some(Cases::SnakeCase);
        } else if self.is_camel_case() {
            return Some(Cases::CamelCase);
        } else if self.is_kebab_case() {
            return Some(Cases::KebabCase);
        } else {
            return None;
        }
    }

    pub fn convert_into(&mut self, case: Cases) -> String {
        // just initializing with capacity of double of the length
        let mut joined = String::with_capacity(self.text.len() * 2);
        let items = match self.get_case() {
            Some(case) => match case {
                Cases::SnakeCase => self.break_snake_case(),
                Cases::CamelCase => self.break_camel_case(),
                Cases::KebabCase => self.break_kebab_case(),
            },
            _ => vec![self.text.borrow()],
        };
        match case {
            Cases::SnakeCase => joined.push_str(items.join("_").to_lowercase().as_str()),
            Cases::CamelCase => {
                for item in items {
                    for (index, item_char) in item.chars().into_iter().enumerate() {
                        let push_char = match index {
                            0 => item_char.to_ascii_uppercase(),
                            _ => item_char,
                        };
                        joined.push(push_char);
                    }
                }
            }
            Cases::KebabCase => joined.push_str(items.join("-").to_lowercase().as_str()),
        };
        joined
    }

    /// get most suitable conversion
    pub fn get_default_conversion(&self) -> Cases {
        match self.get_case() {
            Some(case) => match case {
                Cases::SnakeCase => Cases::CamelCase,
                Cases::CamelCase => Cases::SnakeCase,
                Cases::KebabCase => Cases::CamelCase,
            },
            None => Cases::CamelCase,
        }
    }

    fn break_snake_case(&self) -> Vec<&str> {
        self.text.split("_").collect()
    }

    fn break_camel_case(&self) -> Vec<&str> {
        let regex = Regex::new("[A-Z][a-z]*").unwrap();
        regex
            .captures_iter(self.text)
            .map(|capture| capture.get(0).unwrap().as_str())
            .collect()
    }

    fn break_kebab_case(&self) -> Vec<&str> {
        self.text.split("-").collect()
    }
}
