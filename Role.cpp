#include "Role.h"

// Реализация функции для преобразования роли в строку
std::wstring roleToString(Role role) {
	switch (role) {
		case Mafia: return L"Мафия";
		case Civilian: return L"Мирный";
		case Detective: return L"Детектив";
		case Doctor: return L"Доктор";
	}
	return L"";
}