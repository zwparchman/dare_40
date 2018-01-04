.PHONY:clean 

json = player.json enemy_a.json red_ball.json fire-rate-up.json null-powerup.json shield-up.json shield-regen.json green-ball.json boss_b.json

all: $(json)
	cargo build

run: $(json)
	cargo run

clean:
	cargo clean
	rm $(json)

%.json: %.ase
	$(shell [ -f $< ] )
	aseprite -b $< --format json-array --sheet $(basename $<).png --data $@ 
