use std::fs::read_to_string;

use super::*;

pub fn create_library(mut commands: Commands) {
    let letters: Vec<Vec<char>> = read_to_string("assets/text/characters.txt")
        .unwrap()
        .split('\n')
        .map(|s| {
            s.split(' ')
                .map(|c| c.chars().collect::<Vec<char>>()[0])
                .collect()
        })
        .collect();

	let len: Vec<Length> = read_to_string("assets/text/lengths.txt")
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

	let news: Vec<String> = read_to_string("assets/text/news.txt")
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    let money: usize = 1000;
    let humanness: i32 = 100;
    let day: usize = 1;
	let max_days: usize = 15;

    let modes: Vec<(info::Mode, bool)> = read_to_string("assets/text/modes.txt")
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

    let random_word: Vec<(String, String)> = read_to_string("assets/text/minigame_words.txt")
        .unwrap()
        .split('\n')
        .map(|s| {
            let tuple = s.split_once('@').unwrap();
            (tuple.0.to_string(), tuple.1.to_string())
        })
        .collect();

    let random_letter: Vec<String> = read_to_string("assets/text/minigame_letters.txt")
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect();

	commands.insert_resource(GameProgress {money, humanness, day, max_days, library: Library { letters, len, news}, modes, customers: Customers { random_word, random_letter}})
}


pub struct GameProgress {
    pub money: usize,
    pub humanness: i32,
    pub day: usize,
    pub max_days: usize,
    pub library: info::Library,
    pub modes: Vec<(info::Mode, bool)>,
    pub customers: info::Customers,
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
	pub random_word: Vec<(String, String)>,
	pub random_letter: Vec<String>,
}
