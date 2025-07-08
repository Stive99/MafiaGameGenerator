use crate::role::Role;
use crate::config::GameMode;
use crate::error::AppError;
use std::collections::HashSet;
use std::io::{self, Write};
use std::fs;

/**
 * Запрашивает у пользователя количество игроков через консоль.
 */
pub fn prompt_for_player_count() -> Result<u8, AppError> {
	print!("Введите количество игроков (например, 10): ");
	io::stdout().flush()?;

	let mut input = String::new();
	io::stdin().read_line(&mut input)?;

	let count = input.trim().parse::<u8>()?;
	Ok(count)
}

pub fn prompt_for_game_mode() -> Result<GameMode, AppError> {
	println!("\n--- Выбор режима игры ---");
	println!("1. Классический");
	println!("2. Расширенный (с Маньяком)");
	print!("Выберите режим (1-2): ");
	io::stdout().flush()?;

	let mut input = String::new();
	io::stdin().read_line(&mut input)?;
	let trimmed_input = input.trim();

	match input.trim() {
		"1" => Ok(GameMode::Classic),
		"2" => Ok(GameMode::Extended),
		_ => Err(AppError::InvalidMenuChoice(trimmed_input.to_string())),
	}
}

fn validate_and_get_name(player_index: usize, unique_names: &HashSet<String>) -> Result<String, AppError> {
	// Бесконечный цикл, который будет повторяться, пока не будет введено корректное имя
	print!("Введите имя для Игрока {player_index}: ");
	io::stdout().flush().unwrap_or_default();

	let mut name = String::new();
	io::stdin().read_line(&mut name).unwrap_or_default();
	let trimmed_name = name.trim();

	// Проверяем все условия
	if trimmed_name.is_empty() {
		return Err(AppError::EmptyPlayerName);
	}

	let invalid_chars: &[char] = &['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
	if trimmed_name.chars().any(|c| invalid_chars.contains(&c)) {
		return Err(AppError::InvalidCharactersInName(trimmed_name.to_string()));
	}

	// ИСПРАВЛЕННАЯ ЛОГИКА: Сначала проверяем, потом добавляем.
	let normalized_name = trimmed_name.to_lowercase();
	if unique_names.contains(&normalized_name) {
		return Err(AppError::DuplicatePlayerName(trimmed_name.to_string()));
	}

	Ok(trimmed_name.to_string())
}

/**
 * Запрашивает у пользователя имена для каждого игрока.
 */
pub fn prompt_for_player_names(player_count: u8) -> Result<Vec<String>, AppError> {
	println!("\n--- Ввод имен игроков ---");
	let mut names = Vec::with_capacity(player_count as usize);
	let mut unique_names = HashSet::with_capacity(player_count as usize);

	for i in 0..(player_count as usize) {
		loop {
			// Пытаемся получить и валидировать имя
			match validate_and_get_name(i + 1, &unique_names) {
				Ok(name) => {
					// Если успешно, добавляем имя в ОБА списка и выходим из цикла
					unique_names.insert(name.to_lowercase());
					names.push(name);
					break;
				}
				Err(e) => {
					// Если ошибка, печатаем ее и цикл продолжается
					eprintln!("{e}\nПожалуйста, попробуйте еще раз.");
				}
			}
		}
	}

	Ok(names)
}

/**
 * Создает папку "roles" и записывает в нее файлы с ролями для каждого игрока.
 */
pub fn write_role_files(players: &[(String, Role)]) -> Result<(), AppError> {
	let output_dir = "roles";

	// Создаем папку `roles`. `create_dir_all` не выдает ошибку, если папка уже существует.
	fs::create_dir_all(output_dir)?;

	// Проходим по вектору с ролями, получая и индекс, и саму роль.
	for (player_name, role) in players.iter() {
		// Создаем имя файла на основе имени игрока.
		// Заменяем пробелы на подчеркивания для надежности.
		let safe_filename = player_name.replace(' ', "_");
		let file_path = format!("{output_dir}/{safe_filename}.txt");

		// Формируем содержимое файла, используя имя игрока.
		let content = format!(
			"Игрок: {}\n\nВаша роль: {}\n\nОписание:\n{}\n",
			player_name,
			role.get_name(),
			role.get_description()
		);

		// И здесь тоже `?` делает код чистым и лаконичным.
		fs::write(&file_path, content)?;
	}

	Ok(())
}