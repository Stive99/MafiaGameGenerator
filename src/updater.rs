use std::env;
use std::time::Duration;
use std::process::{Command};
use std::path::Path;
use std::fs::File;
use fs2::FileExt;

const GITHUB_URL: &str = "https://raw.githubusercontent.com/Stive99/MafiaGameGenerator";
const DOWNLOAD_URL: &str = "https://github.com/Stive99/MafiaGameGenerator/releases/download";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36";
const APP_NAME: &str = "MafiaGameGenerator.exe";
const NEW_APP_NAME: &str = "MafiaGameGenerator_new.exe";
const BACKUP_APP_NAME: &str = "MafiaGameGenerator_old.exe";
const LOCK_FILE_NAME: &str = "MafiaGameGenerator.lock";

#[derive(serde::Deserialize, Debug)]
struct CargoToml {
	package: Package,
}

#[derive(serde::Deserialize, Debug)]
struct Package {
	version: String,
}

pub async fn fetch_remote_version() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
	let client = reqwest::Client::builder()
		.timeout(Duration::from_secs(10))
		.build()?;

	let url = format!("{}/main/Cargo.toml", GITHUB_URL);
	// println!("Проверка обновлений по адресу: {}", url);

	let request = client
		.get(&url)
		.header("User-Agent", USER_AGENT)
		.header("Accept", "text/plain")
		.build()?;

	let response = client.execute(request).await?;
	// println!("Статус ответа: {}", response.status());

	// Проверить, успешен ли ответ.
	if response.status().is_success() {
		let cargo_toml_content = response.text().await?;
		// println!("Получено содержимое длиной: {} байт", cargo_toml_content.len());

		// Проверить, что содержимое не пустое.
		if cargo_toml_content.trim().is_empty() {
			return Err("Получено пустое содержимое от сервера".into());
		}

		let cargo_toml = toml::from_str::<CargoToml>(&cargo_toml_content)?;
		Ok(cargo_toml.package.version)
	} else {
		let status = response.status();
		let error_text = response.text().await.unwrap_or_else(|_| "Нет текста ошибки".to_string());
		Err(format!("Ошибка вернула статус: {} - {}", status, error_text).into())
	}
}

pub async fn check_for_update() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	match fetch_remote_version().await {
		Ok(remote_version) => {
			// Используйте правильное сравнение версий.
			if is_newer_version(&remote_version, CURRENT_VERSION) {
				println!("Доступна новая версия: {} (текущая: {})", remote_version, CURRENT_VERSION);

				// Автоматически загрузить и установить обновление.
				match download_update(&remote_version).await {
					Ok(()) => {
						// println!("Обновление успешно загружено!");
						// Применить обновление.
						match apply_update() {
							Ok(true) => {
								println!("Обновление успешно установлено!\nНовая версия будет доступна при следующем запуске!");
								// Запустить новую версию и завершить текущий процесс.
								launch_new_version()?;
								std::process::exit(0);
							}
							Ok(false) => {
								println!("Не удалось установить обновление. Файл сохранен как '{}'", NEW_APP_NAME);
							}
							Err(e) => {
								eprintln!("Ошибка при установке обновления: {}", e);
								// Попробуйте отменить изменения.
								if let Err(rollback_err) = rollback_update() {
									eprintln!("Ошибка при откате обновления: {}", rollback_err);
								}
								return Err(e);
							}
						}
					}
					Err(e) => {
						println!("Не удалось загрузить обновление: {}", e);
						// println!("Вы можете загрузить последнюю версию вручную с:");
						// println!("https://github.com/Stive99/MafiaGameGenerator/releases");
					}
				}
			} else {
				// println!("Установлена последняя версия: {}", CURRENT_VERSION);
			}
			Ok(())
		}
		Err(e) => {
			Err(format!("Не удалось проверить обновления: {}", e).into())
		}
	}
}

// Вспомогательная функция для сравнения версий.
fn is_newer_version(remote: &str, current: &str) -> bool {
	let remote_parts: Vec<&str> = remote.split('.').collect();
	let current_parts: Vec<&str> = current.split('.').collect();

	// Сравнить основные, второстепенные и исправленные версии.
	for (r, c) in remote_parts.iter().zip(current_parts.iter()) {
		match (r.parse::<u32>(), c.parse::<u32>()) {
			(Ok(r_num), Ok(c_num)) => {
				if r_num > c_num {
					return true;
				} else if r_num < c_num {
					return false;
				}
				// Если равны, переходите к следующей части.
			}
			_ => {
				// Если разбор не удался, перейти к сравнению строк.
				return remote > current;
			}
		}
	}

	// Если все детали сравниваются как равные, пульт дистанционного управления не является более новым.
	false
}

pub async fn download_update(remote_version: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	let client = reqwest::Client::builder()
		.timeout(Duration::from_secs(30)) // Увеличить время ожидания при загрузке файла
		.build()?;

	let download_url = format!("{}/{}/{}", DOWNLOAD_URL, remote_version, APP_NAME);
	// println!("Загрузка обновления с адреса: {}", download_url);

	let request = client
		.get(&download_url)
		.header("User-Agent", USER_AGENT)
		.header("Accept", "application/octet-stream")
		.build()?;

	let response = client.execute(request).await?;
	// println!("Статус ответа загрузки: {}", response.status());

	if response.status().is_success() {
		let bytes = response.bytes().await?;
		// println!("Загружено {} байт", bytes.len());

		// Проверить, получили ли мы данные.
		if bytes.is_empty() {
			return Err("Загружен пустой файл обновления".into());
		}

		// Сохраните файл с суффиксом _new.
		std::fs::write(NEW_APP_NAME, &bytes)?;
		// println!("Обновление успешно загружено как '{}'", NEW_APP_NAME);

		Ok(())
	} else {
		let status = response.status();
		// Не пытайтесь прочитать текст ответа, если это 404, так как это может вызвать другую ошибку.
		if status == reqwest::StatusCode::NOT_FOUND {
			Err(format!("Файл обновления не найден на сервере ({}). Возможно, релиз еще не создан.", status).into())
		} else {
			let error_text = response.text().await.unwrap_or_else(|_| "Нет текста ошибки".to_string());
			Err(format!("Ошибка вернула статус: {} - {}", status, error_text).into())
		}
	}
}

