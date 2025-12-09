CXX = g++
CXXFLAGS = -std=c++20 -O2

SRC = day07.cpp
OUT = day07

all: $(OUT)

$(OUT): $(SRC)
	$(CXX) $(CXXFLAGS) $(SRC) -o $(OUT)

run: $(OUT)
	./$(OUT)

example: $(OUT)
	./$(OUT) --example

clean:
	rm -f $(OUT)
