#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <fstream>
#include <random>
#include <ctime>
#include <locale>
#include <codecvt>
#include <climits>
#include <stdexcept>
#include <filesystem>

#ifdef _WIN32
    #include <windows.h>
    #include <fcntl.h>
    #include <io.h>
#endif

using namespace std;

enum Role { Mafia, Civilian, Detective, Doctor, Don };

// Функция для преобразования роли в строку
static std::wstring roleToString(Role role) {
    switch (role) {
        case Mafia: return L"Мафия";
        case Civilian: return L"Мирный";
        case Detective: return L"Детектив";
        case Doctor: return L"Доктор";
        case Don: return L"Мафия (Дон)";
    }
    return L"";
}

// Функция для назначения ролей
static void assignRoles(int numPlayers, std::vector<Role>& roles) {
    int numMafias = numPlayers / 3;
    // std::max(1, numPlayers / 6); | (std::max)(1, numPlayers / 6);
    int numDetectives = 1;
    int numDoctors = 1;
    int numDons = (numMafias > 1) ? 1 : 0;

    for (int i = 0; i < numMafias - numDons; ++i) {
        roles[i] = Mafia;
    }
    if (numDons == 1) {
        roles[numMafias - 1] = Don;
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

// Функция для создания файлов и вывода ролей
static void createFilesAndPrintRoles(const std::vector<std::wstring>& players, const std::vector<Role>& roles) {
#ifdef _WIN32
    if (_setmode(_fileno(stdin), _O_WTEXT) == -1) {
        wcerr << L"Не удалось установить режим для stdin" << endl;
        return;
    }
    if (_setmode(_fileno(stdout), _O_WTEXT) == -1) {
        wcerr << L"Не удалось установить режим для stdout" << endl;
        return;
    }
    if (_setmode(_fileno(stderr), _O_WTEXT) == -1) {
        wcerr << L"Не удалось установить режим для stderr" << endl;
        return;
    }
    if (_setmode(_fileno(stdin), _O_U16TEXT) == -1) {
        wcerr << L"Не удалось установить режим для stdin" << endl;
        return;
    }
    if (_setmode(_fileno(stdout), _O_U16TEXT) == -1) {
        wcerr << L"Не удалось установить режим для stdout" << endl;
        return;
    }
    if (_setmode(_fileno(stderr), _O_U16TEXT) == -1) {
        wcerr << L"Не удалось установить режим для stderr" << endl;
        return;
    }
#endif

    //std::locale loc(std::locale(), new std::codecvt_utf8_utf16<wchar_t>);
    std::locale loc("en_US.UTF-8");
    //std::wcout.imbue(loc);
    //std::wcin.imbue(loc);
    //std::wcerr.imbue(loc);

    for (size_t i = 0; i < players.size(); ++i) {
        try {
            std::wstring fileName = players[i] + L".txt";
            std::wofstream outFile(fileName);
            //outFile.imbue(loc);
            //outFile.imbue(std::locale(std::locale(), new std::codecvt_utf8<wchar_t, 0x10ffff, std::consume_header>));
            if (!outFile) {
                throw std::ios_base::failure("Ошибка открытия файла");
            }
            outFile << L"Игрок: " << players[i] << L"\n" << L"Роль: " << roleToString(roles[i]) << std::endl;
            outFile.close();
            std::wcout << L"Игрок: " << players[i] << L"\n" << L"Роль: " << roleToString(roles[i]) << std::endl;
        }
        catch (const std::exception& e) {
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

// Функция для центрирования окна консоли
static void centerConsoleWindow() {
    // Получаем дескриптор окна консоли
    HWND console = GetConsoleWindow();

    // Получаем текущие размеры окна консоли
    RECT r;
    GetWindowRect(console, &r);

    // Получаем размеры экрана
    int screenWidth = GetSystemMetrics(SM_CXSCREEN);
    int screenHeight = GetSystemMetrics(SM_CYSCREEN);

    // Вычисляем новые координаты для центрирования окна консоли
    int newPosX = (screenWidth - (r.right - r.left)) / 2;
    int newPosY = (screenHeight - (r.bottom - r.top)) / 2;

    // Перемещаем окно консоли в центр экрана
    MoveWindow(console, newPosX, newPosY, r.right - r.left, r.bottom - r.top, TRUE);
}

int main() {
    try {
        // Установка локали для поддержки кириллицы
        setlocale(LC_ALL, "");

#ifdef _WIN32
        //_setmode(_fileno(stdin), _O_U16TEXT);
        //_setmode(_fileno(stdout), _O_U16TEXT);
        //_setmode(_fileno(stderr), _O_U16TEXT);

        if (_setmode(_fileno(stdin), _O_WTEXT) == -1) {
            wcerr << L"Не удалось установить режим для stdin" << endl;
            return 1;
        }
        if (_setmode(_fileno(stdout), _O_WTEXT) == -1) {
            wcerr << L"Не удалось установить режим для stdout" << endl;
            return 1;
        }
        if (_setmode(_fileno(stderr), _O_WTEXT) == -1) {
            wcerr << L"Не удалось установить режим для stderr" << endl;
            return 1;
        }
        if (_setmode(_fileno(stdin), _O_U16TEXT) == -1) {
            wcerr << L"Не удалось установить режим для stdin" << endl;
            return 1;
        }
        if (_setmode(_fileno(stdout), _O_U16TEXT) == -1) {
            wcerr << L"Не удалось установить режим для stdout" << endl;
            return 1;
        }
        if (_setmode(_fileno(stderr), _O_U16TEXT) == -1) {
            wcerr << L"Не удалось установить режим для stderr" << endl;
            return 1;
        }

        // Установить название окна
        SetConsoleTitle(L"Mafia Game Generator");

        // Центрирование окна консоли
        centerConsoleWindow();
#endif

        // Запрос количества игроков
        int numPlayers;
        std::wcout << L"Введите количество игроков: ";
        while (true) {
            if (!(std::wcin >> numPlayers)) {
                std::wcin.clear();
                std::wcin.ignore(INT_MAX, '\n');
                std::wcerr << L"Неверный ввод. Количество игроков должно быть числом и не менее 4." << std::endl;
                std::wcout << L"Введите количество игроков: ";
            }
            else if (numPlayers < 4) {
                std::wcerr << L"Неверный ввод. Количество игроков должно быть не менее 4." << std::endl;
                std::wcout << L"Введите количество игроков: ";
            }
            else {
                std::wcin.ignore(INT_MAX, '\n');
                break;
            }
        }

        // Запрос имен игроков
        std::vector<std::wstring> players(numPlayers);
        std::wcout << L"Введите имена игроков:\n";

        for (int i = 0; i < numPlayers; ++i) {
            std::wstring playerName;
            std::wcout << L"Игрок " << i + 1 << L": ";
            while (true) {
                std::getline(std::wcin, playerName);
                if (playerName.empty()) {
                    std::wcout << L"Имя не может быть пустым.\n";
                    std::wcout << L"Игрок " << i + 1 << L": ";
                }
                else {
                    players[i] = playerName;
                    break;
                }
            }
        }

        std::wcout << L"\n--- Назначение ролей ---\n";

        // Назначение ролей
        std::vector<Role> roles(numPlayers, Civilian);
        assignRoles(numPlayers, roles);

        // Создание файлов и вывод ролей
        createFilesAndPrintRoles(players, roles);

        std::wcout << L"\n---------------\n";

        // Сообщение о завершении
        std::wcout << L"Файлы созданы." << std::endl;
        std::wcout << L"Нажмите Enter для выхода...";
        std::wcin.get();
    }
    catch (const std::exception& e) {
        std::wcerr << L"Критическая ошибка: " << e.what() << std::endl;
        std::wcout << L"Нажмите Enter для выхода...";
        std::wcin.get();
    }

    return 0;
}