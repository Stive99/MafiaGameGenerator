#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <fstream>
#include <random>
#include <ctime>
#include <map>
#include <locale>
#include <codecvt>

enum Language { English, Russian };

std::map<std::string, std::map<Language, std::string>> translations = {
	{"Enter the number of players: ", { {English, "Enter the number of players: "}, {Russian, "Введите количество игроков: "} }},
	{"Number of players should be at least 4.", { {English, "Number of players should be at least 4."}, {Russian, "Количество игроков должно быть не менее 4."} }},
	{"Enter the names of the players:\n", { {English, "Enter the names of the players:\n"}, {Russian, "Введите имена игроков:\n"} }},
	{"Player ", { {English, "Player "}, {Russian, "Игрок "} }},
	{" is ", { {English, " is "}, {Russian, " - "} }},
	{"Mafia", { {English, "Mafia"}, {Russian, "Мафия"} }},
	{"Civilian", { {English, "Civilian"}, {Russian, "Мирный"} }},
	{"Detective", { {English, "Detective"}, {Russian, "Детектив"} }},
};

Language currentLanguage = Russian; // Change to English if you want to use English

/// <summary>
/// Translate text to the current language
/// </summary>
/// <param name="text"></param>
/// <returns></returns>
static std::string translate(const std::string& text) {
	return translations[text][currentLanguage];
}

enum Role { Mafia, Civilian, Detective };

std::string roleToString(Role role) {
	switch (role) {
		case Mafia: return "Mafia";
		case Civilian: return "Civilian";
		case Detective: return "Detective";
	}
	return "";
}

int main() {
	std::setlocale(LC_ALL, "Russian");

	// Number of players
	int numPlayers;
	std::cout << translate("Enter the number of players: ");
	std::cin >> numPlayers;

	// if (numPlayers < 4) {
	//     std::cerr << "Number of players should be at least 4." << std::endl;
	//     return 1;
	// }
	while(!(std::cin >> numPlayers) || numPlayers < 4) {
		std::cin.clear(); // clear the error flags
		std::cin.ignore(std::numeric_limits<std::streamsize>::max(), '\n'); // ignore the rest of the line
		std::cerr << "Invalid input. Number of players should be a number and at least 4." << std::endl;
		std::cout << translate("Enter the number of players: ");
	}

	std::vector<std::string> players(numPlayers);
	std::cout << translate("Enter the number of players: ");
	for (int i = 0; i < numPlayers; ++i) {
		std::cout << translate("Player ") << i + 1 << ": ";
		std::cin >> players[i];
	}

	// Roles assignment
	std::vector<Role> roles(numPlayers, Civilian);
	int numMafias = numPlayers / 3;
	int numDetectives = std::max(1, numPlayers / 6);

	for (int i = 0; i < numMafias; ++i) {
		roles[i] = Mafia;
	}
	for (int i = numMafias; i < numMafias + numDetectives; ++i) {
		roles[i] = Detective;
	}

	// Shuffle roles
	std::srand(std::time(0));
	std::shuffle(roles.begin(), roles.end(), std::mt19937(static_cast<unsigned int>(std::random_device()())));

	// Output roles and create files
	for (int i = 0; i < numPlayers; ++i) {
		std::cout << players[i] << " is " << roleToString(roles[i]) << std::endl;
		std::ofstream outFile(players[i] + ".txt");
		outFile << "Player: " << players[i] << "\nRole: " << roleToString(roles[i]) << std::endl;
		outFile.imbue(std::locale(outFile.getloc(), new std::codecvt_utf8<wchar_t>));  // Set locale to UTF-8
		outFile.close();
	}

	std::cout << "Files created." << std::endl;
	return 0;
}