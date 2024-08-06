use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args:Vec <String> =env::args().collect();

    // println!("{:?}",args);

    // let query=&args[1];
    // let filename=&args[2];
    // println!("Search for {}",query);
    // println!("in filename:{}",filename);
    let config=Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments:{}",err);
        process::exit(0);
    });

    // let contents = fs::read_to_string(config.filename)
    // .expect("something were wrong reading the file");
    // println!("with text:\n{}",contents);

    if let Err(e) = minigrep::run(config){
        eprintln!("Application error:{}",e);
        process::exit(0);
    }
}
