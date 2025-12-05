CXX = g++
CXXFLAGS = -std=c++20 -O2

SRC = day05.cpp
OUT = day05

all: $(OUT)

$(OUT): $(SRC)
	$(CXX) $(CXXFLAGS) $(SRC) -o $(OUT)

run: $(OUT)
	./$(OUT)

example: $(OUT)
	./$(OUT) --example

clean:
	rm -f $(OUT)
