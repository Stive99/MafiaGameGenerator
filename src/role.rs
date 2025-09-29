#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Role {
	Civilian, // Мирный житель
	Mafia,    // Мафия
	Don,      // Мафия (Дон)
	Sheriff,  // Шериф
	Doctor,   // Доктор
	Maniac,   // Маньяк
}

// Здесь мы определяем методы, которые будут доступны для любого значения типа Role.
impl Role {
	/// &self - означает, что метод заимствует экземпляр, но не изменяет его.
	/// &'static str - возвращает строковый срез, который "вшит" в программу и существует всё время её жизни.
	/// Это очень эффективно по памяти и скорости.
	pub fn get_name(&self) -> &'static str {
		match self {
			Role::Civilian => "Мирный житель",
			Role::Mafia => "Мафия",
			Role::Don => "Мафия (Дон)",
			Role::Sheriff => "Шериф",
			Role::Doctor => "Доктор",
			Role::Maniac => "Маньяк",
		}
	}

	/// Возвращает описание роли и её целей в игре.
	pub fn get_description(&self) -> &'static str {
		match self {
			Role::Civilian => "Ваша цель - выжить и вычислить всех членов мафии. Вы побеждаете, когда в городе не остается мафии.",
			Role::Mafia => "Вы - член мафии. Ночью вы просыпаетесь вместе с другими мафиози и выбираете жертву. Ваша цель - добиться численного равенства с мирными жителями.",
			Role::Don => "Вы - глава мафии. Ночью вы принимаете окончательное решение по выбору жертвы. Также ночью вы можете проверить одного из игроков, чтобы узнать, является ли он Шерифом.",
			Role::Sheriff => "Вы - Шериф. Ночью вы можете проверить одного из игроков, чтобы узнать, принадлежит ли он к мафии. Ваша цель - помочь мирным жителям найти и казнить мафию.",
			Role::Doctor => "Вы - Доктор. Ночью вы можете 'вылечить' одного игрока, спасая его от выстрела мафии. Вы не можете лечить одного и того же игрока две ночи подряд (по классическим правилам).",
			Role::Maniac => "Вы играете сами за себя. Каждую ночь вы просыпаетесь и выбираете, кого убить. Ваша цель — остаться последним выжившим в городе.",
		}
	}

	/// Возвращает количество каждого типа роли для заданного количества игроков и режима игры.
	/// Это эффективный с точки зрения памяти способ вычисления ролей без создания векторов.
	pub fn get_role_counts(player_count: u8, game_mode: crate::game_setup::GameMode) -> RoleCounts {
		// Рассчитываем количество ключевых ролей по формулам
		// Мафия составляет примерно треть от всех игроков.
		let num_mafia_total = (player_count as f32 / 3.0).floor() as u8;
		// Если мафиози больше одного, один из них становится Доном.
		let num_don = if num_mafia_total > 1 { 1 } else { 0 };
		let num_mafia = num_mafia_total - num_don;
		// В игре всегда есть один Шериф и один Доктор (по нашей логике).
		let num_sheriff = 1;
		let num_doctor = 1;

		// Добавляем Маньяка только в расширенном режиме и если игроков достаточно (например, 8+)
		let num_maniac = match game_mode {
			crate::game_setup::GameMode::Classic => 0,
			crate::game_setup::GameMode::Extended if player_count >= 8 => 1,
			_ => 0, // В остальных случаях (например, в расширенном режиме, но < 8 игроков) маньяка нет
		};

		// Считаем, сколько осталось мирных жителей
		let active_roles_count = num_mafia_total + num_sheriff + num_doctor + num_maniac;
		let num_civilians = player_count - active_roles_count;

		RoleCounts {
			civilians: num_civilians,
			mafia: num_mafia,
			don: num_don,
			sheriff: num_sheriff,
			doctor: num_doctor,
			maniac: num_maniac,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct RoleCounts {
	pub civilians: u8,
	pub mafia: u8,
	pub don: u8,
	pub sheriff: u8,
	pub doctor: u8,
	pub maniac: u8,
}

impl RoleCounts {
	/// Возвращает общее количество ролей.
	pub fn total(&self) -> u8 {
		self.civilians + self.mafia + self.don + self.sheriff + self.doctor + self.maniac
	}

	/// Создает вектор ролей из подсчетов.
	pub fn to_vec(self) -> Vec<Role> {
		let mut roles = Vec::with_capacity(self.total() as usize);

		// Добавляем рассчитанное количество каждой роли
		for _ in 0..self.mafia { roles.push(Role::Mafia); }
		for _ in 0..self.don { roles.push(Role::Don); }
		for _ in 0..self.sheriff { roles.push(Role::Sheriff); }
		for _ in 0..self.doctor { roles.push(Role::Doctor); }
		for _ in 0..self.civilians { roles.push(Role::Civilian); }
		for _ in 0..self.maniac { roles.push(Role::Maniac); }

		roles
	}
}