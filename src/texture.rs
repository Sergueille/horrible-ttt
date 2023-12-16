
extern crate image;

use crate::state;

pub struct Texture<'a> {
    data: glium::texture::RawImage2d<'a, u8>,
    name: String,
}

pub fn create_to_assets(filename: &str, state: &mut state::State) {
    let res = create(filename);
    state.assets.insert(filename.to_string(), crate::assets::Asset::Texture(res));
}

pub fn create<'a>(filename: &str) -> Texture<'a> {
    let mut path = String::from("./assets/images/");
    path.push_str(filename);

    let data = std::fs::read(path).unwrap();

    let image = image::load(
            std::io::Cursor::new(data),
            image::ImageFormat::Png
        ).unwrap().to_rgba8();

    let image_dimensions = image.dimensions();
    let image_data = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    return Texture {
        data: image_data,
        name: filename.to_string(),
    };
}



