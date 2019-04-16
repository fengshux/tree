use std::env;
use std::path::Path;
use std::fs;

fn read_dir<P: AsRef<Path>>(p: P , prefix: &str) {
    if let Ok(entries) = fs::read_dir(p) {
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
                    read_dir(&path, &next_prefix);
                }
            }
        }
    }
}

fn main() {
    
    let mut args: Vec<String> = env::args().collect();
    let mut param :Vec<&str> = vec![];
    let leng = args.len();
    
    if leng == 1  {
        param.push(".");
    } else {
        args.remove(0);
        for element in args.iter() {
            param.push(element);
        }
    }

    for p in &param {

        let pa = Path::new(p);
        println!("{}",pa.display());        
        read_dir(&pa, "");
    }

}
