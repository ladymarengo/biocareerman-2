use std::fs::read_to_string;

use super::*;

pub fn create_library(mut game_progress: ResMut<GameProgress>) {
    game_progress.library.letters = read_to_string("assets/text/characters.txt")
        .unwrap()
        .split('\n')
        .map(|s| {
            s.split(' ')
                .map(|c| c.chars().collect::<Vec<char>>()[0])
                .collect()
        })
        .collect();

    game_progress.library.len = read_to_string("assets/text/lengths.txt")
        .unwrap()
        .split('\n')
        .map(|s| {
            let tuple = s.split_once('\t').unwrap();
            Length {
                min: tuple.0.parse::<usize>().unwrap(),
                max: tuple.1.parse::<usize>().unwrap(),
            }
        })
        .collect();

    game_progress.library.news = read_to_string("assets/text/news.txt")
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    game_progress.money = 1000;
    game_progress.humanness = 100;
    game_progress.day = 1;

    game_progress.modes = read_to_string("assets/text/modes.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| {
            let mode = s.split('\n').collect::<Vec<&str>>();
            (
                Mode {
                    name: mode[0].to_string(),
                    price: mode[1].parse::<usize>().unwrap(),
                    desc: mode[2].to_string(),
                    humanness_impact: mode[3].parse::<i32>().unwrap(),
                },
                false,
            )
        })
        .collect();

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

pub struct Customers {
	random_word: Vec<(String, String)>,
	random_letter: Vec<String>,
}
