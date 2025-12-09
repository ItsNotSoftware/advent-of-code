#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <format>

using namespace std;

vector<string> read_file(const string& file) {
    ifstream f(file);
    if (!f.is_open()) {
        cerr << "Error opening " << file << "\n";
        exit(1);
    }
    string s;
    vector<string> lines;
    while (getline(f, s)) {
        if (!s.empty()) lines.push_back(s);
    }
    return lines;
}

int rotate_dial(int current, string rotation) {
    int sign = rotation[0] == 'L' ? -1 : 1;
    int dist = stoi(rotation.substr(1));
    return (current + sign * dist % 100 + 100) % 100;
}

int64_t part1(const vector<string>& lines) {
    int current = 50;
    int n_zeros = 0;

    for (string r : lines) {
        current = rotate_dial(current, r);
        if (current == 0) n_zeros++;
    }

    return n_zeros;
}

int64_t part2(const vector<string>& lines) {
    int current = 50;
    int n_zeros = 0;

    for (string r : lines) {
        int sign = r[0] == 'L' ? -1 : 1;
        int dist = stoi(r.substr(1));

        n_zeros += dist / 100;  // full rotations
        int result = rotate_dial(current, r);

        if (sign == -1 && result > current && current != 0) n_zeros++;
        if (sign == 1 && result < current && result != 0) n_zeros++;
        if (result == 0) n_zeros++;
        current = result;
    }

    return n_zeros;
}

int main(int argc, char** argv) {
    bool example = (argc > 1 && string(argv[1]) == "--example");
    string filename = example ? "example.txt" : "input.txt";

    auto lines = read_file(filename);
    auto p1 = part1(lines);
    auto p2 = part2(lines);

    cout << "Part 1: " << p1 << endl;
    cout << "Part 2: " << p2 << endl;

    return 0;
}
