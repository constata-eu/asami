#[macro_use]
mod support;

use image_hasher::{HasherConfig, HashAlg};

test!{ compares_images
  let one = image::open("fixtures/instagram_images/one.jpg").unwrap().into_rgb8();
  let two = image::open("fixtures/instagram_images/two.jpg").unwrap().into_rgb8();
  let three = image::open("fixtures/instagram_images/three.jpg").unwrap().into_rgb8();
  let hasher = HasherConfig::new().hash_alg(HashAlg::DoubleGradient).hash_size(100,100).to_hasher();
  let hash1 = hasher.hash_image(&one);
  let hash2 = hasher.hash_image(&two);
  let hash3 = hasher.hash_image(&three);

  println!("Image1 hash: {}", hash1.to_base64());
  println!("Image2 hash: {}", hash2.to_base64());
  println!("Image3 hash: {}", hash3.to_base64());

  println!("Hamming Distance: {}", hash1.dist(&hash2));
  println!("Hamming Distance: {}", hash1.dist(&hash3));
}
