//! This crate checks for similar image using average hash algorism.
//! The average of the luminance values ​​is calculated, and the 64-bit
//! hash value is calculated as “1” for greater than the average
//! and “0” for less than the average.

use image::{self, FilterType, GenericImageView};

/// SimilarChecker has settings for detecting similar image.
pub struct SimilarChecker {
    threshold: usize,
    compressed_w: usize,
    compressed_h: usize,
}

impl SimilarChecker {
    /// Inits SimilarChecker with params.
    ///
    /// # Examples
    ///
    /// ```
    /// let checker = simimgrs::SimilarChecker::new(10, 8, 8);
    /// ```
    pub fn new(threshold: usize, compressed_w: usize, compressed_h: usize) -> SimilarChecker {
        SimilarChecker {
            threshold,
            compressed_w,
            compressed_h,
        }
    }

    /// Checks for similar image using average hash algorism.
    ///
    /// # Examples
    ///
    /// ```
    /// let img1 = image::open("testdata/aws_batch.png").unwrap();
    /// let img2 = image::open("testdata/aws_rekognition.png").unwrap();

    /// let checker = simimgrs::SimilarChecker::new(10, 8, 8);
    /// assert!(!checker.is_similar(img1, img2))
    /// ```
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
    fn get_distance_1() {
        assert_eq!(get_distance(2247878505465, 2488321179641), 6)
    }

    #[test]
    fn get_distance_2() {
        assert_eq!(get_distance(17431013446337445887, 17431022259610337215), 3)
    }
}
