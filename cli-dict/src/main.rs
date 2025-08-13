use reqwest::blocking::get;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct Meaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    definitions: Vec<Definition>,
}

#[derive(Debug, Deserialize)]
struct Definition {
    definition: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    meanings: Vec<Meaning>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cli-dict <word>");
        std::process::exit(1);
    }

    let word = &args[1];
    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

    println!("Searching for: {}\n", word);

    match get(&url) {
        Ok(resp) => match resp.json::<Vec<ApiResponse>>() {
            Ok(entries) => {
                for entry in entries {
                    for meaning in entry.meanings {
                        println!("Part of Speech: {}", meaning.part_of_speech);
                        for def in meaning.definitions {
                            println!(" - {}", def.definition);
                        }
                        println!();
                    }
                }
            }
            Err(_) => eprintln!("Could not parse dictionary response."),
        },
        Err(err) => eprintln!("Error fetching definition: {}", err),
    }
}
