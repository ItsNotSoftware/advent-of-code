#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <format>
#include <cmath>
#include <sstream>
#include <climits>
#include <algorithm>

#define SQR(x) ((x) * (x))

using namespace std;

int64_t n_connections = 1000;

class Coord {
    inline static int64_t group = 0;

   public:
    int64_t group_id;
    int64_t x, y, z;
    Coord(int64_t x, int64_t y, int64_t z) : x(x), y(y), z(z) { group_id = group++; }
    Coord(string s) {
        group_id = group++;
        stringstream ss(s);
        char comma;
        ss >> x >> comma >> y >> comma >> z;
    }
    int64_t dist(Coord& c) { return (SQR(x - c.x) + SQR(y - c.y) + SQR(z - c.z)); }
};

vector<string> read_lines(const string& file) {
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

vector<Coord> to_coord_vec(const vector<string>& lines) {
    vector<Coord> coords;
    for (auto l : lines) coords.emplace_back(l);
    return coords;
}

vector<vector<int64_t>> get_distances(vector<Coord>& coords) {
    int64_t n = coords.size();
    vector<vector<int64_t>> distances(n, vector<int64_t>(n, -1));

    for (int64_t i = 0; i < n; ++i) {
        for (int64_t j = 0; j < n; ++j) {
            if (i == j) continue;
            distances[i][j] = coords[i].dist(coords[j]);
        }
    }
    return distances;
}

pair<int, int> connect_shortest(vector<Coord>& coords, vector<vector<int64_t>>& distances) {
    int64_t min_dist = INT_MAX;
    int64_t min_i = -1, min_j = -1;

    // Find min distance
    for (int64_t i = 0; i < distances.size(); i++) {
        for (int64_t j = 0; j < distances.size(); j++) {
            int64_t dist = distances[i][j];
            if (dist < min_dist && dist != -1) {
                min_i = i;
                min_j = j;
                min_dist = dist;
            }
        }
    }
    if (min_i == -1 || min_j == -1) return {-1, -1};

    distances[min_i][min_j] = distances[min_j][min_i] = -1;

    // Merge circuits
    int64_t target_id = coords[min_i].group_id;
    int64_t replace_id = coords[min_j].group_id;
    for (auto& c : coords) {
        if (c.group_id == replace_id) c.group_id = target_id;
    }
    return {min_i, min_j};
}

int64_t part1(const vector<string>& lines) {
    auto coords = to_coord_vec(lines);
    auto distances = get_distances(coords);

    for (int64_t i = 0; i < n_connections; i++) {
        connect_shortest(coords, distances);
    }

    vector<int64_t> circuit_sizes(coords.size(), 0);
    for (auto& c : coords) {
        circuit_sizes[c.group_id]++;
    }
    sort(circuit_sizes.begin(), circuit_sizes.end(), std::greater<int64_t>());

    return circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2];
}

int64_t part2(const vector<string>& lines) {
    auto coords = to_coord_vec(lines);
    auto distances = get_distances(coords);

    while (true) {
        auto [i, j] = connect_shortest(coords, distances);
        if (i == -1 && j == -1) {
            return -1;
        }

        int64_t gid0 = coords[0].group_id;
        bool all_one = true;
        for (const auto& c : coords) {
            if (c.group_id != gid0) {
                all_one = false;
                break;
            }
        }

        if (all_one) {
            return coords[i].x * coords[j].x;
        }
    }
}

int main(int argc, char** argv) {
    bool example = (argc > 1 && string(argv[1]) == "--example");
    string filename = example ? "example.txt" : "input.txt";
    if (filename == "example.txt") n_connections = 10;

    auto lines = read_lines(filename);
    auto p1 = part1(lines);
    auto p2 = part2(lines);

    cout << "Part 1: " << p1 << endl;
    cout << "Part 2: " << p2 << endl;

    return 0;
}
