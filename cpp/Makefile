CC = g++
CFLAGS = -Wall

TARGET = emulator
SRC = $(wildcard src/*.cpp)
HEADERS = $(wildcard src/headers/*.h)
# searches for all *.c files in the directory

OBJ = $(SRC:.c=.o)

$(TARGET): $(OBJ) 
	$(CC) $(OBJ) -o $(TARGET)
# target app is compiled using the object files *.o

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@
# Make object files beforehand and compile only the changes *.c files 
#results in faster execution.

.PHONY: clean
clean:
	rm -f $(TARGET) $(OBJ)