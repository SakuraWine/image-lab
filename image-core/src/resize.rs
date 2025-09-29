use image::{RgbaImage, imageops::FilterType};

/// 画像をリサイズする
pub fn resize_lanczos3(img: &RgbaImage, width: u32, height: u32) -> RgbaImage {
    image::imageops::resize(img, width, height, FilterType::Lanczos3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;

    #[test]
    fn change_dimensions() {
        let img = RgbaImage::from_pixel(30, 30, image::Rgba([10, 20, 30, 255]));
        let out = resize_lanczos3(&img, 10, 5);
        assert_eq!(out.dimensions(), (10, 5))
    }
}
