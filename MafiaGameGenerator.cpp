#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <fstream>
#include <random>
#include <ctime>
#include <locale>
#include <codecvt>
#include "Role.h"

void assignRoles(int numPlayers, std::vector<Role>& roles);
void createFilesAndPrintRoles(const std::vector<std::wstring>& players, const std::vector<Role>& roles);

int main() {
	// Установить кодировку которая поддерживает кириллицу
	setlocale(LC_ALL, "");

	// Запросить количество игроков
	int numPlayers;
	std::wcout << L"Введите количество игроков: ";
	while (!(std::wcin >> numPlayers) || numPlayers < 4) {
		std::wcin.clear();
		std::wcin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
		std::wcerr << L"Неверный ввод. Количество игроков должно быть числом и не менее 4." << std::endl;
		std::wcout << L"Введите количество игроков: ";
	}

	// Запросить имена игроков
	std::vector<std::wstring> players(numPlayers);
	std::wcout << L"Введите имена игроков:" << std::endl;
	std::wcin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');
	for (int i = 0; i < numPlayers; ++i) {
		std::wcout << L"Игрок " << i + 1 << L": ";
		std::getline(std::wcin, players[i]);
	}

	// Назначить роли
	std::vector<Role> roles(numPlayers, Civilian);
	assignRoles(numPlayers, roles);

	// Создать файлы и вывести роли в консоль
	createFilesAndPrintRoles(players, roles);

	// Вывести сообщение о завершении
	std::wcout << L"Файлы созданы." << std::endl;
	std::wcout << L"Нажмите Enter для выхода...";
	std::wcin.ignore(std::numeric_limits<std::streamsize>::max(), '\n');

	return 0;
}

void assignRoles(int numPlayers, std::vector<Role>& roles) {
	int numMafias = numPlayers / 3;
	int numDetectives = std::max(1, numPlayers / 6);

	for (int i = 0; i < numMafias; ++i) {
		roles[i] = Mafia;
	}
	for (int i = numMafias; i < numMafias + numDetectives; ++i) {
		roles[i] = Detective;
	}

	// Перемешать роли
	std::srand(static_cast<unsigned int>(std::time(0)));
	std::shuffle(roles.begin(), roles.end(), std::mt19937(std::random_device()()));
}

void createFilesAndPrintRoles(const std::vector<std::wstring>& players, const std::vector<Role>& roles) {
	if (players.size() != roles.size()) {
		std::wcerr << L"Ошибка: количество игроков и ролей не совпадает." << std::endl;
		return;
	}

	for (size_t i = 0; i < players.size(); ++i) {
		try {
			std::wofstream outFile(players[i] + L".txt");
			outFile.imbue(std::locale("Russian_Russia.1251"));

			if (!outFile) {
				throw std::ios_base::failure("Не удалось открыть файл.");
			}
			outFile << L"Игрок: " << players[i] << L"\n" << L"Роль: " << roleToString(roles[i]) << std::endl;
			outFile.close();
			std::wcout << L"Игрок: " << players[i] << L"\n" << L"Роль: " << roleToString(roles[i]) << std::endl;
		} catch (const std::exception& e) {
			std::wcerr << L"Исключение: " << e.what() << L" для игрока: " << players[i] << std::endl;
		}
	}
}