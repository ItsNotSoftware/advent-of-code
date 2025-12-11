#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <format>
#include <regex>
#include <queue>
#include <map>
#include <functional>
#include <cmath>
#include <sstream>
#include <unordered_map>
#include <algorithm>
#include <numeric>

using namespace std;
using Bitmask = int;

struct State {
    int cost = 0;
    Bitmask lights = 0;
};

class Machine {
   public:
    Bitmask target_l;
    vector<Bitmask> buttons_m;
    vector<vector<int>> buttons_i;
    vector<int> target_j;
    vector<int> last_touch;
    Machine(const string& s) : target_l(0) {
        // Target mask
        {
            std::regex re_target(R"(\[([.#]+)\])");
            std::smatch m;
            if (std::regex_search(s, m, re_target)) {
                string inside = m[1].str();
                for (size_t i = 0; i < inside.size() && i < 16; ++i) {
                    if (inside[i] == '#') target_l |= static_cast<Bitmask>(1u << i);
                }
            }
        }

        // Button masks
        {
            std::regex re_buttons(R"(\(([^()]*)\))");
            auto it = std::sregex_iterator(s.begin(), s.end(), re_buttons);
            auto end = std::sregex_iterator();

            for (; it != end; ++it) {
                string inside = (*it)[1].str();
                if (inside.empty()) continue;

                Bitmask mask = 0;
                std::stringstream ss(inside);
                string token;
                vector<int> v;
                while (std::getline(ss, token, ',')) {
                    if (token.empty()) continue;
                    int idx = std::stoi(token);
                    v.push_back(idx);
                    if (idx >= 0 && idx < 16) mask |= static_cast<Bitmask>(1u << idx);
                }
                buttons_i.push_back(v);
                buttons_m.push_back(mask);
            }
        }

        // Joltages
        {
            std::regex re_jolt(R"(\{([^}]*)\})");
            std::smatch m;
            if (std::regex_search(s, m, re_jolt)) {
                string inside = m[1].str();
                std::stringstream ss(inside);
                string token;
                while (std::getline(ss, token, ',')) {
                    if (token.empty()) continue;
                    int val = std::stoi(token);
                    target_j.push_back(static_cast<int>(val));
                }
            }
        }

        // Sort buttons by descending size to tighten part 2 search branching.
        vector<int> order(buttons_i.size());
        iota(order.begin(), order.end(), 0);
        sort(order.begin(), order.end(), [&](int a, int b) {
            if (buttons_i[a].size() != buttons_i[b].size()) return buttons_i[a].size() > buttons_i[b].size();
            return a < b;
        });
        vector<vector<int>> sorted_i;
        vector<Bitmask> sorted_m;
        sorted_i.reserve(buttons_i.size());
        sorted_m.reserve(buttons_m.size());
        for (int idx : order) {
            sorted_i.push_back(buttons_i[idx]);
            sorted_m.push_back(buttons_m[idx]);
        }
        buttons_i.swap(sorted_i);
        buttons_m.swap(sorted_m);

        // Last touch per counter for part 2 bounds.
        int cnt = static_cast<int>(target_j.size());
        last_touch.assign(cnt, -1);
        for (int i = 0; i < static_cast<int>(buttons_i.size()); ++i) {
            for (int idx : buttons_i[i]) {
                if (idx >= 0 && idx < cnt) last_touch[idx] = i;
            }
        }
    }

    int find_light_cost() {
        State s = {.cost = 0, .lights = 0};
        queue<State> q;
        q.push(s);

        while (!q.empty()) {
            auto s = q.front();
            q.pop();

            if (s.lights == target_l) return s.cost;

            for (int i = 0; i < buttons_m.size(); i++) {
                State new_s = {.cost = s.cost + 1, .lights = s.lights ^ buttons_m[i]};
                q.push(new_s);
            }
        }
        return 0;
    }

