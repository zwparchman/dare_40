.PHONY:clean 

json = player.json enemy_a.json red_ball.json

all: $(json)
	cargo build

run: $(json)
	cargo run

clean:
	cargo clean
	rm player.json player.png

%.json: %.ase
	aseprite -b $< --format json-array --sheet $(basename $<).png --data $@ 
