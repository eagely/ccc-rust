mod solutions;

use solutions::level::Level;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

fn read_level_inputs(level_num: usize) -> Vec<(String, String)> {
    let folder = format!("inputs/level{}", level_num);
    let zip = format!("inputs/level{}.zip", level_num);
    let mut inputs = Vec::new();

    if Path::new(&folder).exists() {
        println!("found input of level {}", level_num);
        for entry in fs::read_dir(&folder).unwrap_or_else(|_| fs::read_dir(".").unwrap()) {
            let path = entry.unwrap().path();
            if path.extension().map(|s| s != "out").unwrap_or(true)
                && let Ok(content) = fs::read_to_string(&path)
            {
                inputs.push((path.to_string_lossy().to_string(), content));
            }
        }
    } else if Path::new(&zip).exists() {
        println!("unzipping input of level {}", level_num);
        let file = File::open(&zip).unwrap();
        let mut archive = ZipArchive::new(file).unwrap();
        for i in 0..archive.len() {
            let mut f = archive.by_index(i).unwrap();
            let name = f.name().to_string();
            if !name.ends_with(".out") {
                let mut s = String::new();
                f.read_to_string(&mut s).unwrap();
                inputs.push((name, s));
            }
        }
    }

    inputs
}

macro_rules! run {
    ($($level_struct:path),* $(,)?) => {{
        let levels: Vec<Box<dyn Level>> = vec![$(Box::new($level_struct),)*];

        for level in &levels {
            let inputs = read_level_inputs(level.level());
            if inputs.is_empty() {
                println!("skipping level {}", level.level());
                continue;
            }

            let mut found = false;
            for (filename, data) in inputs {
                if let Some(result) = level.solve(&data) {
                    println!("level {} input: {}\noutput:\n\n{}", level.level(), filename, result.lines().take(10).collect::<Vec<_>>().join("\n"));
                    found = true;
                    break;
                }
            }

            if !found {
                println!("skipping level {}", level.level());
            } else {
                return;
            }
        }

        println!("no levels returned a result");
    }};
}

fn main() {
    run![
        solutions::level3::Level3,
        solutions::level2::Level2,
        solutions::level1::Level1,
    ];
}
