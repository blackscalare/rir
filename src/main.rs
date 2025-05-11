mod render;
mod utils;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

fn main() {
    let mut renderer = render::Renderer::new(WIDTH, HEIGHT);
    renderer.init();
    renderer.start();
}
