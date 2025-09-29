use crate::role::Role;
use crate::error::AppError;
use std::collections::HashSet;
use std::io::{self, Write};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::env;

/**
 * Запрашивает у пользователя количество игроков через консоль.
 */
pub fn prompt_for_player_count() -> Result<u8, AppError> {
	print!("Введите количество игроков (например, 10): ");
	io::stdout().flush()?;

	let mut input = String::new();
	io::stdin().read_line(&mut input)?;

	let trimmed_input = input.trim();

	// Дополнительная валидация входных данных
	if trimmed_input.is_empty() {
		// Создаем ошибку парсинга для пустой строки
		let err = "x".parse::<u8>().unwrap_err();
		return Err(AppError::ParseInt(err));
	}

	// Проверка на наличие только цифр
	if !trimmed_input.chars().all(|c| c.is_ascii_digit()) {
		// Создаем ошибку парсинга для недопустимых символов
		let err = "x".parse::<u8>().unwrap_err();
		return Err(AppError::ParseInt(err));
	}

	let count = trimmed_input.parse::<u8>()?;
	Ok(count)
}

/**
 * Валидирует и получает имя для игрока.
 */
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

	// Проверка максимальной длины имени
	if trimmed_name.len() > 50 {
		return Err(AppError::InvalidCharactersInName(trimmed_name.to_string()));
	}

	let invalid_chars: &[char] = &['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
	if trimmed_name.chars().any(|c| invalid_chars.contains(&c)) {
		return Err(AppError::InvalidCharactersInName(trimmed_name.to_string()));
	}

	// Дополнительная проверка на контрольные символы
	if trimmed_name.chars().any(|c| c.is_control()) {
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
 * Очищает имя файла, удаляя или заменяя недопустимые символы.
 */
fn sanitize_filename(filename: &str) -> String {
	// Более строгая санитизация имени файла
	filename
		.replace(' ', "_")
		.replace(|c: char| !c.is_alphanumeric() && c != '_' && c != '-', "")
		.trim_matches(|c: char| c == '.' || c == '_')
		.chars()
		.take(50) // Ограничиваем длину имени файла
		.collect()
}

/**
 * Создает папку "roles" и записывает в нее файлы с ролями для каждого игрока.
 */
pub fn write_role_files(players: &[(String, Role)]) -> Result<(), AppError> {
	let output_dir = "roles";

	// Создаем папку `roles`. `create_dir_all` не выдает ошибку, если папка уже существует.
	if let Err(e) = fs::create_dir_all(output_dir) {
		return Err(AppError::Io(e));
	}

	// Проходим по вектору с ролями, получая и индекс, и саму роль.
	for (player_name, role) in players.iter() {
		// Создаем имя файла на основе имени игрока.
		// Заменяем пробелы на подчеркивания для надежности.
		let safe_filename = sanitize_filename(player_name);

		// Дополнительная проверка: имя файла не должно быть пустым
		if safe_filename.is_empty() {
			return Err(AppError::InvalidFileName(player_name.clone()));
		}

		let file_path = format!("{output_dir}/{safe_filename}.txt");

		// Проверка безопасности: убедитесь, что путь к файлу находится в ожидаемом каталоге.
		if !is_path_safe(&file_path, output_dir) {
			return Err(AppError::InvalidFileName(player_name.clone()));
		}

		// Формируем содержимое файла, используя имя игрока.
		let content = format!(
			"Игрок: {}\n\nВаша роль: {}\n\nОписание:\n{}\n",
			player_name,
			role.get_name(),
			role.get_description()
		);

		// И здесь тоже `?` делает код чистым и лаконичным.
		if let Err(e) = fs::write(&file_path, content) {
			return Err(AppError::Io(e));
		}
	}

	// Проверяем, что файлы действительно созданы
	match fs::read_dir(output_dir) {
		Ok(entries) => {
			let count = entries.count();
			if count != players.len() {
				eprintln!("Предупреждение: Ожидается {} файлов, но создано {}", players.len(), count);
			}
		}
		Err(e) => {
			eprintln!("Предупреждение: Не удалось проверить содержимое папки '{}': {}", output_dir, e);
		}
	}

	Ok(())
}

/**
 * Проверяет, является ли путь к файлу безопасным и находится ли он в ожидаемом каталоге.
 */
fn is_path_safe(file_path: &str, expected_dir: &str) -> bool {
	let path = Path::new(file_path);
	let expected_base = Path::new(expected_dir);

	// Абсолютный путь всегда небезопасен в этом контексте.
	if path.is_absolute() {
		return false;
	}

	// Проверяем, что путь начинается с ожидаемой директории
	if !path.starts_with(expected_base) {
		return false;
	}

	// Создаём нормализованный путь, разрешая компоненты "." и "..".
	let mut clean_path = PathBuf::new();
	for component in path.components() {
		match component {
			Component::Prefix(_) | Component::RootDir => {
				// Если компонент превратился в префикс или корень, значит,
				// ".." вывел нас за пределы `expected_base`.
				return false;
			}
			Component::CurDir => {
				// Игнорируем "./"
				continue;
			}
			Component::ParentDir => {
				// Попытка подняться на уровень выше. Это и есть основная проверка.
				// Если мы не можем "подняться" в clean_path, значит, ".." указывает выше `expected_base`.
				if !clean_path.pop() {
					return false; // ".." указывает выше базового каталога
				}
			}
			Component::Normal(c) => {
				// Обычный компонент пути (файл или каталог), добавляем его.
				// Дополнительная проверка: имя компонента не должно содержать недопустимых символов
				let component_str = c.to_string_lossy();
				if component_str.contains(['/', '\\', '\0']) {
					return false;
				}
				clean_path.push(c);
			}
		}
	}

	// Это гарантирует, что даже после нормализации путь не вышел за пределы базового каталога.
	if !clean_path.starts_with(expected_base) {
		return false;
	}

	// Дополнительная проверка: путь должен содержать только один уровень вложенности
	// (не более одного компонента после expected_dir)
	let components_after_base: Vec<_> = clean_path
		.components()
		.skip(expected_base.components().count())
		.collect();

	if components_after_base.len() != 1 {
		return false;
	}

	// Если все проверки пройдены, путь считается безопасным.
	true
}

/// Отображает справочную информацию о доступных командах и параметрах запуска приложения.
pub fn print_help() {
	println!("Mafia Game Generator - Генератор ролей для игры в мафию");
	println!("Версия: {}", env!("CARGO_PKG_VERSION"));
	println!();
	println!("Использование:");
	println!("  MafiaGameGenerator              - Интерактивный режим (классический режим по умолчанию)");
	println!("  MafiaGameGenerator --help       - Показать эту справку");
	println!("  MafiaGameGenerator --version    - Показать версию программы");
	println!("  MafiaGameGenerator --update     - Проверить обновления");
	println!("  MafiaGameGenerator --headless <player_count> <game_mode> <player_names...>");
	println!();
	println!("Параметры headless режима:");
	println!("  player_count  - Количество игроков (6-20)");
	println!("  game_mode     - Режим игры (classic или extended)");
	println!("  player_names  - Имена игроков (через пробел)");
	println!();
	println!("Пример:");
	println!("  MafiaGameGenerator --headless 6 classic \"Игрок1\" \"Игрок2\" \"Игрок3\" \"Игрок4\" \"Игрок5\" \"Игрок6\"");
	println!("  MafiaGameGenerator --headless 8 extended \"Игрок1\" \"Игрок2\" \"Игрок3\" \"Игрок4\" \"Игрок5\" \"Игрок6\" \"Игрок7\" \"Игрок8\"");
}

/// Анализирует аргументы командной строки и возвращает соответствующее действие.
pub fn parse_arguments() -> CliAction {
	let args: Vec<String> = env::args().collect();

	// Проверить флаг помощи.
	if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
		return CliAction::ShowHelp;
	}

	// Проверить флаг версии.
	if args.len() > 1 && args[1] == "--version" {
		return CliAction::ShowVersion;
	}

	// Проверить флаг обновления.
	if args.len() > 1 && args[1] == "--update" {
		return CliAction::CheckUpdate;
	}

	if args.len() > 1 && args[1] == "--headless" {
		// Проверка аргументов.
		if args.len() < 5 {
			return CliAction::Error("Недостаточно аргументов для headless режима. Используйте --help для справки.".to_string());
		}

		let player_count = match args[2].parse::<u8>() {
			Ok(count) => count,
			Err(_) => {
				return CliAction::Error(format!("Неверное количество игроков: {}", args[2]));
			}
		};

		let game_mode = match args[3].as_str() {
			"classic" => crate::game_setup::GameMode::Classic,
			"extended" => crate::game_setup::GameMode::Extended,
			_ => {
				return CliAction::Error(format!("Неверный режим игры: {}. Допустимые значения: classic, extended", args[3]));
			}
		};

		// Имена игроков начинаются с индекса 4
		let player_names: Vec<String> = args[4..].to_vec();

		if player_names.len() != player_count as usize {
			return CliAction::Error(format!("Количество предоставленных имен игроков ({}) не соответствует указанному количеству игроков ({})",
				player_names.len(), player_count));
		}

		return CliAction::RunHeadless {
			player_count,
			game_mode,
			player_names,
		};
	}

	CliAction::RunInteractive
}

/// Перечисление возможных действий CLI
pub enum CliAction {
	ShowHelp,
	ShowVersion,
	CheckUpdate,
	RunHeadless {
		player_count: u8,
		game_mode: crate::game_setup::GameMode,
		player_names: Vec<String>,
	},
	RunInteractive,
	Error(String),
}