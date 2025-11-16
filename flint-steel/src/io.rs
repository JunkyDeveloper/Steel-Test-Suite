use crate::TEST_PATH;
use flint_core::test_spec::TestSpec;
use serde_json::to_string_pretty;
use std::collections::{BTreeMap, HashMap};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::Path;
use std::{fs, io};

pub(crate) fn load_scoped_tests(scope: &[String]) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if !Path::new("index.json").exists() {
        println!("Index does not exists, so need to build the index first");
        generate_index(TEST_PATH)?;
    }
    let mut test_paths = vec![];
    let file = File::open("index.json")?;
    let reader = BufReader::new(file);
    let map: HashMap<String, Vec<String>> =
        serde_json::from_reader(reader).expect("broken index, isn't json!");
    for (k, v) in &map {
        if scope.contains(&k) {
            for path in v{
                println!("added test to current run: '{}' ", path);
                test_paths.push(path.clone());
            }
        }
    }
    Ok(test_paths)
}

pub(crate) fn get_all_test_files(start_dir: &Path) -> Vec<String> {
    let mut dirs = vec![start_dir.to_path_buf()];
    let mut index: Vec<String> = Vec::new();

    while let Some(dir) = dirs.pop() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                } else if let Some(ext) = path.extension() {
                    if ext == "json" {
                        index.push(path.to_str().unwrap().to_string());
                    }
                }
            }
        }
    }
    index
}

pub(crate) fn generate_index(path: &str) -> io::Result<()> {
    let start_dir = Path::new(path);
    if !start_dir.is_dir() {
        eprintln!("'{}' is no directory", start_dir.display());
        std::process::exit(1);
    }
    let index = get_all_test_files(start_dir);
    let mut new_index: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for i in index {
        let file = File::open(i.clone())?;
        let reader = BufReader::new(file);
        let test: TestSpec =
            serde_json::from_reader(reader).expect("broken test file, isn't json!");
        for tag in &test.tags {
            if let Some(vec) = new_index.get_mut(tag){
                vec.push(i.clone())
            }
            else {
                new_index.insert(tag.clone(), vec![i.clone()]);
            }
        }
        if test.tags.is_empty()
        {
            if let Some(vec) = new_index.get_mut("default"){
                vec.push(i.clone())
            }
            else {
                new_index.insert("default".to_string(), vec![i.clone()]);
            }
        }
    }
    let index_string = to_string_pretty(&new_index).expect("BTree for index is broken");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("index.json")?;
    file.write_all(index_string.as_bytes())?;
    Ok(())
}

pub(crate) fn get_all_tests_paths() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let test_paths: Vec<String>;
    if !Path::new("index.json").exists() {
        println!("Index does not exists, so need to build the index first");
        test_paths = get_all_test_files(Path::new(TEST_PATH));
    } else {
        let file = File::open("index.json")?;
        let reader = BufReader::new(file);
        test_paths = serde_json::from_reader(reader).expect("broken index, isn't json!");
    }
    Ok(test_paths)
}
