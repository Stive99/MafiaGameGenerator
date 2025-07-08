use crate::error::AppError;
use crate::role::Role;
use crate::config::GameConfig;

const MIN_PLAYERS: u8 = 6;
const MAX_PLAYERS: u8 = 20;

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

	let mut roles = Vec::with_capacity(config.player_count as usize);

	// Рассчитываем количество ключевых ролей по формулам
	// Мафия составляет примерно треть от всех игроков.
	let num_mafia_total = (config.player_count as f32 / 3.0).floor() as u8;
	// Если мафиози больше одного, один из них становится Доном.
	let num_don = if num_mafia_total > 1 { 1 } else { 0 };
	let num_mafia = num_mafia_total - num_don;
	// В игре всегда есть один Шериф и один Доктор (по нашей логике).
	let num_sheriff = 1;
	let num_doctor = 1;

	// Добавляем Маньяка только в расширенном режиме и если игроков достаточно (например, 8+)
	let num_maniac = match config.game_mode {
		crate::config::GameMode::Classic => 0,
		crate::config::GameMode::Extended if config.player_count >= 8 => 1,
		_ => 0, // В остальных случаях (например, в расширенном режиме, но < 8 игроков) маньяка нет
	};

	// Считаем, сколько осталось мирных жителей
	let active_roles_count = num_mafia_total + num_sheriff + num_doctor + num_maniac;
	let num_civilians = config.player_count - active_roles_count;

	// Добавляем рассчитанное количество каждой роли
	for _ in 0..num_mafia { roles.push(Role::Mafia); }
	for _ in 0..num_don { roles.push(Role::Don); }
	for _ in 0..num_sheriff { roles.push(Role::Sheriff); }
	for _ in 0..num_doctor { roles.push(Role::Doctor); }
	for _ in 0..num_civilians { roles.push(Role::Civilian); }
	for _ in 0..num_maniac { roles.push(Role::Maniac); }

	Ok(roles)
}