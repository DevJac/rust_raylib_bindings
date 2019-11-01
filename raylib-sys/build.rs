use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    bindgen();
    cmake();
    extract_colors();
}

fn bindgen() {
    println!("rustc-env=LLVM_CONFIG_PATH=/usr/bin/llvm-config");
    let bindings = bindgen::builder()
        .header("../raylib-c/src/raylib.h")
        .blacklist_item("PI")
        .generate()
        .unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    bindings
        .write_to_file(format!("{}/bindings.rs", out_dir))
        .unwrap()
}

fn cmake() {
    let raylib_dst = cmake::build("../raylib-c/");
    println!(
        "cargo:rustc-link-search=native={}/lib64/",
        raylib_dst.display()
    );
    println!("cargo:rustc-link-lib=static=raylib");
    println!("cargo:rustc-link-lib=dylib=X11")
}

fn extract_colors() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut extracted_colors_file = File::create(format!("{}/colors.rs", out_dir)).unwrap();
    let raylib_header_file = File::open("../raylib-c/src/raylib.h").unwrap();
    let reader = BufReader::new(raylib_header_file);
    for line in reader.lines() {
        let line = line.unwrap();
        parse_color(&line).map(|(c, r, g, b, a)| {
            writeln!(
                extracted_colors_file,
                r"pub const {}:Color=Color{{r:{},g:{},b:{},a:{}}};",
                c, r, g, b, a
            )
        });
    }
}

fn parse_color(s: &str) -> Option<(&str, u8, u8, u8, u8)> {
    lazy_static! {
        static ref COLOR_RE: Regex = Regex::new(
            r"(?x)
\#define\s+
(?P<c>\w+)               # Color Name
\s+CLITERAL\(Color\)\D+
(?P<r>\d+)               # Red Channel
\D+
(?P<g>\d+)               # Green Channel
\D+
(?P<b>\d+)               # Blue Channel
\D+
(?P<a>\d+)               # Alpha Channel"
        )
        .unwrap();
    }
    COLOR_RE.captures(s).map(|caps| {
        (
            caps.name("c").unwrap().as_str(),
            u8::from_str_radix(caps.name("r").unwrap().as_str(), 10).unwrap(),
            u8::from_str_radix(caps.name("g").unwrap().as_str(), 10).unwrap(),
            u8::from_str_radix(caps.name("b").unwrap().as_str(), 10).unwrap(),
            u8::from_str_radix(caps.name("a").unwrap().as_str(), 10).unwrap(),
        )
    })
}
