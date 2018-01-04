.PHONY:clean 

all: player.json
	cargo build

run:
	cargo run

clean:
	cargo clean

player.json: player.ase makefile
	aseprite -b $< --format json-array --sheet $(basename $<).png --data $@ 
