fn usage() {
    println!("Usage: template [file] ($var=val)*");
    println!("Eg: template in.txt (idk=\"I don't know\")");
    std::process::exit(1);
}

fn template(filename: &String, var_strs: &[String]) {
    let mut file_str = std::fs::read_to_string(filename).expect("Invalid file");

    var_strs.iter().for_each(|in_str| {
        let mut split = in_str.splitn(2, '=');
        let (pattern, replacement) = (split.next().unwrap(), split.next().unwrap());

        file_str = file_str.replace(pattern, replacement);
    });

    println!("{}", file_str);
    std::process::exit(0);
}

fn main() {
    // usage: template [file] ($var=val)*
    let arg_vec: Vec<String> = std::env::args().collect();
    let args: &[String] = arg_vec.as_slice();

    match args {
        [_, filename, var_strs @ ..] => template(filename, var_strs),
        _ => usage(),
    }
}
