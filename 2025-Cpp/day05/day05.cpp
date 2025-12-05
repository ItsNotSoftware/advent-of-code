#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <format>

using namespace std;

class Range {
   public:
    int64_t start, end;
    Range(int64_t start, int64_t end) : start(start), end(end) {}
    Range(string range) {
        size_t pos = range.find('-');
        start = stoll(range.substr(0, pos));
        end = stoll(range.substr(pos + 1));
    }
    bool in_range(int64_t val) { return val >= start && val <= end; }
};

pair<vector<Range>, vector<int64_t>> read_file(const string& file) {
    ifstream f(file);
    if (!f.is_open()) {
        cerr << "Error opening " << file << "\n";
        exit(1);
    }
    string s;
    vector<Range> ranges;
    vector<int64_t> ids;

    while (getline(f, s)) {
        if (s.empty()) break;
        ranges.emplace_back(s);
    }
    while (getline(f, s)) {
        ids.push_back(stoll(s));
    }
    return {ranges, ids};
}

void update_processed_ranges(vector<Range>& processed_ranges, Range new_range) {
    for (const auto& existing_r : processed_ranges) {
        // No overlap at all
        if (new_range.end < existing_r.start || new_range.start > existing_r.end) continue;

        //  new_range fully inside an existing range
        if (new_range.start >= existing_r.start && new_range.end <= existing_r.end) return;

        // existing range is fully inside new_range -> split new_range in two
        if (new_range.start < existing_r.start && new_range.end > existing_r.end) {
            Range left{new_range.start, existing_r.start - 1};
            Range right{existing_r.end + 1, new_range.end};

            if (left.start <= left.end) update_processed_ranges(processed_ranges, left);
            if (right.start <= right.end) update_processed_ranges(processed_ranges, right);
            return;
        }

        // Partial overlap on the left or right side -> trim
        if (new_range.start < existing_r.start) {
            new_range.end = existing_r.start - 1;
        } else {
            new_range.start = existing_r.end + 1;
        }
    }

    // Add missing range to vec
    if (new_range.start <= new_range.end) {
        processed_ranges.push_back(new_range);
    }
}

int64_t part1(const vector<Range>& ranges, const vector<int64_t>& ids) {
    int64_t result = 0;
    for (auto id : ids) {
        for (auto r : ranges) {
            if (r.in_range(id)) {
                result++;
                break;
            }
        }
    }
    return result;
}

int64_t part2(const vector<Range>& ranges, const vector<int64_t>& ids) {
    int64_t result = 0;
    vector<Range> processed_ranges = {};

    for (auto r : ranges) update_processed_ranges(processed_ranges, r);
    for (auto r : processed_ranges) result += r.end - r.start + 1;

    return result;
}

int main(int argc, char** argv) {
    bool example = (argc > 1 && string(argv[1]) == "--example");
    string filename = example ? "example.txt" : "input.txt";

    auto [ranges, ids] = read_file(filename);
    auto p1 = part1(ranges, ids);
    auto p2 = part2(ranges, ids);

    cout << "Part 1: " << p1 << endl;
    cout << "Part 2: " << p2 << endl;

    return 0;
}
