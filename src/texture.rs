
extern crate image;

use crate::state;

pub struct Texture {
    pub texture: glium::texture::SrgbTexture2d,
    pub name: String,
}

pub fn create_to_assets(filename: &str, state: &mut state::State) {
    let res = create(filename, state);
    state.assets.insert(filename.to_string(), crate::assets::Asset::Texture(res));
}

pub fn create(filename: &str, state: &state::State) -> Texture {
    let mut path = String::from("./assets/images/");
    path.push_str(filename);

    let data = std::fs::read(path).unwrap();

    let image = image::load(
            std::io::Cursor::new(data),
            image::ImageFormat::Png
        ).unwrap().to_rgba8();

    let image_dimensions = image.dimensions();
    let image_data = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    let texture = glium::texture::SrgbTexture2d::new(&state.display, image_data).unwrap(); // TODO alpha!

    return Texture {
        texture: texture,
        name: filename.to_string(),
    };
}



