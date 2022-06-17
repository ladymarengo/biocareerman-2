use bevy::prelude::*;
use chrono::prelude::*;
use crate::work::MiniGame;
use std::fs::File;
use std::io::Write;

pub struct LogBook
{
	pub tasks: Vec<Vec<TaskLog>>,
}

pub struct TaskLog
{
	pub task_type: MiniGame,
	pub length: usize,
	pub time: u128,
	pub errors: usize,
}

pub fn log_task(
	logbook: &mut ResMut<LogBook>,
	task_type: MiniGame,
	length: usize,
	time: u128,
	errors: usize,
	day: usize,
) {
	logbook.tasks[day].push(TaskLog {
		task_type,
		length,
		time,
		errors,
	});
}

pub fn add_logging(mut commands: Commands)
{
	commands.insert_resource(LogBook {
		tasks: Vec::new(),
	});
}

pub fn save_to_file() {
	let local: DateTime<chrono::Local> = chrono::Local::now();
	
	println!("{local}");
	let mut output = File::create(format!("logs/log{}.log", local)).unwrap();
    write!(output, "Hi").unwrap();
}