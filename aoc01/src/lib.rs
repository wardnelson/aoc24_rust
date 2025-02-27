use std::error::Error;
use std::fs;
//use std::process;
use regex::Regex;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // create empty vectors
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    // get vector content
    get_vector_content(config, & mut list1, & mut list2)?;
    // if let Err(e) = get_vector_content(config, & mut list1, & mut list2) 
    // {
    //     eprintln!("Application error: {e}");
    //     process::exit(1);
    // }

    dump_vector("list1", &list1);
    dump_vector("list2", &list2);
    // sort vectors

    list1.sort();
    list2.sort();

    dump_vector("list1", &list1);
    dump_vector("list2", &list2);
    
    // calculate detlas
    let mut count = 0;
    let mut sum = 0;
    for v1 in &list1 {
        let v2 = &list2[count];
        if v1 < v2 {
            sum += v2 - v1;
        } else {
            sum += v1 - v2;
        }
        count += 1;
    }
    
    // brute force sim calc:
    let mut sim_calc = 0;   
    for v1 in &list1 {
        let mut v2_count = 0;
        for v2 in &list2 {
            if v1 == v2  {
                v2_count += 1;
            }
        }
        sim_calc += v2_count * v1;
    }

    println!("count = {count}");
    println!("sum = {sum}");
    println!("sim_calc = {sim_calc}");


    Ok(())
}


pub fn get_vector_content(config: Config, list1:  & mut Vec<u32>, list2:  & mut Vec<u32>) -> Result<(), Box<dyn Error>> {
//pub fn get_vector_content(config: Config, list1:  & mut Vec<u32>, list2:  & mut Vec<u32>) {
    
    println!("need to read {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    let re = Regex::new(r"^(\d+)\s+(\d+*)$").unwrap();
    let mut count = 0;
    for line in contents.lines() {
        count += 1;
        if ! re.is_match(line) {
            println!("skipping line {count}: [{line}]");
            continue;
        }

        // ref: https://docs.rs/regex/latest/regex/
        // ref: https://stackoverflow.com/questions/72081657/rust-lang-what-is-if-let-somex-x-doing
        let Some(caps) = re.captures(line) else {
            println!("skipping line {count}: [{line}]");
            return Ok(())
        };
        println!("v1 = {} v2 = {}", &caps[1], &caps[2]);
        list1.push(caps[1].parse::<u32>().unwrap());
        list2.push(caps[2].parse::<u32>().unwrap());
    }

    Ok(())
}

pub fn dump_vector(label: &str, list: &Vec<u32>) {

    println!("{label}:");
    let mut count = 0;

    for i in list {
        println!("{count}: {i}");
        count += 1;
    }
}

