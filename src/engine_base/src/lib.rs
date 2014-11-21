extern crate gl;
extern crate glutin;

pub fn engine_init(title: String, width: uint, height: uint) -> Game {
    let game = Game {
        title: title,
        dimmensions: (width, height),
    };
    return game;
}

pub struct Game {
    title: String,
    dimmensions: (uint, uint),
}

impl Game {
    pub fn run(self) {
        let (game_width, game_height) = self.dimmensions;

        let window = glutin::WindowBuilder::new()
            .with_title(self.title)
            .with_dimensions(game_width, game_height)
            .with_vsync()
            .build().unwrap();

        unsafe { window.make_current() };

        gl::load_with(|symbol| window.get_proc_address(symbol));

        unsafe { gl::ClearColor(0.39, 0.58, 0.93, 1.0) };

        while !window.is_closed() {
            window.wait_events();

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
                gl::Flush();
            };
            window.swap_buffers();
        }
    }
}
