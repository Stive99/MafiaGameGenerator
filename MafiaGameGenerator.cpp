#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <fstream>
#include <random>
#include <ctime>
#include <locale>
#include <codecvt>

#ifdef _WIN32
#include <windows.h>
#endif

enum Role { Mafia, Civilian, Detective, Doctor };

// Функция для преобразования роли в строку
std::wstring roleToString(Role role) {
	switch (role) {
		case Mafia: return L"Мафия";
		case Civilian: return L"Мирный";
		case Detective: return L"Детектив";
		case Doctor: return L"Доктор";
	}
	return L"";
}

void assignRoles(int numPlayers, std::vector<Role>& roles) {
	int numMafias = numPlayers / 3;
	int numDetectives = 1;
	int numDoctors = 1;

	for (int i = 0; i < numMafias; ++i) {
		roles[i] = Mafia;
	}
	for (int i = numMafias; i < numMafias + numDetectives; ++i) {
		roles[i] = Detective;
	}
	for (int i = numMafias + numDetectives; i < numMafias + numDetectives + numDoctors; ++i) {
		roles[i] = Doctor;
	}

	// Перемешать роли
	std::random_device rd;
	std::mt19937 g(rd());
	std::shuffle(roles.begin(), roles.end(), g);
}

void createFilesAndPrintRoles(const std::vector<std::wstring>& players, const std::vector<Role>& roles) {
	for (size_t i = 0; i < players.size(); ++i) {
		try {
			std::wstring fileName = players[i] + L".txt";
			std::wofstream outFile(fileName);
			if (!outFile) {
				throw std::ios_base::failure("Ошибка открытия файла");
			}
			// Установить локаль на UTF-8
			outFile.imbue(std::locale(std::locale(), new std::codecvt_utf8<wchar_t>));
			// Записать данные в файл
			outFile << L"Игрок: " << players[i] << L"\n" << L"Роль: " << roleToString(roles[i]) << std::endl;
			outFile.close();
			// Вывести данные в консоль
			std::wcout << L"Игрок: " << players[i] << L"\n" << L"Роль: " << roleToString(roles[i]) << std::endl;
		} catch (const std::exception& e) {
			std::wcerr << L"Исключение: " << e.what() << L" для игрока: " << players[i] << std::endl;
			std::wcout << L"Произошла ошибка при создании файла для игрока: " << players[i] << std::endl;
			std::wcout << L"Продолжить выполнение программы? (y/n): ";
			wchar_t response;
			std::wcin >> response;
			if (response != L'y' && response != L'Y') {
				std::wcout << L"Программа завершена." << std::endl;
				return;
			}
		}
	}
}

int main() {
	// Установить кодировку, которая поддерживает кириллицу
	std::setlocale(LC_ALL, "");
#ifdef _WIN32
	SetConsoleOutputCP(CP_UTF8);
#endif

	// Запросить количество игроков
	int numPlayers;
	std::wcout << L"Введите количество игроков: ";
	while (!(std::wcin >> numPlayers) || numPlayers < 4) {
		std::wcin.clear();
		std::wcin.ignore(INT_MAX, '\n');
		std::wcerr << L"Неверный ввод. Количество игроков должно быть числом и не менее 4." << std::endl;
		std::wcout << L"Введите количество игроков: ";
	}

	// Запросить имена игроков
	std::vector<std::wstring> players(numPlayers);
	std::wcout << L"Введите имена игроков:\n";
	std::wcin.ignore(INT_MAX, '\n');
	for (int i = 0; i < numPlayers; ++i) {
		std::wcout << L"Игрок " << i + 1 << L": ";
		std::getline(std::wcin, players[i]);
	}

	std::wcout << L"\n--- Назначение ролей ---\n";
	std::cout << std::endl;

	// Назначить роли
	std::vector<Role> roles(numPlayers, Civilian);
	assignRoles(numPlayers, roles);

	// Создать файлы и записать роли
	createFilesAndPrintRoles(players, roles);

	std::wcout << L"\n---------------\n";
	std::cout << std::endl;

	// Вывести сообщение о завершении
	std::wcout << L"Файлы созданы." << std::endl;
	std::cout << std::endl;
	std::wcout << L"Нажмите Enter для выхода...";
	std::wcin.ignore(INT_MAX, '\n');
	std::wcin.get(); // Ожидать нажатия Enter

	return 0;
}