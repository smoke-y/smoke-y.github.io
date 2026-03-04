use std::fs;
use std::env;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

fn get_header_footer() -> Result<(String, String), Box<dyn Error>>{
    let index_src = fs::read_to_string("index.html")?;
    let header_end = index_src.find("</div>").expect("Did not find ending of div:id=\"__header\"");
    let footer_start = index_src.find("<footer>").expect("Did not find footer tag");
    let header = &index_src[0..header_end];
    let footer = &index_src[footer_start..];
    Ok((header.to_string(), footer.to_string()))
}
fn add_header_footer(file_name: &str) -> Result<String, Box<dyn Error>>{
    let (mut header, footer) = get_header_footer()?;

    let parts: Vec<&str> = file_name.split('/').collect();
    let name = parts.last().unwrap();

    let crab = fs::read_to_string(&file_name)?;
    header.push_str(&format!("<h1>{}</h1>\n", name));
    header.push_str(&crab);
    header.push_str(&footer);


    let mut file = File::create(format!("article/{}.html", name))?;
    let _ = file.write_all(header.as_bytes());

    Ok(name.to_string())
}

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("Expected crab file name");
    }

    let name = add_header_footer(&args[1])?;

    let mut article_src = fs::read_to_string("crab/articles").expect("Did not find crab/articles");
    if article_src.contains(&name) == false && ["articles", "art", "bookmarks"].contains(&name.as_str()) == false{
        let offset = article_src.find("<ul>\n").expect("Expected unordered list tag");
        article_src.insert_str(offset + 5, &format!("<li><a href=\"/article/{}.html\">{}</a></li>\n", name, name));
        fs::write("crab/articles", article_src)?;
        println!("Updating crab/article");
        let _ = add_header_footer("crab/articles");
        println!("Generating html for crab/article");
    }
    
    Ok(())
}
