//! This crate checks for similar image using average hash algorism.
//! The average of the luminance values ​​is calculated, and the 64-bit
//! hash value is calculated as “1” for greater than the average
//! and “0” for less than the average.

use image::{self, FilterType, GenericImageView};

/// SimilarChecker has settings for detecting similar image.
// #[derive(Default)]
pub struct SimilarChecker {
    threshold: usize,
    compressed_w: usize,
    compressed_h: usize,
}

impl Default for SimilarChecker {
    fn default() -> Self {
        SimilarChecker {
            threshold: 10,
            compressed_w: 8,
            compressed_h: 8,
        }
    }
}

impl SimilarChecker {
    /// Inits SimilarChecker.
    ///
    /// # Examples
    ///
    /// ```
    /// let checker = simimgrs::SimilarChecker::new();
    /// ```
    pub fn new() -> Self {
        // sets default setting.
        SimilarChecker {
            threshold: 10,
            compressed_w: 8,
            compressed_h: 8,
        }
    }

    /// Sets compression_size parametor for SimilarChecker.
    ///
    /// # Examples
    ///
    /// ```
    /// let checker = simimgrs::SimilarChecker::new().compression_size(10, 10);
    /// ```
    pub fn compression_size(self, width: usize, height: usize) -> Self {
        SimilarChecker {
            compressed_w: width,
            compressed_h: height,
            ..self
        }
    }

    /// Sets threshold parametor for SimilarChecker.
    ///
    /// # Examples
    ///
    /// ```
    /// let checker = simimgrs::SimilarChecker::new().threshold(10);
    /// ```
    pub fn threshold(self, threshold: usize) -> Self {
        SimilarChecker { threshold, ..self }
    }

    /// Checks for similar image using average hash algorism.
    ///
    /// # Examples
    ///
    /// ```
    /// let img1 = image::open("testdata/aws_batch.png").unwrap();
    /// let img2 = image::open("testdata/aws_rekognition.png").unwrap();

    /// let checker = simimgrs::SimilarChecker::new();
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

    // TODO: supports other than gray image.
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
