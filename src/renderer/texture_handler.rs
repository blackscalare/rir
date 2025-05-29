use std::ffi::CString;
use std::os::raw::c_int;

use raylib::ffi::{
    Image, /*LoadImageAnim,*/ LoadImageAnimFromMemory, LoadTextureFromImage, Texture2D,
    UnloadImage, UpdateTexture,
};

pub struct AnimatedTexture {
    image: Image,
    frame_count: i32,
    frame_width: i32,
    frame_height: i32,
    texture: FfiTextureWrapper,
}

impl AnimatedTexture {
    // pub unsafe fn new(path: &str) -> Self {
    //     let c_path = CString::new(path).unwrap();
    //     let mut frame_count: c_int = 0;
    //     let image = unsafe { LoadImageAnim(c_path.as_ptr(), &mut frame_count) };
    //     // let texture = LoadTextureFromImage(image);
    //     let raw_texture = unsafe { LoadTextureFromImage(image) };
    //
    //     let frame_width = image.width;
    //     let frame_height = image.height;
    //     let texture = FfiTextureWrapper(raw_texture);
    //
    //     println!("Loading image {} with properties:", path);
    //     println!("\tImage size: {}x{}", image.width, image.height);
    //     println!("\tFame size: {}x{}", frame_width, frame_height);
    //     println!("\tFrame count: {}", frame_count);
    //
    //     Self {
    //         image,
    //         frame_count,
    //         frame_width,
    //         frame_height,
    //         texture,
    //     }
    // }

    pub unsafe fn new_from_memory(data: &[u8], extension: &str) -> Self {
        let ext = CString::new(extension).unwrap();
        let mut frame_count: c_int = 0;
        let image = unsafe {
            LoadImageAnimFromMemory(
                ext.as_ptr(),
                data.as_ptr(),
                data.len() as i32,
                &mut frame_count,
            )
        };
        let raw_texture = unsafe { LoadTextureFromImage(image) };

        let frame_width = image.width;
        let frame_height = image.height;

        println!("Loading image from memory with properties:");
        println!("\tImage size: {}x{}", image.width, image.height);
        println!("\tFame size: {}x{}", frame_width, frame_height);
        println!("\tFrame count: {}", frame_count);

        Self {
            image,
            frame_count,
            frame_width,
            frame_height,
            texture: FfiTextureWrapper(raw_texture),
        }
    }

    pub unsafe fn update_texture_to_frame(&self, frame: i32) {
        // let frame_size = (self.frame_width * self.frame_height * 4) as usize; // 4 bytes per pixel (RGBA)
        // let offset = (frame_size * frame as usize) as isize;
        // let frame_ptr = self.image.data.offset(offset) as *mut _;
        //
        // UpdateTexture(self.texture.0, frame_ptr);
        let frame_pixels = (self.frame_width * self.frame_height) as usize;
        let bytes_per_pixel = 4;
        let frame_size_bytes = frame_pixels * bytes_per_pixel;

        let data_ptr = self.image.data as *const u8;
        let frame_ptr = unsafe { data_ptr.add(frame_size_bytes * frame as usize) } as *mut _;

        unsafe { UpdateTexture(self.texture.0, frame_ptr) };
    }

    pub fn get_texture(&self) -> &FfiTextureWrapper {
        &self.texture
    }

    pub fn frame_count(&self) -> i32 {
        self.frame_count
    }

    // pub fn frame_size(&self) -> (i32, i32) {
    //     (self.frame_width, self.frame_height)
    // }
}

impl Drop for AnimatedTexture {
    fn drop(&mut self) {
        unsafe {
            UnloadImage(self.image);
        }
    }
}

pub struct FfiTextureWrapper(pub Texture2D);

impl AsRef<Texture2D> for FfiTextureWrapper {
    fn as_ref(&self) -> &Texture2D {
        &self.0
    }
}
