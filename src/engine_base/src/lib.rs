extern crate gl;
extern crate glutin;
extern crate cog;

pub fn engine_init(title: &'static str, width: uint, height: uint) -> Game {
    let game = Game {
        title: title,
        dimmensions: (width, height),
    };
    return game;
}

pub struct Game {
    title: &'static str,
    dimmensions: (uint, uint),
}

impl Game {
    pub fn run(&self) {
        let (game_width, game_height) = self.dimmensions;

        let window = glutin::WindowBuilder::new()
            .with_title(String::from_str(self.title))
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
