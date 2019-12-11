//! This crate checks for similar image using average hash algorism.
//! The average of the luminance values ​​is calculated, and the 64-bit
//! hash value is calculated as “1” for greater than the average
//! and “0” for less than the average.

use image::{self, FilterType, GenericImageView};
use std::sync::mpsc;
use std::thread;

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
        let w = self.compressed_w;
        let h = self.compressed_h;

        let (tx, rx) = mpsc::channel();
        let tx1 = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
            let hash1 = get_hash(process(img1, w, h));
            tx1.send(hash1).unwrap();
        });

        thread::spawn(move || {
            let hash2 = get_hash(process(img2, w, h));
            tx.send(hash2).unwrap();
        });

        let mut v: Vec<usize> = Vec::new();
        for received in rx {
            v.push(received)
        }

        let distance = get_distance(v[0], v[1], w * h);
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

/// get_hash calculate average hash.
///
/// # Examples
///
/// ```
/// let img = image::open("testdata/aws_batch.png").unwrap();
/// assert_eq!(simimgrs::get_hash(img), 18446744073709551615)
/// ```
pub fn get_hash(img: image::DynamicImage) -> usize {
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

/// get_distance gets distance between two average hash.
///
/// # Examples
///
/// ```
/// assert_eq!(simimgrs::get_distance(1110, 1101, 64), 4)
/// ```
pub fn get_distance(hash1: usize, hash2: usize, pix_num: usize) -> usize {
    let mut d = 0;
    for i in 0..pix_num {
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
        assert_eq!(get_distance(2247878505465, 2488321179641, 64), 6)
    }

    #[test]
    fn get_distance_2() {
        assert_eq!(
            get_distance(17431013446337445887, 17431022259610337215, 64),
            3
        )
    }
}
