use rfd::FileDialog;
use std::process;
use std::{error::Error,fs, path::PathBuf};
use regex::Regex;
fn read_text_to_json(file_path: &std::path::PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let txt=fs::read_to_string(file_path)?;
    let mut disc_no:u32=0u32;
    let disc_regex=Regex::new(r"^(\d+)\.$").unwrap();
    let movie_regex=Regex::new(r"^(\d{4}) (.*?)(（儿童)?$").unwrap();
    let mut movies=Vec::new();
    for line in txt.lines().map(str::trim).filter(|l|!l.is_empty())
    {
        if let Some(no)=disc_number(line,&disc_regex){
            disc_no=no;

        }else{
            if let Some(movie)=parse_movie(disc_no,line,&movie_regex){
                movies.push(movie);

            }
        }
    }
    Ok(PathBuf::new())
}
fn save_to_json(movies:Vec<movie>)->Result<PathBuf,Box<dyn Error>>{
    
}
fn parse_movie(disc_no:u32,line:&str,re:&Regex)->Option<Movie>{
    re.captures(line).map(|caps|{
        println!("{:#?}",caps);
        Movie{
            disc:disc_no,
            year:caps.get(1).unwrap().as_str().trim().to_string(),
            title:caps.get(2).unwrap().as_str().trim().to_string(),
            remark:caps.get(3).map(|m|m.as_str().trim().to_string())
        }
})
}

fn disc_number(line: &str,re:&Regex) -> Option<u32> {
   re.captures(line).map(|caps| caps.get(1).unwrap().as_str().parse::<u32>().unwrap())
}
struct Movie {
    disc:u32,
    year:String,
    title:String,
    remark:Option<String>
}
fn main() ->Result<(),Box<dyn Error>>{
    match FileDialog::new().add_filter("Text Files",&["txt"])
    .set_title("Select the DVD text file")
    .set_directory("C:\\Users\\Jw\\Desktop")
    .pick_file() {
        Some(file_path)=>{
            let saved_path =read_text_to_json(&file_path)?;
            println!("File saved to {:?}", saved_path);
            Ok(())

        }
        None=>{
            eprintln!("No file selected");
            process::exit(-1)
        }
}
}