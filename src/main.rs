extern crate clap;
use clap::{Arg, App};

use std::path::{Path, PathBuf};
use std::fs;


fn read_dir<P: AsRef<Path>>(p: P , is_all:bool, prefix: &str) {
    if let Ok(entries) = fs::read_dir(p) {

        if !is_all {
            let hidden = entries.filter(| x | {
                if let Ok(y) = x {
                    return !y.file_name().into_string().unwrap().starts_with(".");
                }
                return true;
            });
            for entry in hidden {
                if let Ok(entry) = entry {
                    print!("{}", prefix);
                    if let Ok(name) = entry.file_name().into_string() {
                        println!("|____{}", name);
                    }
                    
                    let path = entry.path();
                    if path.is_dir() {
                        let next_prefix = "| ".to_string() + prefix;
                        read_dir(&path, is_all, &next_prefix);
                    }
                }
            }            
        } else {
            for entry in entries {
                if let Ok(entry) = entry {
                    print!("{}", prefix);
                    if let Ok(name) = entry.file_name().into_string() {
                        println!("|____{}", name);
                    }
                    
                    let path = entry.path();
                    if path.is_dir() {
                        let next_prefix = "| ".to_string() + prefix;
                        read_dir(&path, is_all, &next_prefix);
                    }
                }
            }        
        }       

    }
}

fn main() {
    
    let opt = args();
    println!("{}",opt.path.display());
    read_dir(&opt.path, opt.all,"");
    
}

struct Opt {
    all: bool,
    path: PathBuf,
}

fn args () -> Box<Opt> {
    let matches = App::new("Tree")
        .version("1.0")
        .author("xuxy")
        .about("Show the directory as tree")
        .arg(Arg::with_name("INPUT")
             .help("Sets the input file to use")
             .default_value(".")
             .index(1))
        .arg(Arg::with_name("all")
             .short("a")
             .help(""))
        .get_matches();

    let mut pth: &str = ".";
    let mut all: bool = false;
    
    if let Some(files) = matches.value_of("INPUT") {
        pth = files
    }
    
    if matches.is_present("all") {
        all = true
    }
    
    Box::new(Opt{all:all, path: PathBuf::from(pth)})
}

