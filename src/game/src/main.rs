extern crate engine_base;
extern crate cog;
extern crate events;

fn main() {
    let game = engine_base::engine_init(
        "Test Game",
        1280, 720
    );
    game.run();
}
