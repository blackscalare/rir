use raylib::prelude::*;

pub struct Renderer {
    width: i32,
    height: i32,
    rl_handle: Option<RaylibHandle>,
    rl_thread: Option<RaylibThread>,
}

impl Renderer {
    pub fn new(width: i32, height: i32) -> Renderer {
        Renderer {
            width,
            height,
            rl_handle: None,
            rl_thread: None,
        }
    }

    pub fn init(&mut self) {
        let (handle, thread) = raylib::init()
            .size(self.width, self.height)
            .title("rir")
            .build();

        self.rl_handle = Some(handle);
        self.rl_thread = Some(thread);
    }

    pub fn start(&self) {
        if let (Some(handle)) = &self.rl_handle {
            while !handle.window_should_close() {
                &self.draw();
            }
        }
    }

    fn draw(&self) {}
}
