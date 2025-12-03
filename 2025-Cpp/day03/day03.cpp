#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <format>

using namespace std;

vector<string> parse_input(const string& file) {
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

int64_t get_max_jotage(string bank, string current_val = "", int remaining = 12) {
    if (remaining == 0) return stoll(current_val);

    int max_i = 0;
    for (int i = 0; i <= bank.length() - remaining; i++) {
        if (bank[i] > bank[max_i]) max_i = i;
    }
    return get_max_jotage(bank.substr(max_i + 1, bank.length() - max_i), current_val + bank[max_i],
                          remaining - 1);
}

int64_t part1(const vector<string>& input) {
    int result = 0;

    for (string l : input) {
        auto v1_ptr = l.c_str();
        auto end_ptr = v1_ptr + l.size();
        int v2 = 0;

        for (auto ptr = v1_ptr; ptr < end_ptr - 1; ptr++) v1_ptr = *ptr > *v1_ptr ? ptr : v1_ptr;
        for (auto ptr = v1_ptr + 1; ptr < end_ptr; ptr++) v2 = *ptr > v2 ? *ptr : v2;

        result += (*v1_ptr - '0') * 10 + (v2 - '0');  //  Convert to int
    }

    return result;
}

int64_t part2(const vector<string>& input) {
    int64_t result = 0;
    for (string l : input) result += get_max_jotage(l);
    return result;
}

int main(int argc, char** argv) {
    bool example = (argc > 1 && string(argv[1]) == "--example");
    string filename = example ? "example.txt" : "input.txt";

    auto input = parse_input(filename);
    auto p1 = part1(input);
    auto p2 = part2(input);

    cout << "Part 1: " << p1 << endl;
    cout << "Part 2: " << p2 << endl;

    return 0;
}
