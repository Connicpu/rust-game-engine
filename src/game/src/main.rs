extern crate engine_base;

fn main() {
    let game = engine_base::engine_init(
        String::from_str("Test Game"),
        1280, 720
    );
    game.run();
}
