use image::{GenericImageView, imageops};
use std::fs;
use std::path::Path;
use std::ffi::OsStr;

use std::mem;

use std::io::prelude::*;
use std::fs::File;

fn create_thumb_name(img_path: &Path) -> String {
    let parent = img_path.parent().unwrap().to_str().unwrap();
    let stem = img_path.file_stem().unwrap().to_str().unwrap();
    let ext = img_path.extension().unwrap().to_str().unwrap();

    let mut thumb_file = String::from("");

    // it feels a bit dirty to need to do this, however,
    // a seperator between parent and stem is required, and
    // if there is no parent then we don't want to include a
    // seperator by itself keeping the path relative
    if parent == "" {
        thumb_file = format!("{}_thumb.{}", stem, ext);
    } else {
        thumb_file = format!("{}/{}_thumb.{}", parent, stem, ext);
    }

    thumb_file.clone()
}

fn is_private(file_name: &str) -> bool {
    let s:Vec<&str> = file_name.split("_").collect();
    *s.last().unwrap() == "private.jpg"
}

fn is_photo(file_name: &str) -> bool {
    let extension = Path::new(&file_name)
        .extension()
        .and_then(OsStr::to_str);

    if let Some(e) = extension {
        e == "jpg" || e == "jpeg"
    } else {
        false
    }
}

fn create_thumbs(img_path: &Path) -> Vec<(String, String)>{
    let mut thumb_name_vec: Vec<(String, String)> = Vec::new();

    if let Ok(entries) = fs::read_dir(img_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("path: {:?}", entry.path());
                let fullpath = entry.path().as_path().to_str().unwrap().to_string();

                if is_private(&fullpath) {
                    continue;
                }

                if is_photo(&fullpath) {
                    let thumb_name = create_thumb_name(Path::new(&fullpath));
                    let img = image::open(&fullpath).unwrap();
                    let thumb = img.thumbnail(100, 100);
                    thumb.save(thumb_name.clone()).unwrap();

                    let t = (fullpath, thumb_name);
                    thumb_name_vec.push(t);
                }
            }
        }
    }

    thumb_name_vec
}

fn create_html(mut thumb_name_vec: Vec<(String, String)>) -> String {
    let mut html = String::from("<table>\n");

    loop {
        if thumb_name_vec.len() == 0 {
            break;
        }


        html.push_str("  <tr>\n");
        for i in 0..3 {
            if let Some((img, thumb)) = thumb_name_vec.pop() {

                let row = "    <td>\n";
                html.push_str(&row);

                let row = format!("      <a href='{}'>\n", img);
                html.push_str(&row);

    
                let row = format!("        <img width='100' height='100' src='{}'/>\n", thumb);
                html.push_str(&row);

                let row = "      </a>\n";
                html.push_str(&row);

                let row = "    </td>\n";

                html.push_str(&row);
            }
        }
        html.push_str("  </tr>\n");
    }

    html.push_str("</table>");

    html
}

fn main() -> std::io::Result<()> {
    let mut thumbs = Vec::new();

    if let Some(arg) = std::env::args().nth(1) {
        thumbs = create_thumbs(Path::new(&arg));
    } else {
        thumbs = create_thumbs(Path::new("."));
    }

    let html = create_html(thumbs);

    let mut buffer = File::create("table.html")?;
    write!(buffer, "{}", html)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_photo() {
        let file_name = "photo.jpg";
        assert!(is_photo(file_name));

        let file_name = "photo.jpeg";
        assert!(is_photo(file_name));

        let file_name = "photo.mpg";
        assert!(!is_photo(file_name));
    }

    #[test]
    fn test_is_private() {
        let file: &str = "topsecret_private.jpg";
        assert!(is_private(file));

        let file: &str = "public.jpg";
        assert!(!is_private(file));
    }

    #[test]
    fn test_create_thumbpath() {
        let img_path = Path::new("img.jpg");
        let thumb_path = create_thumb_name(img_path);
        assert_eq!("img_thumb.jpg", thumb_path);

        let img_path = Path::new("./dir/img.jpg");
        let thumb_path = create_thumb_name(img_path);
        assert_eq!("./dir/img_thumb.jpg", thumb_path);
    }

    #[test]
    fn test_create_thumbnails() {
        let file_names = create_thumbs(Path::new("./tests"));
        let (original, thumb) = &file_names[0];

        assert_eq!("./tests/IMG_20210411_132638_thumb.jpg", thumb);
        assert_eq!("./tests/IMG_20210411_132638.jpg", original);
        assert_eq!(file_names.len(), 1);

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
            (String::from("file1.jpg"), String::from("file1_thumb.jpg")),
            (String::from("file2.jpg"), String::from("file2_thumb.jpg")),
            (String::from("file3.jpg"), String::from("file3_thumb.jpg")),
            (String::from("file4.jpg"), String::from("file4_thumb.jpg"))
        ];

        test_vec.reverse();
        let html = create_html(test_vec);

        let mut expected_html = String::from("<table>\n");
        expected_html.push_str("  <tr>\n");
        expected_html.push_str("    <td>\n");
        expected_html.push_str("      <a href='file1.jpg'>\n");
        expected_html.push_str("        <img width='100' height='100' src='file1_thumb.jpg'/>\n");
        expected_html.push_str("      </a>\n");
        expected_html.push_str("    </td>\n");
        expected_html.push_str("    <td>\n");
        expected_html.push_str("      <a href='file2.jpg'>\n");
        expected_html.push_str("        <img width='100' height='100' src='file2_thumb.jpg'/>\n");
        expected_html.push_str("      </a>\n");
        expected_html.push_str("    </td>\n");
        expected_html.push_str("    <td>\n");
        expected_html.push_str("      <a href='file3.jpg'>\n");
        expected_html.push_str("        <img width='100' height='100' src='file3_thumb.jpg'/>\n");
        expected_html.push_str("      </a>\n");
        expected_html.push_str("    </td>\n");
        expected_html.push_str("  </tr>\n");
        expected_html.push_str("  <tr>\n");
        expected_html.push_str("    <td>\n");
        expected_html.push_str("      <a href='file4.jpg'>\n");
        expected_html.push_str("        <img width='100' height='100' src='file4_thumb.jpg'/>\n");
        expected_html.push_str("      </a>\n");
        expected_html.push_str("    </td>\n");
        expected_html.push_str("  </tr>\n");
        expected_html.push_str("</table>");

        assert_eq!(expected_html, html);
    }
}
