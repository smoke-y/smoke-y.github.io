use std::fs;
use std::env;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

const CRAB_CONTENT_DIV: &str = "<div class=\"crab-content\">\n";

fn get_header_footer() -> Result<(String, String), Box<dyn Error>>{
    let index_src = fs::read_to_string("index.html")?;
    let header_end = index_src.find("</div>").expect("Did not find ending of div:id=\"__header\"");
    let footer_start = index_src.find("<footer>").expect("Did not find footer tag");
    let header = &index_src[0..header_end];
    let footer = &index_src[footer_start..];
    Ok((header.to_string(), footer.to_string()))
}
fn add_header_footer(name: &str, crab: &str) -> Result<String, Box<dyn Error>>{
    let (mut header, footer) = get_header_footer()?;

    header.push_str(&format!("<h1>{}</h1>\n", name));
    header.push_str(CRAB_CONTENT_DIV);
    header.push_str(&crab);
    header.push_str("\n</div>\n");
    header.push_str(&footer);

    let mut file = File::create(format!("articles/{}.html", name))?;
    let _ = file.write_all(header.as_bytes());

    Ok(name.to_string())
}

fn update_root(name: &str, c: &str, mut article_src: String) -> Result<(), Box<dyn Error>>{
    let offset = article_src.find("<ul>\n").expect("Expected unordered list tag");
    article_src.insert_str(offset + 5, &format!("<li><a href=\"/articles/{}.html\">{}</a></li>\n", name, name));
    fs::write(c, article_src)?;
    println!("Updating {c}");
    let crab = fs::read_to_string(c)?;
    let parts: Vec<&str> = c.split('/').collect();
    let fname = parts.last().unwrap();
    let _ = add_header_footer(fname, &crab);
    println!("Generating html for {c}");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("Expected crab file name");
    }

    if args[1] == "regen"{
        println!("Regenerating...");
        let entries = fs::read_dir("articles/")?;
        for entry in entries{
            let entry = entry?;
            let file_name = entry.file_name();
            let name_str = file_name.to_string_lossy();
            let article_src = fs::read_to_string(&format!("articles/{}", name_str))?;
            let mut offset = article_src.find(CRAB_CONTENT_DIV).expect("Expected div of class crab-content");
            offset += CRAB_CONTENT_DIV.len();
            let mut end_off = article_src.find("<footer>").expect("Expected footer tag") - 1;
            while article_src.as_bytes()[end_off] == b'\n'{
                end_off -= 1;
            }
            end_off -= "</div>".len();
            let crab_content = &article_src[offset..end_off];
            let stem = name_str.replace(".html", "");
            println!("Generating for {stem}");
            add_header_footer(&stem, crab_content)?;
        }
        return Ok(());
    }

    let parts: Vec<&str> = args[1].split('/').collect();
    let fname = parts.last().unwrap();
    let is_img = parts.len() >= 2 && parts[parts.len() - 2] == "images";
    let crab = fs::read_to_string(&args[1])?;
    let name = add_header_footer(fname, &crab)?;

    let c = "crab/articles";
    let ci = "crab/img";

    let article_src = fs::read_to_string(c).expect("Did not find {c}");
    if article_src.contains(&name) == false && ["articles", "art", "bookmarks", "img"].contains(&name.as_str()) == false && is_img == false{
        let _ = update_root(&name, c, article_src);
    }else if is_img {
        let img_src = fs::read_to_string(ci).expect("Did not find {ci}");
        if img_src.contains(&name) == false{
            let _ = update_root(&name, ci, img_src);
        }
    }
    
    Ok(())
}
