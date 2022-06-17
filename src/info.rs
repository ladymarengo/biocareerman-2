use std::fs::read_to_string;

use super::*;

pub fn create_library(mut game_progress: ResMut<GameProgress>) {
    let letters: Vec<Vec<char>> = read_to_string("assets/text/characters.txt")
        .unwrap()
        .split('\n')
        .map(|s| s.split(' ').map(|c| c.chars().collect::<Vec<char>>()[0]).collect())
        .collect();

    let len: Vec<Length> = read_to_string("assets/text/lengths.txt")
        .unwrap()
        .split('\n')
        .map(|s| {
            let tuple = s.split_once('\t').unwrap();
            Length { min: tuple.0.parse::<usize>().unwrap(), max: tuple.1.parse::<usize>().unwrap() }
        })
        .collect();

    let news: Vec<String> = read_to_string("assets/text/news.txt")
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    game_progress.library.letters = letters;
    game_progress.library.len = len;
    game_progress.library.news = news;
    game_progress.money = 1000;
    game_progress.humanness = 100;
    game_progress.day = 1;
    game_progress.modes = vec![
        (
            Mode {
                name: "Cyborg I".to_string(),
                price: 200,
                desc: "Eagle-eyed employee has a right for one error without a penalty."
                    .to_string(),
                humanness_impact: -10,
            },
            false,
        ),
        (
            Mode {
                name: "Smiley".to_string(),
                price: 200,
                desc: "Additional time for your tasks.".to_string(),
                humanness_impact: -10,
            },
            false,
        ),
        (
            Mode {
                name: "Bcrrmn2".to_string(),
                price: 400,
                desc: "Type any button for a vowel.".to_string(),
                humanness_impact: -40,
            },
            false,
        ),
        (
            Mode {
                name: "No Time To Type".to_string(),
                price: 200,
                desc: "Limit the length of your tasks.".to_string(),
                humanness_impact: -30,
            },
            false,
        ),
        (
            Mode {
                name: "Writetyper's starterkit".to_string(),
                price: 150,
                desc: "Type any button for the letters 'q' and 'e'.".to_string(),
                humanness_impact: -10,
            },
            false,
        ),
        (
            Mode {
                name: "Bot Remover 2000".to_string(),
                price: 600,
                desc: "Restores some of your humanness.".to_string(),
                humanness_impact: 25,
            },
            false,
        ),
    ];
    game_progress.customers.random_word = read_to_string("assets/text/minigame_words.txt")
        .unwrap()
        .split('\n')
        .map(|s| {
            let tuple = s.split_once('@').unwrap();
            (tuple.0.to_string(), tuple.1.to_string())
        })
        .collect();

    game_progress.customers.random_letter = read_to_string("assets/text/minigame_letters.txt")
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect();
}

pub struct Library {
    pub letters: Vec<Vec<char>>,
    pub len: Vec<Length>,
    pub news: Vec<String>,
}

pub struct Length {
	pub min: usize,
	pub max: usize,
}

pub struct Mode {
    pub name: String,
    pub price: usize,
    pub desc: String,
    pub humanness_impact: i32,
}
