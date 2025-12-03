#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <format>
#include <regex>
#include <fstream>

using namespace std;
bool is_repeating_seq(const string& s, int n_parts);

class Range {
   public:
    int64_t start;
    int64_t end;

    Range(int64_t a, int64_t b) : start(a), end(b) {}

    int64_t invalid_sum_p1() {
        int64_t result = 0;
        int64_t flag;

        for (int64_t i = start; i <= end; i++) {
            flag = 1;

            string nr = to_string(i);
            int64_t size = nr.size();
            if (size % 2 != 0) continue;

            for (int64_t j = 0; j < size / 2; j++) {
                if (nr[j] != nr[j + size / 2]) {
                    flag = 0;
                    break;
                }
            }
            result += flag * stoll(nr);
        }
        return result;
    }

    int64_t invalid_sum_p2() {
        int64_t result = 0;
        int64_t flag;

        for (int64_t i = start; i <= end; i++) {
            string nr = to_string(i);

            for (int64_t j = 2; j <= nr.size(); j++) {
                if (is_repeating_seq(nr, j)) {
                    result += stoll(nr);
                    // cout << format("{}-{} = {}\n", start, end, nr);
                    break;
                }
            }
        }
        return result;
    }
};

// Check if nr is a repeating seq of n_parts
bool is_repeating_seq(const string& s, int n_parts) {
    int len = s.size();
    if (len % n_parts != 0) return false;

    int part_size = len / n_parts;
    string first = s.substr(0, part_size);

    for (int i = 1; i < n_parts; ++i) {
        if (s.compare(i * part_size, part_size, first) != 0) return false;
    }
    return true;
}

vector<Range> parse_input(const string& path) {
    vector<Range> ranges;
    ifstream file(path);
    string s((istreambuf_iterator<char>(file)), istreambuf_iterator<char>());

    regex re(R"((\d+)-(\d+))");
    smatch match;

    for (auto it = sregex_iterator(s.begin(), s.end(), re); it != sregex_iterator(); ++it) {
        int64_t a = stoll((*it)[1].str());
        int64_t b = stoll((*it)[2].str());
        ranges.emplace_back(a, b);
    }

    return ranges;
}

int64_t part1(const vector<Range>& input) {
    int64_t result = 0;
    for (auto range : input) result += range.invalid_sum_p1();
    return result;
}

int64_t part2(const vector<Range>& input) {
    int64_t result = 0;
    for (auto range : input) result += range.invalid_sum_p2();
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