// Функция для применения обновления путем атомарного переименования файлов.
fn apply_update() -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
	let current_exe = std::env::current_exe()?;
	let current_dir = current_exe.parent().ok_or("Не удалось определить директорию приложения")?;

	// println!("Установка обновления...");

	// Проверить, существует ли файл новой версии.
	let new_file_path = format!("{}/{}", current_dir.to_string_lossy(), NEW_APP_NAME);
	if !Path::new(&new_file_path).exists() {
		return Err(format!("Файл обновления '{}' не найден", NEW_APP_NAME).into());
	}

	// Создать резервную копию текущей версии атомарно.
	let current_file_path = format!("{}/{}", current_dir.to_string_lossy(), APP_NAME);
	let backup_path = format!("{}/{}", current_dir.to_string_lossy(), BACKUP_APP_NAME);

	if Path::new(&current_file_path).exists() {
		// println!("Создание резервной копии текущей версии...");
		atomic_copy(&current_file_path, &backup_path)?;
	}

	// Атомарно заменить текущее приложение новой версией.
	// println!("Установка новой версии...");
	atomic_replace(&new_file_path, &current_file_path)?;

	Ok(true)
}

// Функция для выполнения атомарного копирования файла.
fn atomic_copy(from: &str, to: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	// Создать временное имя файла.
	let temp_path = format!("{}.tmp", to);

	// Копировать во временный файл.
	std::fs::copy(from, &temp_path)?;

	// Атомарно переименовать временный файл в целевой.
	std::fs::rename(&temp_path, to)?;

	Ok(())
}

// Функция для выполнения атомарной замены файла.
fn atomic_replace(from: &str, to: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	// Создать временное имя файла.
	let temp_path = format!("{}.tmp", to);

	// Копировать новый файл во временное местоположение.
	std::fs::copy(from, &temp_path)?;

	// Атомарно переименовать временный файл в целевой, заменив исходный.
	std::fs::rename(&temp_path, to)?;

	// Удалить исходный файл (теперь замененный)
	std::fs::remove_file(from)?;

	Ok(())
}

// Функция запуска новой версии.
fn launch_new_version() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	let current_exe = std::env::current_exe()?;
	let current_dir = current_exe.parent().ok_or("Не удалось определить директорию приложения")?;

	// Запустить новую версию с флагом, указывающим, что это перезапуск после обновления.
	let _new_process = Command::new(format!("{}/{}", current_dir.to_string_lossy(), APP_NAME))
		.arg("--updated")
		.spawn()?;

	println!("Новая версия запущена. Закрытие текущего процесса...");
	Ok(())
}

// Функция очистки для удаления резервной копии после успешного обновления.
pub fn cleanup_old_version() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	// Проверьте, было ли приложение запущено с флагом --updated.
	let args: Vec<String> = std::env::args().collect();
	if args.len() > 1 && args[1] == "--updated" && Path::new(BACKUP_APP_NAME).exists() {
		// println!("Удаление резервной копии старой версии...");
		std::fs::remove_file(BACKUP_APP_NAME)?;
	}

	// Очистить файл блокировки, если он существует.
	if Path::new(LOCK_FILE_NAME).exists() {
		let _ = std::fs::remove_file(LOCK_FILE_NAME);
	}

	Ok(())
}

// Функция отката в случае сбоя обновления.
pub fn rollback_update() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
	let current_exe = std::env::current_exe()?;
	let current_dir = current_exe.parent().ok_or("Не удалось определить директорию приложения")?;

	let backup_path = format!("{}/{}", current_dir.to_string_lossy(), BACKUP_APP_NAME);
	let current_path = format!("{}/{}", current_dir.to_string_lossy(), APP_NAME);

	if Path::new(&backup_path).exists() && Path::new(&current_path).exists() {
		println!("Откат обновления...");
		// Удалить неудачную новую версию.
		std::fs::remove_file(&current_path)?;
		// Восстановить резервную копию.
		std::fs::rename(&backup_path, &current_path)?;
		println!("Откат успешно выполнен. Приложение восстановлено до предыдущей версии.");
	}
	Ok(())
}

// Функция для проверки, запущен ли другой экземпляр.
pub fn is_another_instance_running() -> bool {
	let lock_file_path = LOCK_FILE_NAME;

	// Попробуйте создать или открыть файл блокировки.
	match File::create(lock_file_path) {
		Ok(file) => {
			// Попробуйте заблокировать файл в эксклюзивном режиме.
			match file.try_lock_exclusive() {
				Ok(_) => {
					// Успешно заблокировано, это первый случай.
					true
				}
				Err(_) => {
					// Другой пример — запуск и удержание блокировки.
					false
				}
			}
		}
		Err(_) => {
			// Невозможно получить доступ к файлу блокировки, предположительно работает другой экземпляр.
			false
		}
	}
}