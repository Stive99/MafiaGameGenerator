mod role;
mod config;
mod game_setup;
mod generator;
mod io_handler;
mod error;
mod updater;

use game_setup::get_roles_for_players;
use generator::shuffle_roles;
use io_handler::{prompt_for_game_mode, prompt_for_player_count, write_role_files, prompt_for_player_names};
use crate::error::AppError;
use crate::role::Role;
use std::error::Error;
use crate::config::{GameConfig};

fn main() {
	println!("--- {} ---", env!("CARGO_PKG_NAME"));

	// Проверка обновлений при запуске
    // Мы не хотим, чтобы сбой обновления прерывал работу программы,
    // поэтому просто выводим ошибку в консоль, если она есть.
    if let Err(e) = updater::check_for_updates() {
        eprintln!("Не удалось проверить обновления: {e}");
    }

	// Проверяем аргументы командной строки
	let args: Vec<String> = std::env::args().collect();
	if args.len() > 1 && args[1] == "--update" {
		// Если запуск с флагом --update, запускаем только обновление
		if let Err(e) = updater::check_for_updates() {
			eprintln!("Критическая ошибка в модуле обновления: {e}");
		}
		// Завершаем программу после попытки обновления
		return;
	}

	// Вызываем нашу основную логику и обрабатываем единый тип ошибки
	if let Err(e) = run_app() {
		// Теперь мы просто выводим ошибку. Форматирование (`Ошибка ввода: ...`)
		// уже встроено в реализацию `Display` для `AppError`.
		eprintln!("\nКритическая ошибка: {e}");
		// В случае ошибки, мы можем "раскрутить" цепочку, если она есть
		if let Some(source) = e.source() {
			eprintln!("  Источник: {source}");
		}
	}
}

// Мы вынесли всю логику в отдельную функцию, которая может возвращать Result.
// Это стандартная практика в Rust для приложений, которые могут завершиться с ошибкой.
fn run_app() -> Result<(), AppError> {
	loop {
		// Получаем режим игры
		let game_mode = match prompt_for_game_mode() {
			Ok(mode) => mode,
			Err(e) => {
				eprintln!("{e}\nПожалуйста, попробуйте еще раз.\n");
				continue;
			}
		};

		// Получаем количество игроков
		let player_count = match prompt_for_player_count() {
			Ok(count) => count,
			Err(e) => {
				// Если ошибка ввода, печатаем ее и начинаем цикл заново
				eprintln!("Ошибка: {e}. Пожалуйста, попробуйте еще раз.\n");
				continue; // Переходим к следующей итерации цикла
			}
		};

		// Создаем конфигурацию
		let config = GameConfig { player_count, game_mode };

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
		write_role_files(&players_with_roles)?;

		println!("\nУспех! Роли сгенерированы и сохранены в папке 'roles'.");
		println!("Для каждого игрока создан персональный файл. Количество игроков: {}", players_with_roles.len());

		// --- Все прошло успешно, выходим из цикла и завершаем программу ---
		println!("\nНажмите Enter для выхода...");
		let mut buffer = String::new();
		std::io::stdin().read_line(&mut buffer).unwrap_or_default(); // Ожидаем нажатия Enter

		// Успешный выход из функции и программы
		return Ok(());
	}
}