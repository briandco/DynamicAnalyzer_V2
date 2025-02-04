# Compiler
CXX = g++
OPTIMIZATION ?= -O3
CXXFLAGS = $(OPTIMIZATION) -std=c++17

# Source files
SRC = main.cpp transporter.cpp vehicle.cpp
OBJ = $(SRC:.cpp=.o)
ASM_FILES = $(SRC:.cpp=.s)
TARGET = app

# Rules
all: $(TARGET)

$(TARGET): $(OBJ)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(OBJ)

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

%.s: %.cpp
	$(CXX) $(CXXFLAGS) -S $< -o $@

asm: $(ASM_FILES)

clean:
	rm -f $(OBJ) $(TARGET) $(ASM_FILES)

.PHONY: all clean asm
