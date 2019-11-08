extern crate simimgrs;

#[test]
fn similar_gopher_img() {
    let img1 = image::open("testdata/gopher1.jpg").unwrap();
    let img2 = image::open("testdata/gopher2.jpg").unwrap();

    let checker = simimgrs::SimilarChecker::new(10, 8, 8);
    assert!(checker.is_similar(img1, img2))
}

#[test]
fn similar_aws_logo() {
    let img1 = image::open("testdata/aws_batch.png").unwrap();
    let img2 = image::open("testdata/aws_rekognition.png").unwrap();

    let checker = simimgrs::SimilarChecker::new(10, 8, 8);
    assert!(!checker.is_similar(img1, img2))
}
