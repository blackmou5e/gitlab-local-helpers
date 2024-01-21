use std::env;
use std::path::Path;
use std::collections::HashMap;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let path = Path::new(file);

    let ci = gitlab_ci_parser::parse(path).unwrap();

    let mut result: HashMap<String, String> = HashMap::new();

    for (k, v) in ci.variables {
        result.insert(k, v);
    }

    for (_, job) in ci.jobs {
        for value in job.variables.iter() {
            for (k, v) in value {
                result.insert(k.to_string(), v.to_string());
            }
        }
    }

    println!("{:?}", result);
}
