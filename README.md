
simple
======

[![Build Status](https://travis-ci.org/alexandercampbell/simple.svg?branch=master)](https://travis-ci.org/alexandercampbell/simple)

Simple is supposed to be a completely obvious and intuitive game library for
Rust. Simple was inspired by [LOVE2D](http://love2d.org). I wrote Simple because
I liked the ease-of-use that LOVE2D provided but I wanted the advantage of a
typed compiler (Rust).

Simple is implemented as a layer on top of [AngryLawyer's
Rust-SDL2](https://github.com/AngryLawyer/rust-sdl2) and is NOT written for
performance. Remember: **Simple is a prototyping tool!**

Example
-------

```rust
let mut app = simple::Window::new("hello world", 1920, 1080);

app.set_color(255, 0, 255, 255);
app.draw_rect(simple::Rect{
    x: 100,
    y: 110,
    w: 120,
    h: 130,
});

while app.next_frame() {}
```


Maintainer
----------

Alexander Campbell <alexanderhcampbell@gmail.com>

