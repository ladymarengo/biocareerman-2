use ::rand::Rng;

pub fn get_random_word(valid_letters: &[char], min_length: usize, max_length: usize) -> String {
	let mut rng = ::rand::thread_rng();
    let length: i32 = rng.gen_range(min_length as i32..=max_length as i32);
	let mut letter: usize;

	let mut word: Vec<char> = Vec::new();
	for _i in 0..length {
		letter = rng.gen_range(0..valid_letters.len());
		word.push(valid_letters[letter]);
	}
	word.into_iter().collect()
}
