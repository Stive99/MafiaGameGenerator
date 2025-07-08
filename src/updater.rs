use crate::error::AppError;
use self_update::cargo_crate_version;
use self_update::Status;

/// Извлекает имя пользователя и имя репозитория из URL в Cargo.toml.
fn get_repo_info() -> Result<(String, String), AppError> {
	// Получаем URL репозитория из Cargo.toml во время компиляции
	let repo_url = env!("CARGO_PKG_REPOSITORY");

	// Простая проверка, что URL похож на GitHub URL
	if !repo_url.starts_with("https://github.com/") {
		return Err(AppError::UpdateConfig(
			"URL репозитория в Cargo.toml должен быть полным URL-адресом GitHub.".to_string(),
		));
	}

	// Отделяем часть после "https://github.com/"
	let path = repo_url.trim_start_matches("https://github.com/");

	// Разделяем путь на части по символу '/'
	let mut parts = path.split('/');

	// Извлекаем имя пользователя и имя репозитория
	let owner = parts.next().ok_or_else(|| {
		AppError::UpdateConfig("Не удалось извлечь имя пользователя из URL репозитория.".to_string())
	})?;

	let repo_name = parts.next().ok_or_else(|| {
		AppError::UpdateConfig("Не удалось извлечь имя репозитория из URL.".to_string())
	})?;

	Ok((owner.to_string(), repo_name.to_string()))
}

/// Проверяет наличие обновлений и выполняет их.
pub fn check_for_updates() -> Result<(), AppError> {
	// println!("Проверка обновлений...");

	let (repo_owner, repo_name) = get_repo_info()?;

	// Создаем конфигурацию для обновления, используя метаданные из Cargo.toml
	let updater = self_update::backends::github::Update::configure()
		.repo_owner(&repo_owner)
		.repo_name(&repo_name)
		.bin_name(env!("CARGO_PKG_NAME"))
		.current_version(cargo_crate_version!())
		.show_output(false)
		.build()?;

	// Запускаем проверку и обновление
	match updater.update() {
		Ok(status) => {
			match status {
				Status::Updated(new_version) => {
					println!("Приложение успешно обновлено до версии {new_version}!");
					println!("Пожалуйста, перезапустите приложение, чтобы использовать новую версию.");
				}
				Status::UpToDate(_) => {
					// println!("Ваша версия приложения является самой новой.");
				}
			}
		}
		Err(e) => {
			eprintln!("Ошибка при обновлении: {e}");
			eprintln!("Пожалуйста, попробуйте снова или обновитесь вручную со страницы GitHub.");
			// Мы не возвращаем ошибку, чтобы не завершать программу, если обновление не удалось
		}
	}

	Ok(())
}

// Добавляем преобразование ошибки из `self_update` в нашу `AppError`
// для использования оператора `?`.
impl From<self_update::errors::Error> for AppError {
	fn from(e: self_update::errors::Error) -> Self {
		AppError::Io(std::io::Error::other(e))
	}
}