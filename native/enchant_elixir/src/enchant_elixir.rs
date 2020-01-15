extern crate enchant;

use crate::atoms::{error, ok};
use enchant::Dict;
use rustler::{Encoder, Env, NifStruct, Term};

#[derive(NifStruct)]
#[module = "Enchant.Word"]
struct Item {
    word: String,
    correct: bool,
    suggestions: Vec<String>,
}

fn item_for_word(word: &str, dict: &Dict) -> Item {
    if dict.check(word).expect("Could not check the word") {
        return Item {
            word: word.to_owned(),
            suggestions: Vec::new(),
            correct: true,
        };
    } else {
        return Item {
            word: word.to_owned(),
            suggestions: dict.suggest(word),
            correct: false,
        };
    }
}

pub fn check<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, rustler::Error> {
    let word: &str = args[0].decode()?;
    if word.len() <= 1 {
        return Ok((ok(), Item {
            word: word.to_owned(),
            suggestions: Vec::new(),
            correct: true,
        }).encode(env));
    }

    let lang: &str = args[1].decode()?;

    let mut broker = enchant::Broker::new();

    if !broker.dict_exists(lang) {
        return Ok((error(), "Invalid dictionary").encode(env));
    } else {
        match broker.request_dict(lang) {
            Ok(dict) => {
                return Ok((ok(), item_for_word(word, &dict)).encode(env));
            }
            Err(msg) => return Ok((error(), msg).encode(env)),
        }
    }
}

pub fn load<'a>(_env: Env, _info: Term<'a>) -> bool {
    true
}
