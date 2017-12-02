OFLAGS = 
CC=g++
STD=-std=c++17
CFLAGS= -g -c -W -Wall -Wextra $(STD) -Wno-missing-field-initializers -Wshadow \
				$(OFLAGS)
LFLAGS= -g $(STD) $(OFLAGS) -lraylib -lGL -lglfw -lopenal

.PHONY:clean 

Objects= main.o

all : $(Objects) program  

program : $(Objects)
	$(CC) $(Std) $(Objects) $(LFLAGS)  -o program

$(Objects): %.o: %.cpp
	$(CC) $(CFLAGS) $<

dbg: program
	gdb program

run: program
	./program

time: program
	time ./program

cache: program
	rm c*grind* -f
	valgrind --tool=cachegrind ./program

call: program
	rm c*grind* -f
	valgrind --tool=callgrind --collect-jumps=yes --dump-instr=yes ./program

inspect: 
	kcachegrind c*grind\.out\.*

clean:
	rm -f *o 
	rm -f program
	rm -f c*grind\.out\.*
