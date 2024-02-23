use std::env;
use std::io::Read;
use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};
use serde_json::Result;

use rand::prelude::*;

pub struct Phrase {
    pub queue: (String, String),
    pub colors: Vec<bool>,
    pub char_ptr: usize,
}

#[derive(Serialize, Deserialize)]
struct Words {
    content: Vec<String>,
}

fn gen_phrase() -> String {
    let json_path: String;
    match env::current_dir() {
        Ok(current_dir) => {
            json_path = format!("{}/{}", current_dir.display().to_string(), "words.json");
        }
        Err(e) => {
            panic!("Error al obtener el directorio actual: {}", e);
        }
    }
    let mut json = File::open(&json_path).expect("error opening json");
    let mut json_content = String::new();
    match json.read_to_string(&mut json_content) {
        Ok(_) => {}
        Err(e) => panic!("err reading json"),
    }
    let mut w: Words = serde_json::from_str(&json_content).expect("error deserializing");

    let mut rng = thread_rng();
    w.content.shuffle(&mut rng);
    let mut words = String::new();
    let mut c = 0;
    for word in w.content {
        words.push_str(format!("{} ", word).as_str());
        c += 1;
        if c > 5 {
            break;
        }
    }
    words
}

impl Phrase {
    pub fn new() -> Phrase {
        let p = Phrase {
            queue: (String::from(gen_phrase()), String::from(gen_phrase())),
            colors: Vec::new(),
            char_ptr: 0,
        };
        p
    }

    pub fn reset(&mut self) {
        // move queue.1 (pending) to queue.0 (current)
        self.queue.0.clear();
        self.queue.0.push_str(self.queue.1.as_str());

        // gen new queue.1
        self.queue.1.clear();
        self.queue.1.push_str(&gen_phrase());
        self.char_ptr = 0;
        self.colors = Vec::new();
    }

    pub fn update(&mut self, correct: Option<bool>) {
        if let Some(v) = correct {
            self.colors.push(v);
            self.char_ptr += 1;
        }

        // check if ptr reached end of phrase
        if self.char_ptr == self.queue.0.len() {
            self.reset();
        }
    }

    pub fn delete(&mut self) {
        if self.char_ptr == 0 {
            return;
        }
        self.char_ptr -= 1;
        self.colors.pop();
    }
}
