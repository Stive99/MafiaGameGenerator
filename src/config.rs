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