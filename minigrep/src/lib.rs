use std::error::Error;
use std::fs;
use std::env;

//There is a connection between the query and filename,so we can use a struct rather than a tuple
pub struct Config{
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool,
}
//get parameters,use a tuple
// fn parse_config(args:&[String])->(&str,&str)
// {
//     let query=&args[1];//the target string
//     let filename=&args[2];//where to find
//     (query,filename)
// }

//use a struct
// fn parse_config(args:&[String])->Config
// {
//     let query=args[1].clone();
//     let filename=args[2].clone();
//     Config {query,filename}
// }

impl Config {
    pub fn new(mut args:std::env::Args)->Result<Config,&'static str>//the function actually creat a instance ,so we change the parse_config to new
{
    if args.len()<3{
        return Err("not enough arguments");
    }
    // let query=args[1].clone();
    // let filename=args[2].clone();
    args.next();
    let query=match args.next(){
        Some(arg)=>arg,
        None=>return Err("Didn't get a query string"),
    };
    let filename=match args.next(){
        Some(arg) => arg,
        None=>return Err("Didn't get a filename string"),
    };
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    println!("{}",case_sensitive);
    Ok(Config {query,filename,case_sensitive})
}
}

pub fn run (config:Config) ->Result<(),Box<dyn Error>> {
    let contents = 
        fs::read_to_string(config.filename)?;//? will return the err to the caller of run so that the caller can solve the problem
    let results = if config.case_sensitive{
        search(&config.query,&contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };
    for line in results{
        println!("{}",line);
    }
    Ok(())
}

pub fn search<'a>(query:& str,contents:&'a str)->Vec<&'a str>{
    // let mut results=Vec::new();
    // for line in contents.lines(){
    //     if line.contains(query){
    //         results.push(line);
    //     }
    // }
    // println!("run search");
    // results
    return contents.lines().filter(|line| line.contains(query)).collect()
}
pub fn search_case_insensitive<'a>(query:& str,contents:&'a str)->Vec<&'a str>{
    // let mut results=Vec::new();
    // let query=query.to_lowercase();
    // for line in contents.lines(){
    //     if line.to_lowercase().contains(&query){
    //         results.push(line);
    //     }
    // }
    // println!("run search_case_insensitive");
    // results
    return contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}





#[cfg(test)]//compiled and executed only when run test
mod test{
    use super::*;
    #[test]
    fn case_sensitive()
    {
        let query="duct";
        let contents="\
Rust:
safe,fast,productive.
Pick three.";
        assert_eq!(vec!["safe,fast,productive."],search(query,contents))
    }


    #[test]
    fn case_insensitive(){
        let query="rust";
        let contents="\
Rust:
safe,fast,productive.
Pick three.";
        assert_eq!(vec!["Rust:"],search_case_insensitive(query,contents))
    }
}