use image::{GenericImageView, imageops};
use std::fs;
use std::path::Path;
use std::ffi::OsStr;

use std::mem;

use std::io::prelude::*;
use std::fs::File;

fn create_thumbs(img_path: &str) -> Vec<String>{
    let mut thumb_name_vec: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(img_path) {
        for entry in entries {
            if let Ok(entry) = entry {

                let entry_file_name = &entry.file_name();
                let file_name_str = entry_file_name.to_str();

                let file_name = file_name_str.unwrap();
                println!("{}", file_name);
                let s:Vec<&str> = file_name.split("_").collect();
                if *s.last().unwrap() == "private.jpg"{
                    continue;
                }

                let path = format!("{}/{}", img_path, file_name);

                let extension = Path::new(&path)
                    .extension()
                    .and_then(OsStr::to_str);

                let file_stem = Path::new(&path)
                    .file_stem()
                    .and_then(OsStr::to_str)
                    .unwrap();

                if let Some(e) = extension {
                    if e == "jpg" {
                        let img = image::open(&path).unwrap();
                        let thumb = img.thumbnail(100, 100);
                        let thumb_name = format!("{}/{}_thumb.jpg", img_path, file_stem);
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
    let thumbs = create_thumbs(".");
    let html = create_html(thumbs);

    let mut buffer = File::create("foo.txt")?;
    write!(buffer, "{}", html)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_thumbnails() {
        let thumbs = create_thumbs("./tests");
        let mut entries = fs::read_dir("./tests").unwrap();


        let file_one = entries.next().unwrap().unwrap().file_name();
        let file_two = entries.next().unwrap().unwrap().file_name();
        let file_three = entries.next().unwrap().unwrap().file_name();

        assert_eq!(file_one, "IMG_20210411_132638_private.jpg");
        assert_eq!(file_two, "IMG_20210411_132638_thumb.jpg");
        assert_eq!(file_three, "IMG_20210411_132638.jpg");

        fs::remove_file("./tests/IMG_20210411_132638_thumb.jpg").unwrap();
    }

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
