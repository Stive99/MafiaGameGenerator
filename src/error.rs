use std::fmt;
use std::num::ParseIntError;
use std::env::VarError;

#[derive(Debug)]
pub enum AppError {
	/// Ошибка ввода/вывода. Оборачивает стандартную ошибку `std::io::Error`.
	Io(std::io::Error),

	/// Ошибка парсинга строки в число. Оборачивает стандартную `ParseIntError`.
	ParseInt(ParseIntError),

	/// Некорректное количество игроков. Хранит информацию о том, какое
	/// значение было введено, и какие являются допустимыми.
	InvalidPlayerCount { given: u8, min: u8, max: u8 },

	/// Введено пустое имя игрока.
	EmptyPlayerName,

	/// Имя игрока содержит недопустимые для имени файла символы.
	InvalidCharactersInName(String),

	/// Имя игрока уже используется.
	DuplicatePlayerName(String),

	/// Ошибка при обновлении конфигурации приложения.
	UpdateConfig(String),

	/// Недопустимое имя файла (возможная попытка path traversal).
	InvalidFileName(String),
}

impl fmt::Display for AppError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			AppError::Io(err) => write!(f, "Ошибка ввода/вывода: {err}"),
			AppError::ParseInt(_) => write!(f, "Ошибка ввода: ожидалось целое число."),
			AppError::InvalidPlayerCount { given, min, max } => write!(
				f,
				"Ошибка конфигурации: для игры требуется от {min} до {max} игроков. Вы введи: {given}."
			),
			AppError::EmptyPlayerName => write!(f, "Ошибка ввода: имя игрока не может быть пустым."),
			AppError::InvalidCharactersInName(name) => write!(
				f,
				"Ошибка ввода: имя '{name}' содержит недопустимые символы (/, \\, :, и т.д.).",
			),
			AppError::DuplicatePlayerName(name) => write!(
				f,
				"Ошибка ввода: игрок с именем '{name}' уже существует.",
			),
			AppError::UpdateConfig(msg) => {
				write!(f, "Ошибка конфигурации обновления: {msg}")
			},
			AppError::InvalidFileName(name) => {
				write!(f, "Ошибка безопасности: имя файла '{name}' недопустимо.")
			}
		}
	}
}

/// Реализация трейта `Error`, чтобы наш тип был полноценной ошибкой.
impl std::error::Error for AppError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			AppError::Io(err) => Some(err),
			AppError::ParseInt(err) => Some(err),
			_ => None,
		}
	}
}

impl From<std::io::Error> for AppError {
	fn from(err: std::io::Error) -> Self {
		AppError::Io(err)
	}
}

impl From<ParseIntError> for AppError {
	fn from(err: ParseIntError) -> Self {
		AppError::ParseInt(err)
	}
}

impl From<VarError> for AppError {
	fn from(err: VarError) -> Self {
		match err {
			VarError::NotPresent => AppError::UpdateConfig("Переменная окружения не найдена.".to_string()),
			VarError::NotUnicode(_) => AppError::UpdateConfig("Переменная окружения содержит недопустимые символы.".to_string()),
		}
	}
}