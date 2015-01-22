ecs-rs
======
An Entity Component System (ECS) library written in Rust.

**For info about why an ECS may be beneficial, see some of these articles:**

- http://gameprogrammingpatterns.com/component.html
- http://t-machine.org/index.php/2007/09/03/entity-systems-are-the-future-of-mmog-development-part-1/
- http://www.gamedev.net/page/resources/_/technical/game-programming/understanding-component-entity-systems-r3013
- http://cowboyprogramming.com/2007/01/05/evolve-your-heirachy/

There is a large variety of ways an ECS may work. This particular one is similar to
[Artemis](http://gamadu.com/artemis/).
Although this isn't a port to Rust, most functionality should be similar, and the
tutorials/manual there should be able to make up for the current lack of documentation here.

## Adding ecs-rs to your project
Add `ecs` to your `Cargo.toml`
```toml
[dependencies]
ecs = "*"
```
(Of of course you can pick a specific version, but at least until version 1.0, that's probably not a good idea)

## How to use ecs-rs
### Tutorial
There are parts of a WIP tutorial in the `doc/` directory. More work is being done but I'm a little bit busy on other projects and don't have that much time.
### Ask a question
I've opened an issue for questions [here](https://github.com/HeroesGrave/ecs-rs/issues/13).
Alternatively, you may occasionally be able to catch me on the #rust-gamedev IRC channel. This is the fastest way to get help but if I'm not there then leave the question in the aforementioned issue thread.
### Rustdocs
At the moment the documentation is rather lacking, but at least some information can be gathered by looking at the API docs. Run `cargo doc` in your project and open up the `ecs` docs in your browser.

### Contributions are welcome
