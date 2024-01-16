use std::fs;

fn main() {
  html2md();
  variables();
  shadow();
}

fn html2md() {
  let url = "https://www.rust-lang.org/";
  let output = "rust.md";
  
  println!("Fetching url: {}", url);
  let body = reqwest::blocking::get(url).unwrap().text().unwrap();

  println!("Converting html to markdown...");
  let md = html2md::parse_html(&body);

  fs::write(output, md.as_bytes()).unwrap();
  println!("Converted markdown has been saved in {}.", output);
}


/// 
fn variables() {
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);
}

/// 遮蔽
fn shadow() {
  let x = 5;
  let x = x + 1;
  {
    let x = x * 2;
    println!("The value of x in the inner scope is: {}", x);
  }
  println!("The value of x is: {}", x);
}