    int find_joltage_cost() {
        if (target_j.empty()) return 0;

        // Depth-first branch-and-bound over button press counts (order doesn't matter).
        const int INF = 1e9;
        vector<int> remaining = target_j;
        int global_best = INF;

        // Greedy upper bound: repeatedly press the button that hits the most remaining counters
        // as many times as possible without overshooting.
        {
            vector<int> tmp = remaining;
            int greedy = 0;
            while (true) {
                int best_idx = -1, best_hit = 0, best_size = 0;
                int best_max_press = 0;
                for (int i = 0; i < static_cast<int>(buttons_i.size()); ++i) {
                    int hit = 0;
                    int max_press = INT32_MAX;
                    for (int idx : buttons_i[i]) {
                        if (idx < 0 || idx >= static_cast<int>(tmp.size())) continue;
                        if (tmp[idx] > 0) ++hit;
                        max_press = min(max_press, tmp[idx]);
                    }
                    if (hit == 0 || max_press == 0) continue;
                    if (hit > best_hit || (hit == best_hit && (int)buttons_i[i].size() > best_size)) {
                        best_hit = hit;
                        best_idx = i;
                        best_size = static_cast<int>(buttons_i[i].size());
                        best_max_press = max_press;
                    }
                }
                if (best_idx == -1) break;
                greedy += best_max_press;
                for (int idx : buttons_i[best_idx]) tmp[idx] -= best_max_press;
            }
            if (all_of(tmp.begin(), tmp.end(), [](int v) { return v == 0; })) global_best = greedy;
        }

        vector<int> suffix_max_span(buttons_i.size(), 1);
        for (int i = static_cast<int>(buttons_i.size()) - 1; i >= 0; --i) {
            suffix_max_span[i] = buttons_i[i].empty() ? 1 : static_cast<int>(buttons_i[i].size());
            if (i + 1 < static_cast<int>(buttons_i.size())) suffix_max_span[i] = max(suffix_max_span[i], suffix_max_span[i + 1]);
        }

        function<int(const vector<int>&, int)> rem_heuristic = [&](const vector<int>& rem, int idx) -> int {
            int rem_sum = 0;
            int rem_max = 0;
            for (int r : rem) {
                rem_sum += r;
                rem_max = max(rem_max, r);
            }
            if (rem_max == 0) return 0;
            int span = (idx < static_cast<int>(suffix_max_span.size())) ? suffix_max_span[idx] : 1;
            int span_bound = (rem_sum + span - 1) / span;
            return max(rem_max, span_bound);
        };

        struct RemKeyHash {
            size_t operator()(const pair<int, vector<int>>& p) const noexcept {
                size_t h = static_cast<size_t>(p.first);
                for (int v : p.second) {
                    h ^= static_cast<size_t>(v) + 0x9e3779b97f4a7c15ULL + (h << 6) + (h >> 2);
                }
                return h;
            }
        };
        unordered_map<pair<int, vector<int>>, int, RemKeyHash> memo;

        function<int(int, int)> dfs = [&](int btn_idx, int presses_so_far) -> int {
            if (btn_idx == static_cast<int>(buttons_i.size())) {
                return all_of(remaining.begin(), remaining.end(), [](int v) { return v == 0; }) ? 0 : INF;
            }

            auto key = make_pair(btn_idx, remaining);
            auto it_m = memo.find(key);
            if (it_m != memo.end()) return it_m->second;

            const auto& button = buttons_i[btn_idx];
            if (button.empty()) {
                int res = dfs(btn_idx + 1, presses_so_far);
                memo[key] = res;
                return res;
            }

            // Fast solve when this is the last button.
            if (btn_idx == static_cast<int>(buttons_i.size()) - 1) {
                int required = -1;
                for (size_t i = 0; i < remaining.size(); ++i) {
                    bool in_button = find(button.begin(), button.end(), static_cast<int>(i)) != button.end();
                    if (!in_button) {
                        if (remaining[i] != 0) return memo[key] = INF;  // impossible
                        continue;
                    }
                    if (required == -1) required = remaining[i];
                    else if (remaining[i] != required) return memo[key] = INF;
                }
                int res = required >= 0 ? required : INF;
                if (res != INF) global_best = min(global_best, presses_so_far + res);
                memo[key] = res;
                return res;
            }

            int max_cnt = INT32_MAX;
            int min_cnt = 0;
            for (int idx : button) {
                if (idx >= 0 && idx < static_cast<int>(remaining.size())) {
                    max_cnt = min(max_cnt, remaining[idx]);
                    if (last_touch[idx] == btn_idx) {
                        min_cnt = max(min_cnt, remaining[idx]);  // must satisfy here
                    }
                }
            }
            if (max_cnt == INT32_MAX) max_cnt = 0;
            if (min_cnt > max_cnt) return memo[key] = INF;

            int best_here = INF;
            for (int cnt = max_cnt; cnt >= min_cnt; --cnt) {
                if (presses_so_far + cnt > global_best) continue;
                bool ok = true;
                for (int idx : button) {
                    remaining[idx] -= cnt;
                    if (remaining[idx] < 0) ok = false;
                }
                if (ok) {
                    int h = rem_heuristic(remaining, btn_idx + 1);
                    if (h != INF && presses_so_far + cnt + h <= global_best) {
                        int sub = dfs(btn_idx + 1, presses_so_far + cnt);
                        if (sub != INF) {
                            best_here = min(best_here, cnt + sub);
                            global_best = min(global_best, presses_so_far + cnt + sub);
                        }
                    }
                }
                for (int idx : button) remaining[idx] += cnt;
            }

            memo[key] = best_here;
            return best_here;
        };

        int res = dfs(0, 0);
        return res == INF ? -1 : res;
    }
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

vector<Machine> get_machines(const vector<string>& lines) {
    vector<Machine> m;
    for (auto& l : lines) m.emplace_back(l);
    return m;
}

int64_t part1(const vector<string>& lines) {
    auto machines = get_machines(lines);

    int cost = 0;
    for (auto& m : machines) cost += m.find_light_cost();

    return cost;
}

int64_t part2(const vector<string>& lines) {
    auto machines = get_machines(lines);

    int cost = 0;
    for (auto& m : machines) cost += m.find_joltage_cost();

    return cost;
}

int main(int argc, char** argv) {
    bool example = (argc > 1 && string(argv[1]) == "--example");
    string filename = example ? "example.txt" : "input.txt";

    auto lines = read_lines(filename);
    auto p1 = part1(lines);
    cout << "-" << endl;
    auto p2 = part2(lines);

    cout << "Part 1: " << p1 << endl;
    cout << "Part 2: " << p2 << endl;

    return 0;
}
