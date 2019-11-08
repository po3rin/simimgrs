# simimgrs

-----

## What is

<img src="./testdata/similar.png" width="460px">

Are these similar images ? yes !!
this crate determine if it is a similar image using average hash algorism. The average of the luminance values ​​is calculated, and the 64-bit hash value is calculated as “1” for greater than the average and “0” for less than the average.

## Quick Start

```rs
use image;
extern crate simimgrs;

use std::env;

fn main() {
    let img1 = image::open("testdata/gopher1.jpg").unwrap();
    let img2 = image::open("testdata/gopher2.jpg").unwrap();

    let checker = simimgrs::SimilarChecker::new(10, 8, 8);

    println!("similar image: {}", checker.is_similar(img1, img2)) // true !
}
```

## Dependencies

https://github.com/image-rs/image
