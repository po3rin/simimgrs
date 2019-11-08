use image::{self, FilterType, GenericImageView};

pub struct SimilarChecker {
    pub threshold: usize,
    pub compressed_w: usize,
    pub compressed_h: usize,
}

impl SimilarChecker {
    pub fn new(threshold: usize, compressed_w: usize, compressed_h: usize) -> SimilarChecker {
        SimilarChecker {
            threshold,
            compressed_w,
            compressed_h,
        }
    }

    pub fn is_similar(&self, img1: image::DynamicImage, img2: image::DynamicImage) -> bool {
        let hash1 = get_hash(process(img1, self.compressed_w, self.compressed_h));
        let hash2 = get_hash(process(img2, self.compressed_w, self.compressed_h));

        let distance = get_distance(hash1, hash2);

        distance < self.threshold
    }
}

fn process(
    img: image::DynamicImage,
    compressed_w: usize,
    compressed_h: usize,
) -> image::DynamicImage {
    img.resize_exact(
        compressed_w as u32,
        compressed_h as u32,
        FilterType::Lanczos3,
    )
    .grayscale()
}

fn get_hash(img: image::DynamicImage) -> usize {
    let mut sum_pixels: usize = 0;
    let mut pixels: Vec<usize> = Vec::new();

    for (_x, _y, pixel) in img.pixels() {
        let red = pixel[0];
        sum_pixels += red as usize;
        pixels.push(red as usize)
    }

    let (width, height) = img.dimensions();
    let ave = (sum_pixels as f64) / (f64::from(width) * f64::from(height));

    let mut hash: usize = 0;
    let mut one: usize = 1;

    for pixel in pixels {
        if pixel as f64 > ave {
            hash |= one;
        }
        one <<= 1
    }
    hash
}

fn get_distance(hash1: usize, hash2: usize) -> usize {
    let mut d = 0;
    for i in 0..64 {
        let k = 1 << i;
        if (hash1 & k) != (hash2 & k) {
            d += 1
        }
    }
    d
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_hash_1() {
        assert_eq!(get_distance(2247878505465, 2488321179641), 6)
    }

    #[test]
    fn get_hash_2() {
        assert_eq!(get_distance(17431013446337445887, 17431022259610337215), 3)
    }
}
