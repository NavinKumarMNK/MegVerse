// scan the directory and return the list of files & hashes
use std::path::{
    Path, PathBuf
};
use std::sync::{
    Arc
};


use std::fs;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use num_cpus;
use io;
use hex;
use mpsc;

use sha2::{
    Sha256, Digest as Sha256Digest
};
use md5::{
    Md5, Digest as Md5Digest
};

fn scan_directory<P>(path: P, file_callback: impl Fn(&Path) 
    -> Result<(), String> + Send + Sync + 'static) 
    -> Result<(), String> 
    where P: AsRef<Path> + Send + 'static {
    
    let path = path.as_ref().to_path_buf();
    let mut pool = ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build()
        .map_err(|err| err.to_string())?;

    println!("Scanning directory: {}", path.display());
    println!("Number of threads: {}", pool.current_num_threads());
    
    let (tx, rx) = mpsc::sync_channel(pool, current_num_threads());
    let shared_callback = Arc::new(file_callback);

    let file_visitor = |path: PathBuf| {
        let callback = shared_callback.clone();
        let tx = tx.clone();

        pool.spawn(move || {
            if let Err(err) = callback(&path){
                tx.send(Err(err)).unwrap();
            }
            else {
                tx.send(Ok(())).unwrap();
            }
        });
    };

    visit_dirs(&path, &file_visitor)?;
    drop(tx);

    let errors:Vec<String> = rx
        .iter()
        .filter_map(Result::err)
        .collect();

    if errors.is_empty() {
        Ok(())
    }
    else {
        Err(errors.join("\n"))
    }   
}

fn visit_dirs<F>(dir: &Path, file_callback: &F) -> Result<(), String>
where
    F: Fn(&Path) -> Result<(), String> + Send + Sync + 'static,{
    if dir.is_dir() {
        let entries = std::fs::read_dir(dir).map_err(
            |err| format!("Failed to read directory {}: {}", dir.display(), err)
        )?;
        for entry in entries {
            let entry = entry.map_err(
                |e| format!("Failed to read directory entry: {}: {}", dir.display(), e)
            )?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, file_callback)?;
            }
            else {
                file_callback(&path)?;
            }
        }
    }
    Ok(())
}

fn scan_file_sha256(path: &Path) -> Result<(), String> {
    let mut file = fs::File::open(path).map_err(
        |err| format!("Failed to open file {}: {}", path.display(), err)
    )?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher).map_err(
        |err| format!("Failed to read file {}: {}", path.display(), err)
    )?;
    let hash = hasher.finalize();
    println!("{}: {}", path.display(), hex::encode(hash));
    Ok(())
}

fn scan_file_md5(path: &Path) -> Result<(), String> {
    let mut file = fs::File::open(path).map_err(
        |err| format!("Failed to open file {}: {}", path.display(), err)
    )?;
    let mut hasher = Md5::new();
    io::copy(&mut file, &mut hasher).map_err(
        |err| format!("Failed to read file {}: {}", path.display(), err)
    )?;
    let hash = hasher.finalize();
    println!("{}: {}", path.display(), hex::encode(hash));
    Ok(())
}

fn main() {
    let path = Path::new("C:\\Users\\user\\Desktop\\test");
    scan_directory(path, scan_file_sha256).unwrap();
}

