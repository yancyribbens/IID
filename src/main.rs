use image::{GenericImageView, imageops};
use std::fs;
use std::path::Path;
use std::ffi::OsStr;

use std::mem;

use std::io::prelude::*;
use std::fs::File;

fn create_thumbs() -> Vec<String>{
    let mut thumb_name_vec: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {

                let file_name = &entry.file_name();
                let opt_ext = Path::new(&file_name)
                    .extension()
                    .and_then(OsStr::to_str);
                let filename = Path::new(&file_name)
                    .file_stem()
                    .and_then(OsStr::to_str)
                    .unwrap();

                if let Some(ext) = opt_ext {
                    if ext == "jpg" {
                        let img = image::open(file_name).unwrap();
                        let thumb = img.thumbnail(100, 100);
                        let thumb_name = format!("{}_thumb.jpg", filename);
                        thumb.save(thumb_name.clone()).unwrap();
                        thumb_name_vec.push(thumb_name);
                    }
                }
            }
        }
    }

    thumb_name_vec
}

fn create_html(mut thumb_name_vec: Vec<String>) -> String {
    let mut html = String::from("<table>\n");

    loop {
        if thumb_name_vec.len() == 0 {
            break;
        }


        html.push_str("  <tr>\n");
        for i in 0..3 {
            if let Some(val) = thumb_name_vec.pop() {
                let row =
                    format!("    <td><img width='100' height='100' src='{}'/></td>\n", val);
                html.push_str(&row);
            }
        }
        html.push_str("  </tr>\n");
    }

    html.push_str("</table>");

    html
}

fn main() -> std::io::Result<()> {
    let thumbs = create_thumbs();
    let html = create_html(thumbs);

    let mut buffer = File::create("foo.txt")?;
    write!(buffer, "{}", html)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_html() {
        let mut test_vec = vec![
            String::from("thumb_1"),
            String::from("thumb_2"),
            String::from("thumb_3"),
            String::from("thumb_4")
        ];

        test_vec.reverse();
        let html = create_html(test_vec);

        let mut expected_html = String::from("<table>\n");
        expected_html.push_str("  <tr>\n");
        expected_html.push_str("    <td><img width='100' height='100' src='thumb_1'/></td>\n");
        expected_html.push_str("    <td><img width='100' height='100' src='thumb_2'/></td>\n");
        expected_html.push_str("    <td><img width='100' height='100' src='thumb_3'/></td>\n");
        expected_html.push_str("  </tr>\n");
        expected_html.push_str("  <tr>\n");
        expected_html.push_str("    <td><img width='100' height='100' src='thumb_4'/></td>\n");
        expected_html.push_str("  </tr>\n");
        expected_html.push_str("</table>");

        assert_eq!(expected_html, html);
    }
}
