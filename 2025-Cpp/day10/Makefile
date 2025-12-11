CXX = g++
CXXFLAGS = -std=c++20 -O3

SRC = day10.cpp
OUT = day10

all: $(OUT)

$(OUT): $(SRC)
	$(CXX) $(CXXFLAGS) $(SRC) -o $(OUT)

run: $(OUT)
	./$(OUT)

example: $(OUT)
	./$(OUT) --example

clean:
	rm -f $(OUT)
