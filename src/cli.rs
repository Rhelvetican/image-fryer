use std::{fs::DirBuilder, path::Path};

use anyhow::{anyhow, Ok, Result};
use clap::{Parser, ValueEnum, ValueHint};
use gcd::binary_u32;
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgba};

const FILTER_VALUE: u32 = 450;

use crate::utils::image::fs::read_image;

#[derive(Parser, Debug, Clone)]
#[command(version, long_about = None)]
pub struct Cli {
    /// The path to the image to read
    #[arg(
        value_name = "PATH",
        value_hint = ValueHint::FilePath
    )]
    pub path: String,

    /// Pixel size
    #[arg(short, long)]
    pub size: u8,

    /// Black and White
    #[arg(short, long)]
    pub bw: bool,

    /// No resize
    #[arg(short, long)]
    pub no_resize: bool,

    /// Output path
    /// If not provided, the image will be saved in the same directory as the input image
    #[arg(short, long, value_name = "PATH", value_hint = ValueHint::FilePath)]
    pub output: Option<String>,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        let path = Path::new(&self.path);
        let output_path = match self.output {
            Some(outpath) => outpath,
            None => format!(
                "{}/{}_p{}.{}",
                path.parent().unwrap().to_str().unwrap_or("."),
                path.file_stem().unwrap().to_str().unwrap_or("img"),
                self.size,
                path.extension().unwrap().to_str().unwrap_or("png")
            ),
        };
        let output_path = Path::new(&output_path);
        if !output_path.exists() {
            DirBuilder::new()
                .recursive(true)
                .create(match output_path.parent() {
                    Some(parent) => parent,
                    None => return Err(anyhow!("Invalid path")),
                })?;
        }
        let img = read_image(self.path)?;
        let mut buf = ImageBuffer::new(img.width(), img.height());
        for (x, y, col) in buf.enumerate_pixels_mut() {
            let pixel = img.get_pixel(x, y);
            let pixel = pixel.channels();
            let (r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);
            if r as u32 + g as u32 + b as u32 >= FILTER_VALUE {
                *col = Rgba([255, 255, 255, a]);
            } else {
                *col = Rgba([0, 0, 0, a]);
            }
        }
        Ok(())
    }
}
