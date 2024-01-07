
extern crate image;

use crate::{state, assets};

pub enum FilterType {
    Nearest, Linear
}

pub struct Texture {
    pub texture: glium::texture::SrgbTexture2d,
    pub name: String,
    pub filter: FilterType,
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

    let texture = glium::texture::SrgbTexture2d::new(&state.display, image_data).unwrap();

    return Texture {
        texture: texture,
        name: filename.to_string(),
        filter: FilterType::Linear,
    };
}

pub fn from_grayscale_buffer(buffer: Vec<u8>, width: u32, height: u32, name: &str, state: &state::State) -> Texture {
    let image_data = glium::texture::RawImage2d {
        data: std::borrow::Cow::Owned(buffer),
        format: glium::texture::ClientFormat::U8,
        width,
        height,
    };

    let texture = glium::texture::SrgbTexture2d::new(&state.display, image_data).unwrap();

    return Texture {
        texture,
        name: name.to_string(),
        filter: FilterType::Linear,
    };
}

impl Texture {
    pub fn put_in_assets(self, state: &mut state::State) {
        state.assets.insert(self.name.clone(), assets::Asset::Texture(self));
    }

    pub fn set_filtering(mut self, filter: FilterType) -> Texture {
        self.filter = filter;
        return self;
    }

    pub fn get_uniform(&self) -> glium::uniforms::Sampler<glium::texture::SrgbTexture2d> {
        return glium::uniforms::Sampler(&self.texture, get_sampler_behavior(&self.filter));
    }
}

fn get_sampler_behavior(filter: &FilterType) -> glium::uniforms::SamplerBehavior {
    return match filter {
        FilterType::Nearest => glium::uniforms::SamplerBehavior {
            magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
            minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
            ..Default::default()
        },
        FilterType::Linear => glium::uniforms::SamplerBehavior {
            magnify_filter: glium::uniforms::MagnifySamplerFilter::Linear,
            minify_filter: glium::uniforms::MinifySamplerFilter::Linear,
            ..Default::default()
        },
    };
}



