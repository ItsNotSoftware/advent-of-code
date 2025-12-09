#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <sstream>
#include <format>
#include <cctype>
#include <algorithm>

using namespace std;
typedef pair<vector<vector<string>>, vector<char>> Input;

bool is_only_whitespace(const std::string& s) {
    return all_of(s.begin(), s.end(), [](unsigned char c) { return std::isspace(c); });
}

Input read_file(const string& file) {
    ifstream in(file);
    vector<string> lines;
    string line;

    while (getline(in, line)) {
        if (!line.empty()) lines.push_back(line);
    }
    if (lines.empty()) return {};

    string op_line = lines.back();
    lines.pop_back();

    vector<char> ops;
    vector<size_t> col_starts;

    // Find operator positions
    for (size_t i = 0; i < op_line.size(); ++i) {
        char c = op_line[i];
        if (!isspace((unsigned char)c)) {
            ops.push_back(c);
            col_starts.push_back(i);
        }
    }

    vector<size_t> col_widths;
    col_widths.reserve(col_starts.size());
    for (size_t j = 0; j < col_starts.size(); ++j) {
        size_t start = col_starts[j];
        size_t end = (j + 1 < col_starts.size()) ? col_starts[j + 1] : op_line.size();
        col_widths.push_back(end - start);
    }

    vector<vector<string>> nums;
    nums.reserve(lines.size());

    for (auto& r : lines) {
        if (r.size() < op_line.size()) r.append(op_line.size() - r.size(), ' ');

        vector<string> row;
        row.reserve(col_starts.size());

        for (size_t j = 0; j < col_starts.size(); ++j) {
            row.emplace_back(r.substr(col_starts[j], col_widths[j]));
        }
        nums.push_back(std::move(row));
    }

    return {nums, ops};
}

pair<vector<string>, int> collect_col(const vector<vector<string>>& nums, size_t c) {
    int max_digits = 0;
    vector<string> col;
    for (const auto& row : nums) {
        string s = row[c];
        int digits = s.length();
        max_digits = digits > max_digits ? digits : max_digits;
        col.push_back(s);
    }
    return {col, max_digits};
}

int64_t get_cephalopod_nr(vector<string> values, int col, int max_digits) {
    string nr = "";
    for (auto v : values) {
        if (col >= v.length()) continue;
        nr += v[col];
    }

    if (is_only_whitespace(nr)) return 0;
    return stoll(nr);
}

int64_t part1(const Input& input) {
    int64_t result = 0;
    auto [nums, ops] = input;
    int lines = nums.size();
    int cols = nums[0].size();

    for (int c = 0; c < cols; c++) {
        char operand = ops[c];
        int64_t val = stoll(nums[0][c]);
        for (int l = 1; l < lines; l++) {
            if (operand == '+') {
                val += stoll(nums[l][c]);
            } else {
                val *= stoll(nums[l][c]);
            }
        }
        result += val;
    }

    return result;
}

int64_t part2(const Input& input) {
    int64_t result = 0;
    auto [nums, ops] = input;
    auto [col, max] = collect_col(nums, 1);
    int n_lines = nums.size();
    int n_cols = nums[0].size();

    for (int c = n_cols - 1; c >= 0; c--) {
        auto [vals, max_digits] = collect_col(nums, c);
        char operand = ops[c];
        int64_t val = operand == '+' ? 0 : 1;

        for (int i = 0; i < max_digits; i++) {
            auto nr = get_cephalopod_nr(vals, i, max_digits);
            if (nr == 0) continue;

            if (operand == '+') {
                val += nr;
            } else {
                val *= nr;
            }
        }
        result += val;
    }

    return result;
}

int main(int argc, char** argv) {
    bool example = (argc > 1 && string(argv[1]) == "--example");
    string filename = example ? "example.txt" : "input.txt";

    auto input = read_file(filename);
    auto p1 = part1(input);
    auto p2 = part2(input);

    cout << "Part 1: " << p1 << endl;
    cout << "Part 2: " << p2 << endl;

    return 0;
}
