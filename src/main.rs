mod role;
mod game_setup;
mod io_handler;
mod error;
mod updater;

use std::{env};

#[tokio::main]
async fn main() {
	println!("--- {} ---", env!("CARGO_PKG_NAME"));

	// CПроверить, запущен ли другой экземпляр.
	if !updater::is_another_instance_running() {
		eprintln!("Обнаружен запущенный экземпляр приложения. Завершение работы.");
		return;
	}

	// Удалить старую версию, если она существует (от предыдущего обновления)
	if let Err(e) = updater::cleanup_old_version() {
		eprintln!("Предупреждение: Не удалось очистить старую версию: {}", e);
	}

	// Анализ аргументов командной строки.
	match io_handler::parse_arguments() {
		io_handler::CliAction::ShowHelp => {
			// Вывод информации о программе.
			io_handler::print_help();
			return;
		}
		io_handler::CliAction::ShowVersion => {
			// Вывод версии программы.
			println!("MafiaGameGenerator v{}", env!("CARGO_PKG_VERSION"));
			return;
		}
		io_handler::CliAction::CheckUpdate => {
			match updater::check_for_update().await {
				Ok(()) => println!("Проверка обновлений завершена."),
				Err(e) => eprintln!("Ошибка при проверке обновлений: {}", e),
			}
			return;
		}
		io_handler::CliAction::RunHeadless { player_count, game_mode, player_names } => {
			if let Err(e) = game_setup::run_headless_mode(player_count, game_mode, player_names) {
				eprintln!("\nКритическая ошибка: {e}");
				if let Some(source) = e.source() {
					eprintln!("  Источник: {source}");
				}
			}
			return;
		}
		io_handler::CliAction::RunInteractive => {
			match updater::check_for_update().await {
				Ok(()) => {}
				Err(e) => {
					eprintln!("Ошибка при проверке обновлений: {}", e);
				}
			}

			if let Err(e) = game_setup::run_interactive_mode(game_setup::GameMode::Classic) {
				eprintln!("\nКритическая ошибка: {e}");
				if let Some(source) = e.source() {
					eprintln!("  Источник: {source}");
				}
			}
			return;
		}
		io_handler::CliAction::Error(msg) => {
			eprintln!("{}", msg);
			return;
		}
	}
}