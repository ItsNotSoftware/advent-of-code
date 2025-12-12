#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <array>
#include <unordered_map>
#include <unordered_set>
#include <sstream>

using namespace std;

using Map = unordered_map<string, vector<string>>;

Map parse_file(const string& file) {
    ifstream f(file);
    if (!f.is_open()) {
        cerr << "Error opening " << file << "\n";
        exit(1);
    }

    Map m;
    string line;

    while (getline(f, line)) {
        if (line.empty()) continue;

        auto colon = line.find(':');
        if (colon == string::npos) continue;

        string key = line.substr(0, colon);
        string rest = line.substr(colon + 1);

        istringstream iss(rest);
        string v;
        while (iss >> v) m[key].push_back(v);
    }

    return m;
}

int64_t count_paths(const Map& map, const string& node, unordered_set<string>& visiting, unordered_map<string, int64_t>& memo) {
    if (node == "out") return 1;
    if (auto it = memo.find(node); it != memo.end()) return it->second;
    if (visiting.count(node)) return 0;  // avoid cycles

    visiting.insert(node);

    int64_t paths = 0;
    if (auto it = map.find(node); it != map.end()) {
        for (const auto& next : it->second) {
            paths += count_paths(map, next, visiting, memo);
        }
    }

    visiting.erase(node);
    memo[node] = paths;
    return paths;
}

int64_t part1(const Map& map) {
    unordered_set<string> visiting;
    unordered_map<string, int64_t> memo;
    return count_paths(map, "you", visiting, memo);
}

int64_t count_paths_with_stops(const Map& map,
                               const string& node,
                               bool seen_dac,
                               bool seen_fft,
                               unordered_set<string>& visiting,
                               unordered_map<string, array<int64_t, 4>>& memo) {
    if (visiting.count(node)) return 0;  // break potential cycles

    bool now_dac = seen_dac || node == "dac";
    bool now_fft = seen_fft || node == "fft";
    int state = (now_dac ? 2 : 0) | (now_fft ? 1 : 0);

    if (node == "out") return state == 3 ? 1 : 0;

    auto [it, inserted] = memo.try_emplace(node, array<int64_t, 4>{-1, -1, -1, -1});
    auto& memoized = it->second;
    if (memoized[state] != -1) return memoized[state];

    visiting.insert(node);
    int64_t paths = 0;
    if (auto it2 = map.find(node); it2 != map.end()) {
        for (const auto& next : it2->second) {
            paths += count_paths_with_stops(map, next, now_dac, now_fft, visiting, memo);
        }
    }
    visiting.erase(node);

    memoized[state] = paths;
    return paths;
}

int64_t part2(const Map& map) {
    unordered_set<string> visiting;
    unordered_map<string, array<int64_t, 4>> memo;
    return count_paths_with_stops(map, "svr", false, false, visiting, memo);
}

int main(int argc, char** argv) {
    bool example = (argc > 1 && string(argv[1]) == "--example");
    string filename = example ? "example.txt" : "input.txt";

    auto m = parse_file(filename);
    auto p1 = part1(m);
    auto p2 = part2(m);

    cout << "Part 1: " << p1 << endl;
    cout << "Part 2: " << p2 << endl;

    return 0;
}
