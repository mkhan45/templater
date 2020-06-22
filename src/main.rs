fn usage() {
    println!("Usage: template [file] ($var=val)*");
    println!("Eg: template in.txt ($idk=\"I don't know\")");
    println!("The $ sign before variables is optional but recommended");
    std::process::exit(1);
}

fn template(filename_or_string: &String, var_strs: &[String]) {
    let mut template_str = match std::fs::read_to_string(filename_or_string) {
        Ok(s) => s,
        Err(_) => filename_or_string.to_owned(),
    };

    var_strs.iter().for_each(|in_str| {
        let mut split = in_str.splitn(2, '=');
        let (pattern, replacement) = (split.next().unwrap(), split.next().unwrap());

        template_str = template_str.replace(pattern, replacement);
    });

    print!("{}", template_str);
    std::process::exit(0);
}

fn main() {
    // usage: template [file] ($var=val)*
    let arg_vec: Vec<String> = std::env::args().collect();
    let args: &[String] = arg_vec.as_slice();

    match args {
        [_, filename_or_string, var_strs @ ..] => template(filename_or_string, var_strs),
        _ => usage(),
    }
}
