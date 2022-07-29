use anyhow::Result;
use eframe::egui::{Context, TextureId};
use tiny_skia::Pixmap;
use usvg::FitTo;

// fn render_svg(svg: &[u8], (width, height): (u32, u32)) -> Result<Pixmap> {
//     let opts = usvg::Options::default();
//     let tree = usvg::Tree::from_data(svg, &opts.to_ref())?;

//     let pixmap_size = usvg::Size::new(width as f64, height as f64)
//         .unwrap()
//         .to_screen_size();
//     let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
//     resvg::render(&tree, FitTo::Size(width, height), pixmap.as_mut())
//         .ok_or(anyhow::anyhow!("Failed to render SVG"))?;

//     Ok(pixmap)
// }

// fn svg2texture(svg: &[u8], size: (u32, u32), tex: &Context) -> Result<TextureHandle> {
//     let pixmap = render_svg(svg, size)?;
//     let pixels = pixmap.data();

//     let img = ColorImage::from_rgba_unmultiplied([size.0 as usize, size.1 as usize], pixels);
//     let texture = tex.load_texture("", img);
//     Ok(texture)
// }

pub struct Icon {
    pub texture: u32, // TextureHandle,
    pub width: u32,
    pub height: u32,
}

impl Icon {
    pub fn from_svg(bytes: &[u8], size: (u32, u32), alloc: &Context) -> Result<Self> {
        // alloc.load_texture();
        // let texture = svg2texture(bytes, size, alloc)?;
        // let this = Self {
        //     width: size.0,
        //     height: size.1,
        //     texture,
        // };

        // Ok(this)

        Ok(Self {
            width: 0,
            height: 0,
            texture: 0,
        })
    }

    pub fn size_f32(&self) -> (f32, f32) {
        (self.width as f32, self.height as f32)
    }

    pub fn id(&self) -> TextureId {
        // self.texture.id()
        TextureId::User(10)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::render_svg;
//     use crate::defines::icons::ICON_PLAY;

//     #[test]
//     fn test_me() {
//         let res = render_svg(ICON_PLAY, (32, 32)).unwrap();
//         res.save_png("test.png").expect("Failed to save");
//     }
// }
