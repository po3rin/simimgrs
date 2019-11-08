use image;
extern crate simimgrs;

use std::env;

fn main() {
    let mut args = env::args();
    args.next(); // skip command name

    // open image file 1
    let img_path1 = match args.next() {
        None => {
            eprintln!("Error: Input file path is not specified. ");
            return;
        }
        Some(s) => s,
    };

    // open image file 2
    let img_path2 = match args.next() {
        None => {
            eprintln!("Error: Input file path is not specified. ");
            return;
        }
        Some(s) => s,
    };

    let img1 = image::open(&img_path1).unwrap();
    let img2 = image::open(&img_path2).unwrap();

    let checker = simimgrs::SimilarChecker::new(10, 8, 8);

    println!("similar image: {}", checker.is_similar(img1, img2))
}
