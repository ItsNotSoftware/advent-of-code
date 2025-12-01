#!/usr/bin/env bash

if [ -z "$1" ]; then
    echo "Usage: ./init_day.sh <day_number>"
    exit 1
fi

# Normalize to 2-digit format
DAY=$(printf "%02d" "$1")
DIR="day${DAY}"
CPP="day${DAY}.cpp"

# Create directory
mkdir -p "$DIR"

# ---- Create C++ file ----
cat > "$DIR/$CPP" <<EOF
#include <cstdint>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <format>


using namespace std;

vector<string> read_lines(const string &file) {
    ifstream f(file);
    if (!f.is_open()) {
        cerr << "Error opening " << file << "\\n";
        exit(1);
    }
    string s;
    vector<string> lines;
    while (getline(f, s)) {
        if (!s.empty())
            lines.push_back(s);
    }
    return lines;
}

int64_t part1(const vector<string>& lines) {
    return 0;
}

int64_t part2(const vector<string>& lines) {
    return 0;
}

int main(int argc, char** argv) {
    bool example = (argc > 1 && string(argv[1]) == "--example");
    string filename = example ? "example.txt" : "input.txt";

    auto lines = read_lines(filename);
    auto p1 = part1(lines);
    auto p2 = part2(lines);

    cout << "Part 1: " << p1 << endl;
    cout << "Part 2: " << p2 << endl;

    return 0;
}
EOF

# ---- Create empty inputs ----
touch "$DIR/input.txt"
touch "$DIR/example.txt"

# ---- Create Makefile ----
cat > "$DIR/Makefile" <<EOF
CXX = g++
CXXFLAGS = -std=c++20 -O2

SRC = ${CPP}
OUT = day${DAY}

all: \$(OUT)

\$(OUT): \$(SRC)
	\$(CXX) \$(CXXFLAGS) \$(SRC) -o \$(OUT)

run: \$(OUT)
	./\$(OUT)

example: \$(OUT)
	./\$(OUT) --example

clean:
	rm -f \$(OUT)
EOF

echo "Created $DIR with:"
echo "  - $CPP"
echo "  - input.txt"
echo "  - example.txt"
echo "  - Makefile"
