use std::fs::File;
use std::io::prelude::*;


//use std::path::Path;
//use std::io::{BufRead, BufReader};
fn main() {
     // Create a path to the desired file
   // let path = Path::new("C:/Users/romus/Downloads/DS210/hw9/src/data.txt");
 //   let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file: File = File::open("C:/Users/romus/Downloads/DS210/final_project/Amazon0601.txt").expect("Unable to open file");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    read_amazon_meta(1);
}


pub fn read_amazon_meta(productid: usize) -> String {
    let mut file2: File = File::open("C:/Users/romus/Downloads/DS210/final_project/amazon-meta.txt").expect("Unable to open file");
    let mut contents2: String = String::new();
    file2.read_to_string(&mut contents2).expect("Unable to read file");
    let mut products: Vec<Product> = vec![];
    let mut product: Option<Product> = None;
    for line in contents2.lines() {
        if line.starts_with("Id:") {
            if let Some(p) = product {
                products.push(p);
            }
            product = Some(Product {
                id: line[4..].trim().to_string(),
                asin: "".to_string(),
                title: "".to_string(),
                group: "".to_string(),
                //salesrank: 0,
                similar: vec![],
                categories: vec![],
                reviews: "".to_string(),
            });
        } else if line.starts_with("ASIN:") {
            product.as_mut().unwrap().asin = line[6..].trim().to_string();
        } else if line.starts_with("  title:") {
            product.as_mut().unwrap().title = line[9..].trim().to_string();
        } else if line.starts_with("  group:") {
            product.as_mut().unwrap().group = line[9..].trim().to_string();
        } /*else if line.starts_with("  salesrank:") {
            product.as_mut().unwrap().salesrank = line[10..].trim().parse().unwrap();
        }*/ else if line.starts_with("  similar:") {
            product.as_mut().unwrap().similar = line[10..].split_whitespace().map(|s| s.to_string()).collect();
        } else if line.starts_with("  categories:") {
            product.as_mut().unwrap().categories = line[13..].split_whitespace().map(|s| s.to_string()).collect();
        } else if line.starts_with("  reviews:") {
            // let rating: f32 = line[8..].split_whitespace().nth(2).unwrap().parse().unwrap();
            // let count: u32 = line[8..].split_whitespace().nth(7).unwrap().parse().unwrap();
            product.as_mut().unwrap().reviews = line[10..].trim().to_string();
        }
    }
    //println!("{} products", products.len());
    println!("{:?}", products[productid]);
    // return title as String
    return products[productid].title.to_string();

    
}
#[derive(Debug)]
struct Product {
    id: String,
    asin: String,
    title: String,
    group: String,
   // salesrank: u32,
    similar: Vec<String>,
    categories: Vec<String>,
    reviews: String,
}