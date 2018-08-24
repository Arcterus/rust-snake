Snake [![Build Status](https://api.travis-ci.org/Arcterus/rust-snake.svg?branch=master)](https://travis-ci.org/Arcterus/rust-snake)
================

An implementation of [Snake](http://en.wikipedia.org/wiki/Snake_(video_game))
in [Rust](https://github.com/rust-lang/rust) using
[Piston](https://github.com/PistonDevelopers/piston).

Build Instructions
------------------

```
cargo build
```

Game Instructions
-----------------

![screenshot](https://raw.githubusercontent.com/arcterus/rust-snake/master/rust-snake.png)

You may change the direction of the snake with the arrow or WASD keys.  To pause the
game, hit either the `return` key or the `p` key.  If you'd like to
restart the game, press `r`, or press `esc` to quit.  The goal is to touch each randomly appearing
block with the "head" of the snake.  The game ends when the "head" touches
another part of the snake.

Contribute
----------

I'd appreciate any contributions, especially for fixing bugs and improving the
UI.  Contributions target Rust's master branch until Rust 1.0 is released.

Credits
-------

* Arcterus (this entire project)
* Indiv0 (updates and Cargo support)
* Nathan Scowcroft (updates and Game Over message)

License
-------

Copyright (C) 2014 by Arcterus.
This project is licensed under the MPL v2.0.  See `LICENSE` for more
details.
