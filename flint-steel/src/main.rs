use std::env;
use std::path::Path;
use flint_core::index::Index;

const TEST_PATH: &str = "./test";
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut test_paths: Vec<String> = vec![];
    if args.len() > 1 {
        match args[1].as_str() {
            "index" => {
                println!("index");
                if let Err(err) = Index::generate_index(&Path::new(TEST_PATH)) {
                    println!("error while generating index: {}", err);
                }
            }
            _ => {
                println!("Will run tests on a specific scope");
                match Index::load_tagged_tests_paths(&args[1..]) {
                    Ok(_test_paths) => {
                        test_paths = _test_paths;
                    }
                    Err(err) => {
                        println!("error while loading test files: {}", err);
                    }
                }
            }
        }
    } else {
        // Loads all test from the index
        println!("Will run all tests");
        match Index::get_all_tests_paths() {
            Ok(_test_paths) => test_paths = _test_paths,
            Err(err) => {
                println!("error while getting all_tests_paths: {}", err);
            }
        }
    }
}

