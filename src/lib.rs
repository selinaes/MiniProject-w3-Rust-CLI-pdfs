//walks a filesystem and finds duplicate files
// use indicatif::{ParallelProgressIterator, ProgressStyle};
// use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
// use std::collections::HashMap;
// use std::error::Error;
// use walkdir::WalkDir;

use reqwest::Client;
use select::document::Document;
use select::predicate::{Attr, Name};
use std::env;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::Path;
use pdf_rs::{Document, Object};

pub fn download_pdfs(url: &str, folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    let pdf_dir = current_dir.join(folder);

    if !pdf_dir.exists() {
        create_dir_all(pdf_dir)?;
    }

    let client = Client::new();
    let body = client.get(url).send()?.text()?;
    let document = Document::from_read(body.as_bytes())?;
    let pdf_links = document.find(Attr("href", |value| value.ends_with(".pdf")))
        .map(|n| n.attr("href").unwrap().to_owned())
        .collect::<Vec<String>>();

    for link in pdf_links {
        let filename = link.split("/").last().unwrap();
        let file_path = pdf_dir.join(filename);
        let mut file = File::create(file_path)?;
        let pdf = client.get(&link).send()?.bytes()?;
        file.write_all(&pdf)?;
    }

    Ok(())
}


fn extract_table_data(pdf_file: &str) -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![];
    File::open(pdf_file)?.read_to_end(&mut buffer)?;
    let doc = Document::new(&buffer)?;

    let mut male_count = 0;
    let mut female_count = 0;

    for page in doc.pages() {
        let page = page?;
        let contents = page.contents()?;
        for content in contents {
            if let Object::Text(text) = content {
                let line = text.text();
                if line.contains("Degree-seeking, first-time, first-year") {
                    let words: Vec<&str> = line.split_whitespace().collect();
                    if words.len() >= 4 {
                        let male = words[0].parse::<i32>().ok();
                        let female = words[2].parse::<i32>().ok();
                        if let (Some(m), Some(f)) = (male, female) {
                            male_count += m;
                            female_count += f;
                        }
                    }
                }
            }
        }
    }
    println!("Number of male students: {}", male_count);
    println!("Number of female students: {}", female_count);
    Ok(())
}






// pub fn walk(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
//     let mut files = Vec::new();
//     for entry in WalkDir::new(path) {
//         let entry = entry?;
//         if entry.file_type().is_file() {
//             files.push(entry.path().to_str().unwrap().to_string());
//         }
//     }
//     Ok(files)
// }

// //Find files matching a pattern
// pub fn find(files: Vec<String>, pattern: &str) -> Vec<String> {
//     let mut matches = Vec::new();
//     for file in files {
//         if file.contains(pattern) {
//             matches.push(file);
//         }
//     }
//     matches
// }

// /*  Parallel version of checksum using rayon with a mutex to ensure
//  that the HashMap is not accessed by multiple threads at the same time
// Uses indicatif to show a progress bar
// */
// pub fn checksum(files: Vec<String>) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
//     //set the progress bar style to allow for elapsed time and percentage complete
//     let checksums = std::sync::Mutex::new(HashMap::new());
//     let pb = indicatif::ProgressBar::new(files.len() as u64);
//     let sty = ProgressStyle::default_bar()
//         .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
//         .unwrap();
//     pb.set_style(sty);
//     files.par_iter().progress_with(pb).for_each(|file| {
//         let checksum = md5::compute(std::fs::read(file).unwrap());
//         let checksum = format!("{:x}", checksum);
//         let mut checksums = checksums.lock().unwrap();
//         checksums
//             .entry(checksum)
//             .or_insert_with(Vec::new)
//             .push(file.to_string());
//     });
//     Ok(checksums.into_inner().unwrap())
// }

// /*
// Find all the files with more than one entry in the HashMap
// */
// pub fn find_duplicates(checksums: HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
//     let mut duplicates = Vec::new();
//     for (_checksum, files) in checksums {
//         if files.len() > 1 {
//             duplicates.push(files);
//         }
//     }
//     duplicates
// }

// // invoke the actions along with the path and pattern and progress bar
// pub fn run(path: &str, pattern: &str) -> Result<(), Box<dyn Error>> {
//     let files = walk(path)?;
//     let files = find(files, pattern);
//     println!("Found {} files matching {}", files.len(), pattern);
//     let checksums = checksum(files)?;
//     let duplicates = find_duplicates(checksums);
//     println!("Found {} duplicate(s)", duplicates.len());
//     for duplicate in duplicates {
//         println!("{:?}", duplicate);
//     }
//     Ok(())
// }
