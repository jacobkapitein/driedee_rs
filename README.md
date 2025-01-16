# driedee_rs

driedee_rs is a little fun project I'm working on to familiarize myself with graphics programming and the Rust programming language.

This code is mostly inspired by the 3D graphics "code-it-yourself" video's from [javidx9](https://www.youtube.com/@javidx9).

This code is in no way efficient, fast or good. But it does work.

## How to run

- Install Rust: https://www.rust-lang.org/
- Run `cargo run` in the terminal, in the root folder of this project
    - Note: You might need an additional C compiler to build the dependencies

First time running might take a while to install the dependencies. After that, each run should be pretty quick.

Again, this is just for learning, and optimization is really bad.

## Controls

Controls are a bit funky, but they work.

- Use WASD to move
- Arrow up: Move up
- Arrow down: Move down
- Arrow left: turn left
- Arrow right: turn right

## Thoughts

I am really thankful for [javidx9](https://www.youtube.com/@javidx9) for making a great tutorial series! I have ported his project to Rust and improved some stuff, without relying on the libraries he provided.

Furthermore, I am really proud that this engine is performing pretty well. For some reason, it's running WAY faster on my 8 year old Arch Linux laptop (Intel Core i7-8550U) than my modern Windows 11 PC (AMD Ryzen 7 3800x). My thought is that the Intel CPU actually has graphics capabilities, while my AMD CPU doesn't. So the AMD CPU probably can't work as good with OpenGL rendering as the Intel CPU, even though I am only drawing pixels directly on the screen and the rendering is done by software.

Just to give an idea: it's running the `mountains.obj` scene at a minimum of 40 fps on both my old laptop and my modern PC (using release build).