pub mod fs {
    use std::path::Path;

    use anyhow::Result;
    use image::{open, DynamicImage};

    pub fn read_image<T: AsRef<Path>>(path: T) -> Result<DynamicImage> {
        let img = open(path)?;
        Ok(img)
    }

    pub fn write_image<T: AsRef<Path>>(path: T, img: &DynamicImage) -> Result<()> {
        img.save(path)?;
        Ok(())
    }
}
