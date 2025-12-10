#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <sstream>
#include <algorithm>

#define CROSS(v, w) ((v.first * w.second) - (v.second * w.first))

using namespace std;

class Segment {
   public:
    pair<int64_t, int64_t> p1, p2;
    pair<int64_t, int64_t> dir;

    Segment(pair<int64_t, int64_t> p1, pair<int64_t, int64_t> p2) : p1(p1), p2(p2) {
        dir = {p2.first - p1.first, p2.second - p1.second};
    }
};

static inline pair<int64_t, int64_t> sub(const pair<int64_t, int64_t>& a,
                                         const pair<int64_t, int64_t>& b) {
    return {a.first - b.first, a.second - b.second};
}

bool intercept(const Segment& s1, const Segment& s2) {
    auto A = s1.p1;
    auto B = s1.p2;
    auto C = s2.p1;
    auto D = s2.p2;

    int64_t o1 = CROSS(sub(B, A), sub(C, A));
    int64_t o2 = CROSS(sub(B, A), sub(D, A));
    int64_t o3 = CROSS(sub(D, C), sub(A, C));
    int64_t o4 = CROSS(sub(D, C), sub(B, C));

    bool ab_straddle = (o1 > 0 && o2 < 0) || (o1 < 0 && o2 > 0);
    bool cd_straddle = (o3 > 0 && o4 < 0) || (o3 < 0 && o4 > 0);

    return ab_straddle && cd_straddle;
}

bool point_in_or_on_polygon(const vector<pair<int64_t, int64_t>>& poly, double px, double py) {
    int n = (int)poly.size();

    for (int i = 0; i < n; ++i) {
        int j = (i + 1) % n;
        const auto& A = poly[i];
        const auto& B = poly[j];

        if (A.first == B.first) {  // vertical edge
            if (px == (double)A.first) {
                double ymin = min(A.second, B.second);
                double ymax = max(A.second, B.second);
                if (py >= ymin && py <= ymax) return true;
            }
        } else if (A.second == B.second) {  // horizontal edge
            if (py == (double)A.second) {
                double xmin = min(A.first, B.first);
                double xmax = max(A.first, B.first);
                if (px >= xmin && px <= xmax) return true;
            }
        }
    }

    // Ray-casting to +infinity in x
    bool inside = false;
    for (int i = 0; i < n; ++i) {
        int j = (i + 1) % n;
        double x1 = (double)poly[i].first;
        double y1 = (double)poly[i].second;
        double x2 = (double)poly[j].first;
        double y2 = (double)poly[j].second;

        bool cond = ((y1 > py) != (y2 > py));
        if (cond) {
            double x_intersect = x1 + (x2 - x1) * (py - y1) / (y2 - y1);
            if (px < x_intersect) inside = !inside;
        }
    }
    return inside;
}

vector<pair<int64_t, int64_t>> parse_input(const string& file) {
    ifstream f(file);
    if (!f.is_open()) {
        cerr << "Error opening " << file << "\n";
        exit(1);
    }
    string s;
    vector<pair<int64_t, int64_t>> squares;
    int64_t x, y;
    while (getline(f, s)) {
        if (s.empty()) continue;
        stringstream ss(s);
        char comma;
        ss >> x >> comma >> y;
        squares.emplace_back(x, y);
    }
    return squares;
}

vector<Segment> get_segments(const vector<pair<int64_t, int64_t>>& squares) {
    vector<Segment> segments;
    for (int i = 0; i < (int)squares.size() - 1; i++) {
        segments.emplace_back(squares[i], squares[i + 1]);
    }
    segments.emplace_back(squares.back(), squares[0]);
    return segments;
}

int64_t part1(const vector<pair<int64_t, int64_t>>& squares) {
    int64_t max_area = 0;

    for (auto [s1_x, s1_y] : squares) {
        for (auto [s2_x, s2_y] : squares) {
            int64_t area = (std::llabs(s1_x - s2_x) + 1) * (std::llabs(s1_y - s2_y) + 1);
            if (area > max_area) max_area = area;
        }
    }
    return max_area;
}

int64_t part2(const vector<pair<int64_t, int64_t>>& squares) {
    auto segments = get_segments(squares);
    int64_t max_area = 0;

    int n = (int)squares.size();

    for (int i = 0; i < n; ++i) {
        auto [x1_raw, y1_raw] = squares[i];
        for (int j = 0; j < n; ++j) {
            auto [x2_raw, y2_raw] = squares[j];

            int64_t xmin = min(x1_raw, x2_raw);
            int64_t xmax = max(x1_raw, x2_raw);
            int64_t ymin = min(y1_raw, y2_raw);
            int64_t ymax = max(y1_raw, y2_raw);

            int64_t area = (xmax - xmin + 1) * (ymax - ymin + 1);
            if (area <= max_area) continue;

            // Rectangle corners (axis-aligned)
            pair<int64_t, int64_t> A = {xmin, ymin};
            pair<int64_t, int64_t> B = {xmin, ymax};
            pair<int64_t, int64_t> C = {xmax, ymax};
            pair<int64_t, int64_t> D = {xmax, ymin};

            // Edges of the rectangle
            Segment e1(A, B);  // left
            Segment e2(B, C);  // top
            Segment e3(C, D);  // right
            Segment e4(D, A);  // bottom

            Segment edges[4] = {e1, e2, e3, e4};

            bool ok = true;

            for (int k = 0; k < 4 && ok; ++k) {
                const auto& e = edges[k];

                double mx = (e.p1.first + e.p2.first) / 2.0;
                double my = (e.p1.second + e.p2.second) / 2.0;
                if (!point_in_or_on_polygon(squares, mx, my)) {
                    ok = false;
                    break;
                }

                for (const auto& seg : segments) {
                    if (intercept(e, seg)) {
                        ok = false;
                        break;
                    }
                }
            }

            if (!ok) continue;

            if (area > max_area) {
                max_area = area;
            }
        }
    }
    return max_area;
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
