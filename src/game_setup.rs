/// Перечисление возможных режимов игры.
/// `Copy` и `Clone` позволят нам легко передавать этот небольшой тип.
#[derive(Debug, Copy, Clone)]
pub enum GameMode {
	Classic,  // Классический режим без доп. ролей
	Extended, // Расширенный режим с Маньяком
}

/// Структура для хранения всех настроек текущей игровой сессии.
#[derive(Debug)]
pub struct GameConfig {
	pub player_count: u8,
	pub game_mode: GameMode,
}

use crate::error::AppError;
use crate::role::{Role};
use rand::seq::SliceRandom;

const MIN_PLAYERS: u8 = 6;
const MAX_PLAYERS: u8 = 20;

pub fn shuffle_roles(roles: &mut [Role]) {
	let mut rng = rand::rng();
	roles.shuffle(&mut rng);
}

/**
 * Динамически определяет и возвращает набор ролей для заданного количества игроков.
 */
pub fn get_roles_for_players(config: &GameConfig) -> Result<Vec<Role>, AppError> {
	// Проверяем количество игроков на валидность
	if !(MIN_PLAYERS..=MAX_PLAYERS).contains(&config.player_count) {
		return Err(AppError::InvalidPlayerCount {
			given: config.player_count,
			min: MIN_PLAYERS,
			max: MAX_PLAYERS,
		});
	}

	// Используем более эффективный подход для расчета ролей
	let role_counts = Role::get_role_counts(config.player_count, config.game_mode);

	// Создаем вектор ролей из подсчитанных значений
	let roles = role_counts.to_vec();

	Ok(roles)
}

pub fn run_headless_mode(
	player_count: u8,
	game_mode: GameMode,
	player_names: Vec<String>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	use crate::io_handler::write_role_files;
	use crate::role::Role;

	// Создаем конфигурацию
	let config = GameConfig { player_count, game_mode };

	// Валидируем количество
	let mut roles = match get_roles_for_players(&config) {
		Ok(role_set) => role_set,
		Err(e) => {
			return Err(format!("Ошибка конфигурации: {}", e).into());
		}
	};

	println!("\nГенерирую {} ролей для ваших игроков...", roles.len());

	// Перемешиваем роли
	shuffle_roles(&mut roles);

	// Соединяем имена с ролями
	let players_with_roles: Vec<(String, Role)> = player_names.into_iter().zip(roles).collect();

	// Записываем файлы
	match write_role_files(&players_with_roles) {
		Ok(()) => {
			println!("\nУспех! Роли сгенерированы и сохранены в папке 'roles'.");
			println!("Для каждого игрока создан персональный файл. Количество игроков: {}", players_with_roles.len());
		}
		Err(e) => {
			return Err(format!("Ошибка при записи файлов: {}", e).into());
		}
	}

	// Добавляем небольшую задержку, чтобы убедиться, что файлы записались
	std::thread::sleep(std::time::Duration::from_millis(100));

	Ok(())
}

pub fn run_interactive_mode(default_game_mode: GameMode) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	use crate::io_handler::{prompt_for_player_count, prompt_for_player_names, write_role_files};
	use crate::role::Role;

	loop {
		// Получаем количество игроков
		let player_count = match prompt_for_player_count() {
			Ok(count) => count,
			Err(e) => {
				// Если ошибка ввода, печатаем ее и начинаем цикл заново
				eprintln!("Ошибка: {e}. Пожалуйста, попробуйте еще раз.\n");
				continue; // Переходим к следующей итерации цикла
			}
		};

		// Создаем конфигурацию с фиксированным режимом игры
		let config = GameConfig { player_count, game_mode: default_game_mode };

		// Валидируем количество
		let mut roles = match get_roles_for_players(&config) {
			Ok(role_set) => role_set,
			Err(e) => {
				// Если количество не подходит, печатаем ошибку и начинаем заново
				eprintln!("Ошибка: {e}. Пожалуйста, попробуйте еще раз.\n");
				continue;
			}
		};

		// Если все хорошо, получаем имена
		let names = prompt_for_player_names(config.player_count)?;

		println!("\nГенерирую {} ролей для ваших игроков...", roles.len());

		// Перемешиваем роли
		shuffle_roles(&mut roles);

		// Соединяем имена с ролями
		let players_with_roles: Vec<(String, Role)> = names.into_iter().zip(roles).collect();

		// Записываем файлы
		match write_role_files(&players_with_roles) {
			Ok(()) => {
				println!("\nУспех! Роли сгенерированы и сохранены в папке 'roles'.");
				println!("Для каждого игрока создан персональный файл. Количество игроков: {}", players_with_roles.len());
			}
			Err(e) => {
				eprintln!("Ошибка при записи файлов: {}", e);
				eprintln!("Пожалуйста, проверьте права доступа к папке и попробуйте снова.");
				continue;
			}
		}

		// --- Все прошло успешно, выходим из цикла и завершаем программу ---
		println!("\nНажмите Enter для выхода...");
		let mut buffer = String::new();
		std::io::stdin().read_line(&mut buffer).unwrap_or_default(); // Ожидаем нажатия Enter

		// Успешный выход из функции и программы
		return Ok(());
	}
}