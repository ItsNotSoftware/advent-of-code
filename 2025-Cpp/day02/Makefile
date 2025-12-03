CXX = g++
CXXFLAGS = -std=c++20 -O2

SRC = day02.cpp
OUT = day02

all: $(OUT)

$(OUT): $(SRC)
	$(CXX) $(CXXFLAGS) $(SRC) -o $(OUT)

run: $(OUT)
	./$(OUT)

example: $(OUT)
	./$(OUT) --example

clean:
	rm -f $(OUT)
