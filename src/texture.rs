use glow::HasContext;

pub fn load_texture(gl: &glow::Context, path: &str, texture_unit: u32) -> glow::NativeTexture {
    unsafe {
        let image: image::DynamicImage = image::open(path).unwrap();
        let rgba_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image.flipv().to_rgba8();
        let image_width: u32 = image.width();
        let image_height: u32 = image.height();
        let image_pixels: &[u8] = rgba_image.as_raw();
        let texture: glow::NativeTexture = gl.create_texture().unwrap();

        gl.active_texture(texture_unit);
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        gl.tex_parameter_i32(glow::TEXTURE_2D,glow::TEXTURE_WRAP_S , glow::REPEAT as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR_MIPMAP_LINEAR as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR_MIPMAP_LINEAR as i32);

        gl.tex_image_2d(glow::TEXTURE_2D, 0, 
            glow::RGBA as i32, image_width as i32, 
            image_height as i32, 0, glow::RGBA, 
            glow::UNSIGNED_BYTE, Some(image_pixels));
        gl.generate_mipmap(glow::TEXTURE_2D); 

        texture
    }    
}