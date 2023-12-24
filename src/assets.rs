
use std::collections::HashMap;

pub enum Asset {
    Texture(crate::texture::Texture),
    Shader(crate::shader::Shader),
}

// FIXME: 'static should not be used
pub type AssetBase = HashMap<String, Asset>;

pub fn crate_base() -> AssetBase {
    return HashMap::new();
}

fn get<'a>(name: &String, assets: &'a AssetBase) -> &'a Asset {
    let res = assets.get(name);

    return match res {
        None => {
            println!("ERR: couldn't find asset with name: {}", name);
            panic!(); // TODO: return a default thing?
        },
        Some(thing) => thing,
    }
}

pub fn get_texture<'a>(name: &String, assets: &'a AssetBase) -> &'a crate::texture::Texture {
    let res = get(name, assets);

    return match res {
        Asset::Texture(tex) => tex,
        _ => {
            println!("ERR: expected asset with name '{}' to be a texture, but it isn't", name);
            panic!(); // TODO: return a default thing?
        },
    }
}

pub fn get_shader<'a>(name: &String, assets: &'a AssetBase) -> &'a crate::shader::Shader {
    let res = get(name, assets);

    return match res {
        Asset::Shader(shader) => shader,
        _ => {
            println!("RR: expected asset with name '{}' to be a shader, but it isn't", name);
            panic!(); // TODO: return a default thing?
        },
    }
}
