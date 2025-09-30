use image::{GrayImage, Rgba, RgbaImage};
use rayon::prelude::*;

/// RgbaImageからGrayImageを作成する
pub fn rgba_to_grayscale(img: &RgbaImage) -> GrayImage {
    let (w, h) = img.dimensions();
    let mut out = GrayImage::new(w, h);
    out.par_iter_mut().enumerate().for_each(|(i, px)| {
        let x = (i as u32) % w;
        let y = (i as u32) / w;
        let rgba_px = img.get_pixel(x, y);
        let luma = rgba_to_luma(*rgba_px);
        *px = luma;
    });
    out
}

/// Rgbaから輝度値を計算する
fn rgba_to_luma(px: Rgba<u8>) -> u8 {
    let luma = 0.299f32 * px[0] as f32 + 0.587f32 * px[1] as f32 + 0.114f32 * px[2] as f32;
    luma.clamp(0.0, 255.0) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_grayscale() {}
}
