use super::*;

pub fn create_library(mut game_progress: ResMut<GameProgress>) {
    let letters: Vec<Vec<char>> = vec![
        vec!['q', 'e'],
        vec!['q', 'e', 'l'],
        vec!['q', 'e', 'l'],
        vec!['q', 'e', 'l', 'k'],
        vec!['a', 'f', 'u'],
        vec!['a', 'f', 'u'],
        vec!['i', 'c', 'u', 'v', 'r'],
        vec!['f'],
        vec!['a', 's', 'd', 'f', 'g'],
        vec!['a', 's', 'd', 'i', 'o', 'p'],
        vec!['a', 'e', 'g', 'o', 'u'],
        vec!['g', 'j', 'h', 'f'],
        vec![
            'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 'd', 'f', 'g', 'h', 'j', 'k', 'l',
        ],
        vec!['l', 'i', '1', 'o', '0', 'j'],
        vec!['q', 'e'],
    ];

    let min_len: Vec<usize> = vec![1, 2, 4, 2, 2, 4, 3, 2, 3, 3, 4, 3, 7, 4, 1];
    let max_len: Vec<usize> = vec![3, 4, 6, 5, 5, 7, 5, 8, 6, 6, 6, 6, 10, 7, 5];
    let news: Vec<String> = vec![
        "Breaking news from the corporate world!\nGovCorp has consolidated all the call centers of the \ncountry into one. Critics of the merger warn \nof increasing queuing time. Call center specialist \nAnne Swear had this to say: \"It's not even hard \nto press two buttons. I'm not sure why \nis the job paid by the taxes.\"".to_string(),
		"Lorem ipsum dolor sit amet, consectetur \nadipiscing elit. Phasellus ac lacus ac ipsum tincidunt \neuismod. Aliquam erat volutpat. Etiam ornare \nest non egestas aliquam. Curabitur a tempor \nsapien. Etiam pharetra.".to_string(),
		"JuLiUs_CaEsAr-BoT#42 says on the annual \nMethGala: \"Veni, vidi, vici!\"".to_string(),
		"Happy news! Govt informs it's netizens to \ncome fetch the 85th booster shot. This \ntime: no refunds permitted.".to_string(),
		"Alert! Do not believe anything you've learnt before.".to_string(),
		"The sun always shines at The Dumpster Inn. \nCome book your own occasionally ratless alley.".to_string(),
		"...".to_string(),
		"Don't push yourself too hard today. \nGo outside, read a book, lay back.".to_string(),
		"Don't you know what to do? Search no more! \nCome hang out with our second best cows \nand chickens on the AnalogFarm.".to_string(),
		"Hafdsi iojasdf fodsaij jfj fjewro rquq. Fdsjf jro.".to_string(),
		"Td thr wll b  lt f vwls n r cstmrs rdrs'. \nPrhps  shld b  md fr tht.".to_string(),
		"OneNationHolidayBeach invites you to spend your \nretirement days on the hot sand with a cold drink.".to_string(),
		"Persistent bird will get the worm. If you like \nto work hard, today is your day.".to_string(),
		"GovCorp binary specialist warns our audience of \nsimilarity of appearance. On the other hand \nthe weather is particularly sunny.".to_string(),
		"It is a wonderful day to retire!".to_string(),
    ];

    game_progress.library.letters = letters;
    game_progress.library.min_len = min_len;
    game_progress.library.max_len = max_len;
    game_progress.library.news = news;
    game_progress.money = 500;
    game_progress.humanness = 100;
    game_progress.day = 1;
	game_progress.modes = vec![
		(Mode{
			name: "Cyborg I".to_string(),
			price: 200,
			desc: "Eagle-eyed employee has a right for\none error without a penalty.".to_string(),
			humanness_impact: -10,
		}, false),
		(Mode{
			name: "Smiley".to_string(),
			price: 200,
			desc: "Additional time for your tasks.".to_string(),
			humanness_impact: -10,
		}, false),
		(Mode{
			name: "Bcrrmn2".to_string(),
			price: 400,
			desc: "Type any button for a vowel.".to_string(),
			humanness_impact: -40,
		}, false),
		(Mode{
			name: "No Time To Type".to_string(),
			price: 200,
			desc: "Limit the length of your tasks.".to_string(),
			humanness_impact: -30,
		}, false),
		(Mode{
			name: "Writetyper's starterkit".to_string(),
			price: 150,
			desc: "Type any button for the letters 'q' and 'e'.".to_string(),
			humanness_impact: -10,
		}, false),
		(Mode{
			name: "Bot Remover 2000".to_string(),
			price: 600,
			desc: "Restores some of your humanness.".to_string(),
			humanness_impact: 25,
		}, false),];
	game_progress.customers = vec![
		CallCenterTask {
			task: "I want refurbished parts for my ...".to_string(),
		},
		CallCenterTask {
			task: "... is broken, could you fix it?".to_string(),
		},
		CallCenterTask {
			task: "I would like to return ...".to_string(),
		},
		CallCenterTask {
			task: "Help! Emergency! We need at least 10 ... here!".to_string(),
		},
		CallCenterTask {
			task: "Flat Apple Factory needs fresh ... for R&D".to_string(),
		},
		CallCenterTask {
			task: "Pizza one plz, plenty ...".to_string(),
		},
		CallCenterTask {
			task: "Is this BiO Career Man on the phone?".to_string(),
		},
		CallCenterTask {
			task: "Gyd morni. Vi fine virus. Egcexute dis fail.".to_string(),
		},
		CallCenterTask {
			task: "It's your destiny calling. What do you reckon, recon?".to_string(),
		},
		CallCenterTask {
			task: "My eagle-eye mod is broken. Can you bring me ...?".to_string(),
		},
		CallCenterTask {
			task: "Where to check-in for the orbital flight?".to_string(),
		},
		CallCenterTask {
			task: "One teleportation, please. Address is ...".to_string(),
		},
		CallCenterTask {
			task: "Can you bring my organic nose, package id is ...".to_string(),
		},
	];
}

pub struct Library {
    pub letters: Vec<Vec<char>>,
    pub min_len: Vec<usize>,
    pub max_len: Vec<usize>,
    pub news: Vec<String>,
}

pub struct Mode {
	pub name: String,
    pub price: usize,
    pub desc: String,
    pub humanness_impact: i32,
}

pub struct CallCenterTask {
	pub task: String,
}
