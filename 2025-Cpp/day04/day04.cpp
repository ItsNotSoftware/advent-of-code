#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <format>

using namespace std;

constexpr char ROLL = '@';
constexpr char EMPTY = '.';

vector<vector<char>> read_file(const string& file) {
    ifstream f(file);
    if (!f.is_open()) {
        cerr << "Error opening " << file << "\n";
        exit(1);
    }
    string s;
    vector<vector<char>> map;
    while (getline(f, s)) {
        if (!s.empty()) map.emplace_back(s.begin(), s.end());
    }
    return map;
}

bool can_be_accessed(const vector<vector<char>>& map, int l, int c) {
    const vector<pair<int, int>> dirs = {{
        {-1, -1},
        {-1, 0},
        {-1, 1},
        {0, -1},
        {0, 1},
        {1, -1},
        {1, 0},
        {1, 1},
    }};
    int count = 0;

    for (auto d : dirs) {
        auto [dl, dc] = d;
        l += dl;
        c += dc;

        if (l >= map.size() || l < 0 || c >= map[0].size() || c < 0) {
            l -= dl;
            c -= dc;
            continue;
        }
        if (map[l][c] != EMPTY) count++;
        l -= dl;
        c -= dc;
    }

    return count < 4;
}

int64_t part1(const vector<vector<char>>& map) {
    int64_t result = 0;
    for (int i = 0; i < map.size(); i++) {
        for (int j = 0; j < map[0].size(); j++) {
            if (map[i][j] == EMPTY) continue;
            if (can_be_accessed(map, i, j)) result++;
        }
    }
    return result;
}

int64_t part2(vector<vector<char>>& map) {
    int64_t result = 0;
    bool roll_removed = false;

    do {
        roll_removed = false;
        for (int i = 0; i < map.size(); i++) {
            for (int j = 0; j < map[0].size(); j++) {
                if (map[i][j] == EMPTY) continue;
                if (can_be_accessed(map, i, j)) {
                    result++;
                    map[i][j] = EMPTY;
                    roll_removed = true;
                }
            }
        }
    } while (roll_removed);

    return result;
}

int main(int argc, char** argv) {
    bool example = (argc > 1 && string(argv[1]) == "--example");
    string filename = example ? "example.txt" : "input.txt";

    auto map = read_file(filename);
    auto p1 = part1(map);
    auto p2 = part2(map);

    cout << "Part 1: " << p1 << endl;
    cout << "Part 2: " << p2 << endl;

    return 0;
}
