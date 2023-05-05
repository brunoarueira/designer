use color_thief::{Color, ColorFormat};

use std::path::Path;

fn find_color(t: image::ColorType) -> ColorFormat {
    match t {
        image::ColorType::Rgb8 => ColorFormat::Rgb,
        image::ColorType::Rgba8 => ColorFormat::Rgba,
        _ => unreachable!(),
    }
}

#[derive(Debug)]
pub struct Image<'a> {
    path: &'a str,
}

impl<'a> Image<'a> {
    pub fn new(path: &'a str) -> Self {
        Image { path }
    }

    fn resolved_path(&self) -> &Path {
        Path::new(&self.path)
    }

    pub fn palette(&self) -> Option<Vec<Color>> {
        match image::open(self.resolved_path()) {
            Ok(image) => {
                let color_type = find_color(image.color());
                let colors =
                    color_thief::get_palette(&image.into_bytes(), color_type, 10, 10).unwrap();

                Some(colors)
            }
            Err(error) => {
                match error {
                    image::ImageError::IoError(_error) => {
                        println!("Problem opening the file: {}", &self.path);
                    }
                    _ => {
                        panic!("Unexpected error when opening the file: {}", &self.path)
                    }
                }

                None
            }
        }
    }

    pub fn file_basename(&self) -> &str {
        self.resolved_path().file_stem().unwrap().to_str().unwrap()
    }

    pub fn dominant_color(&self) -> Color {
        self.palette().unwrap()[0]
    }
}
