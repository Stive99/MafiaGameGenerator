#ifndef ROLE_H
#define ROLE_H

#include <string>

// Перечисление для ролей
enum Role { Mafia, Civilian, Detective, Doctor };

// Функция для преобразования роли в строку
std::wstring roleToString(Role role);

#endif // ROLE_H