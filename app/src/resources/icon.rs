use anyhow::Result;
use eframe::egui::{Context, TextureId};
use eframe::epaint::{ColorImage, ImageData};
use tiny_skia::Pixmap;
use resvg::FitTo;

fn render_svg(svg: &[u8], [width, height]: [usize; 2]) -> Result<Pixmap> {
    let opts = usvg::Options::default();
    let tree = usvg::TreeParsing::from_data(svg, &opts)?;

    let pixmap_size = usvg::Size::new(width as f64, height as f64)
        .unwrap()
        .to_screen_size();
    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(
        &tree,
        FitTo::Size(width as u32, height as u32),
        tiny_skia::Transform::identity(),
        pixmap.as_mut(),
    )
    .ok_or(anyhow::anyhow!("Failed to render SVG"))?;

    Ok(pixmap)
}

pub struct Icon {
    pub texture: TextureId,
    pub width: usize,
    pub height: usize,
}

impl Icon {
    pub fn from_svg(bytes: &[u8], size: [usize; 2], alloc: &Context) -> Result<Self> {
        let pixmap = render_svg(bytes, size)?;

        let image = ColorImage::from_rgba_unmultiplied(size, pixmap.data());
        let id = alloc
            .tex_manager()
            .write()
            .alloc("Test name".into(), ImageData::Color(image));

        Ok(Self {
            width: size[0],
            height: size[1],
            texture: id,
        })
    }

    pub fn size_f32(&self) -> (f32, f32) {
        (self.width as f32, self.height as f32)
    }

    pub fn id(&self) -> TextureId {
        self.texture
    }
}
