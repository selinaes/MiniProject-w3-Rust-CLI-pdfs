//walks a filesystem and finds duplicate files
// use indicatif::{ParallelProgressIterator, ProgressStyle};
// use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
// use std::collections::HashMap;
// use std::error::Error;
// use walkdir::WalkDir;

// use lopdf::{Document, Object};
use reqwest::Client;
type SelDocument = select::document::Document;
use select::predicate::Name;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
// use std::path::Path;

pub async fn download_pdfs(url: &str, folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?; //get current directory
    let parent_dir = current_dir.parent().unwrap(); //get parent directory
    let pdf_dir = parent_dir.join(folder); //create a path to the pdf folder

    //create the pdf folder if it doesn't exist
    if !pdf_dir.exists() {
        create_dir_all(&pdf_dir)?;
    }

    let client = Client::new(); //create a client

    let res = client.get(url).send().await.unwrap().text().await?; //get the respond of the url

    let document_result = SelDocument::from_read(res.as_bytes());

    let document = match document_result {
        Ok(doc) => doc,
        Err(error) => panic!("Problem generating: {:?}", error),
    };

    //find all the links that contain the word "pdf"
    let pdf_links = document.find(Name("a")).filter_map(|n| n.attr("href"));

    for link in pdf_links {
        // if the link contains the word "pdf", download the pdf
        if link.contains("pdf") {
            let filename = link.split('/').last().unwrap();
            let file_path = pdf_dir.join(filename);
            let mut file = File::create(file_path)?;
            let pdf = client.get(link).send().await?.bytes().await?;
            file.write_all(&pdf)?;
        }
    }

    Ok(())
}

// fn extract_table_data(pdf_file_path: &str) -> Result<Vec<Vec<String>>, String> {
//     let document = match Document::load(pdf_file_path) {
//         Ok(doc) => doc,
//         Err(e) => return Err(format!("Error while loading the PDF document: {}", e)),
//     };

//     let mut table_data = Vec::new();
//     for page in document.pages() {
//         for content in page.contents() {
//             if let Object::Stream(stream) = content {
//                 let decoded = stream.decode().unwrap();
//                 let data = String::from_utf8_lossy(&decoded);
//                 if data.contains("Degree-seeking, first-time, first-year") {
//                     let lines: Vec<&str> = data.split("\n").collect();
//                     for line in lines {
//                         if line.contains("Degree-seeking, first-time, first-year") {
//                             let values: Vec<String> = line.split(",").map(|s| s.to_string()).collect();
//                             table_data.push(values);
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     Ok(table_data)
// }

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
