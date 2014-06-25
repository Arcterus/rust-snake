all: snake

snake: src/snake.rs piston
	rustc -O -L src/piston-workspace/piston-symlinks/ -o $@ $<

piston:
	git submodule update --init --recursive && cd src/piston-workspace && chmod +x build.sh && ./build.sh && make
	touch $@

clean-snake:
	rm -f snake

clean: clean-snake
	rm -f piston
	cd src/piston-workspace && make clean
