use image;
extern crate simimgrs;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("arguments require only 2 path of image files");
    }

    let img_path1 = &args[1];
    let img_path2 = &args[2];

    let img1 = image::open(&img_path1).unwrap();
    let img2 = image::open(&img_path2).unwrap();

    let checker = simimgrs::SimilarChecker::default();

    println!("similar image: {}", checker.is_similar(img1, img2))
}
