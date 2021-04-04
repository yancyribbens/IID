use image::{GenericImageView, imageops};
use std::fs;
use std::path::Path;
use std::ffi::OsStr;

use std::mem;

fn main() {
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = &entry.file_name();
                let opt_ext = Path::new(&file_name).extension().and_then(OsStr::to_str);
                let filename = Path::new(&file_name).file_stem().and_then(OsStr::to_str).unwrap();

                if let Some(ext) = opt_ext {
                    if ext == "jpg" {
                        let img = image::open(file_name).unwrap();
                        let thumb = img.thumbnail(100, 100);
                        let thumb_name = format!("{}_thumb.jpg", filename);
                        thumb.save(thumb_name).unwrap();
                    }
                }
            }
        }
    }
}
