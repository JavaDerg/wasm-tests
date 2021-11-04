use bytes::Buf;
use image::{RgbImage, ImageBuffer, DynamicImage};
use image::imageops::FilterType;
use std::ops::{Div, Mul, Add, Rem, Sub};

const GRADIENT: [f64; 6] = [
    0.23467 * 2.0,
    0.19738,
    0.11747,
    0.04941,
    0.01473,
    0.0031
];

struct Gradient {
    pub data: Box<[f64]>
}

impl Gradient {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0.0; size].into_boxed_slice(),
        }
    }

    pub fn push_pt(&mut self, pt: u32) {
        let pt = pt;
        self.add_maybe(pt as isize, GRADIENT[0]);
        for n in 0usize..6 {
            self.add_maybe(pt as isize + n as isize, GRADIENT[n]);
            self.add_maybe(pt as isize - n as isize, GRADIENT[n]);
        }
    }

    pub fn normalize(&mut self) -> usize {
        let _: Option<()> = (|| {
            let mut max = 0.0f64;
            self.data.iter().for_each(|pt| max = max.max(pt.sqrt().sqrt()));
            self.data.iter_mut().for_each(|pt| *pt = pt.sqrt().sqrt().div(max));
            Some(())
        })();
        self.data.len()
    }

    #[inline(always)]
    fn add_maybe(&mut self, index: isize, pt: f64) {
        if index < 0 || self.data.len() <= index as usize {
            return;
        }
        self.data[index as usize] += pt;
    }
}

fn clamp(val: f64, min: f64, max: f64) -> f64 {
    val.min(max).max(min)
}

fn temp(h: f64) -> [u8; 3] {
    // darkness
    let d = clamp(h / 60.0, 0.0, 1.0);
    // over saturation
    let os = clamp(h.sub(300.0).div(60.0), 0.0, 1.0);
    let osn = 1.0 - os;

    let b = 120.0.sub(clamp(h - 30.0, 0.0, 120.0)).div(120.0).sqrt();
    let g = 120.0.sub(clamp(h - 150.0, -120.0, 120.0).abs()).div(120.0).sqrt();
    let r = 120.0.sub(clamp(h - 270.0, -120.0, 120.0).abs()).div(120.0).sqrt();

    [
        r.mul(d * osn).add(os).mul(255.0) as u8,
        g.mul(d * osn).add(os).mul(255.0) as u8,
        b.mul(d * osn).add(os).mul(255.0) as u8,
    ]
}

fn main() {
    let mut bytes = bytes::Bytes::from(std::fs::read("../samples_u32_le.bin").unwrap());
    let mut data = vec![];
    data.reserve(bytes.len() / 4);
    while bytes.has_remaining() {
        data.push((bytes.get_u32_le() as f64).sqrt() as u32);
    }
    println!("{}", data.len());
    let size = *data.iter().max().unwrap() as usize;
    println!("{}", size);

    let mut gradient = Gradient::new(size);
    println!("pass 1");
    for pt in data {
        gradient.push_pt(pt);
    }
    println!("pass 2");
    let max = gradient.normalize();
    println!("pass 3");

    let mut img: RgbImage = ImageBuffer::new(max as u32, 1);

    for (b, i) in gradient.data.iter().zip(0..) {
        img[(i, 0)].0 = temp(*b * 360.0);
    }

    DynamicImage::ImageRgb8(img)
        .resize_exact(1000, 1, FilterType::Lanczos3)
        .resize_exact(1000, 200, FilterType::Nearest)
        .save("horror.png").unwrap();



    let mut img: RgbImage = ImageBuffer::new(361, 1);

    for i in 0..=360 {
        img[(i, 0)].0 = temp(i as f64);
    }

    DynamicImage::ImageRgb8(img)
        .resize_exact(1000, 1, FilterType::Lanczos3)
        .resize_exact(1000, 200, FilterType::Nearest)
        .save("temp.png").unwrap();

    println!("{}", max);
}