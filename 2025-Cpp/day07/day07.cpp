#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <format>
#include <algorithm>
#include <unordered_map>

#define SPLITTER '^'

using namespace std;
using Mat = std::vector<std::vector<char>>;

Mat read_file(const std::string& file) {
    std::ifstream f(file);
    if (!f.is_open()) {
        std::cerr << "Error opening " << file << "\n";
        std::exit(1);
    }

    std::string s;
    Mat map;

    while (std::getline(f, s)) {
        if (!s.empty()) {
            std::vector<char> row(s.begin(), s.end());
            map.push_back(std::move(row));
        }
    }
    return map;
}

int64_t part1(Mat& map) {
    int64_t count = 0;
    const int64_t rows = (int64_t)map.size();
    const int64_t cols = (int64_t)map[0].size();

    vector<pair<int64_t, int64_t>> beams = {{0, cols / 2}};

    while (!beams.empty()) {
        vector<pair<int64_t, int64_t>> moved;
        moved.reserve(beams.size());

        for (auto [l, c] : beams) {
            int64_t nl = l + 1;
            if (nl < rows) moved.emplace_back(nl, c);
        }

        if (moved.empty()) break;

        sort(moved.begin(), moved.end());
        moved.erase(unique(moved.begin(), moved.end()), moved.end());

        vector<pair<int64_t, int64_t>> next;

        for (auto [l, c] : moved) {
            if (map[l][c] == SPLITTER) {
                count++;
                if (c - 1 >= 0) next.emplace_back(l, c - 1);
                if (c + 1 < cols) next.emplace_back(l, c + 1);
            } else {
                next.emplace_back(l, c);
            }
        }

        sort(next.begin(), next.end());
        next.erase(unique(next.begin(), next.end()), next.end());
        beams.swap(next);
    }

    return count;
}

int64_t part2(const Mat& map) {
    const int64_t rows = (int64_t)map.size();
    const int64_t cols = (int64_t)map[0].size();

    auto key = [cols](int64_t l, int64_t c) { return (int64_t)l * cols + c; };
    unordered_map<int64_t, int64_t> active, next;

    active[key(0, cols / 2)] = 1;
    int64_t finished = 0;

    while (!active.empty()) {
        next.clear();

        for (const auto& [k, cnt] : active) {
            int64_t l = (int64_t)(k / cols);
            int64_t c = (int64_t)(k % cols);
            int64_t nl = l + 1;

            if (nl >= rows) {
                finished += cnt;
                continue;
            }

            if (map[nl][c] == SPLITTER) {
                if (c - 1 >= 0) {
                    next[key(nl, c - 1)] += cnt;
                } else {
                    finished += cnt;
                }
                if (c + 1 < cols) {
                    next[key(nl, c + 1)] += cnt;
                } else {
                    finished += cnt;
                }
            } else {
                next[key(nl, c)] += cnt;
            }
        }

        active.swap(next);
    }

    return finished;
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
