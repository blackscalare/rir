use crate::renderer::texture_handler::AnimatedTexture;

pub struct AnimationHandler {
    pub texture: AnimatedTexture,
    pub current_frame: i32,
    pub frame_timer: f32,
    pub frame_delay: f32,
}

impl AnimationHandler {
    pub unsafe fn new(texture_path: &str, frame_delay: f32) -> Self {
        Self {
            texture: AnimatedTexture::new(texture_path),
            current_frame: 0,
            frame_timer: 0.0,
            frame_delay,
        }
    }

    pub unsafe fn new_from_memory(data: &[u8], extension: &str, frame_delay: f32) -> Self {
        Self {
            texture: AnimatedTexture::new_from_memory(data, extension),
            current_frame: 0,
            frame_timer: 0.0,
            frame_delay,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.texture.frame_count() <= 1 {
            return;
        }

        self.frame_timer += delta_time;
        if self.frame_timer >= self.frame_delay {
            self.frame_timer = 0.0;
            self.current_frame = (self.current_frame + 1) % self.texture.frame_count();
        }

        unsafe {
            self.texture.update_texture_to_frame(self.current_frame);
        }
    }

    pub fn get_frame_source_rect(&self) -> raylib::core::math::Rectangle {
        let (w, h) = self.texture.frame_size();

        println!("{} {}", w, h);
        raylib::core::math::Rectangle {
            x: 0.0,
            y: (self.current_frame * h) as f32,
            width: w as f32,
            height: h as f32,
        }
    }
}
