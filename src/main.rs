use image::ImageOutputFormat;
use image::imageops::FilterType;
use std::fs::File;

const IN_FILENAMES: &[&str] = &[
    "hannah-reding-unsplash-1920x1280",
    "martin-fu-unsplash-1920x1280",
    "nick-kane-unsplash-1920x1280"
];

fn main() {
    for in_filename in IN_FILENAMES {
        let in_path = format!("in/{}.jpg", in_filename);
        let in_img = image::open(in_path).unwrap();
        
        let resized_img = in_img.resize_exact(1920, 1280, FilterType::Lanczos3);
    
        let mut quality_percent = 0;
        while quality_percent <= 100 {
            let out_path = format!("out/{}-image-{}.jpg", in_filename, quality_percent);
            let mut out_file = File::create(out_path).unwrap();
            
            resized_img.write_to(&mut out_file, ImageOutputFormat::Jpeg(quality_percent)).unwrap();
            
            quality_percent += 10;
        }
    }
}
