use std::env;
use std::io::{self, BufRead};
use std::process::{self, Command};

use regex::Regex;


fn process_arguments(mut args: Vec<String>)->(String, String){

    let help_command = args.iter().find(|a|["help", "-h", "--help"].contains(&&a[..]));

    if help_command.is_some() || args.len() < 3 {
        println!("usage: regexargs RegularExpression ExecuteCommand");
        println!("ex: echo hello.txt | regexargs \"(.*)\\\\.(.*)\" echo {{0}} {{1}} {{2}}");
        println!("Output: hello.txt hello txt");
        process::exit(1);
    }

    args.remove(0);
    let user_regex_string = args.remove(0);
    let user_command = args.join(" ");

    (user_regex_string, user_command)

}


fn make_command(user_regex:&Regex, user_command:&String, line:String)->String{

    let line_parse_regex = Regex::new(r"(\{\d+\})").unwrap();

    let captures = user_regex.captures(&line);


    let matches = match captures{
        Some(caps)=> {
            let mut v:Vec<String> = caps.iter().map(|v|v.unwrap().as_str().to_string()).collect();
            v.remove(0);
            v
        },
        None=> vec![]
    };


    let mut exec_command = user_command.clone();

    for target in line_parse_regex.captures_iter(&user_command){

        let holder = target.get(1).unwrap();
        let text = holder.as_str();

        let x: &[_] = &['{', '}'];
        let index:usize = text.trim_matches(x).parse().unwrap();

        if index > matches.len(){
            println!("{:?}", matches);
            panic!(format!("{}th group is not exist", index));

        }
        
        if index == 0{
            exec_command = exec_command.replace(holder.as_str(), &line);
        }
        else{
            exec_command = exec_command.replace(holder.as_str(), &matches[index-1]);
        }

    }

    exec_command

}


fn split_commandline(command:&String) -> (String, Vec<String>){

    let mut tokens:Vec<String> = command.split(' ').map(|s| s.to_string()).collect();
    let process = tokens.remove(0);
    (process, tokens)

}


fn main() {

    let (user_regex_string, user_command) = process_arguments(env::args().collect());


    let user_regex = match Regex::new(&user_regex_string){
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            process::exit(1)
        }
    };
 

    let stdin = io::stdin();
    for line in stdin.lock().lines(){

        let this_line = line.unwrap();
        let exec_command = make_command(&user_regex, &user_command, this_line);       

        let (process, args) = split_commandline(&exec_command);
        
        let mut cmd = Command::new(process);
        cmd.args(args);

        let output = cmd.output().unwrap().stdout;
        print!("{}", String::from_utf8(output).unwrap());

    }

}





#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process_arguments() {

        let input:Vec<String> = vec!["process", "def", "ghi", "jkl"].into_iter().map(String::from).collect();
        assert_eq!(process_arguments(input), (String::from("def"), String::from("ghi jkl")));

    }


    #[test]
    fn test_make_command() {
    
        let user_regex = Regex::new(r"(\d+)").unwrap();
        let result = make_command(&user_regex, &String::from("good {1} bye"), String::from("hello 123 world"));
        assert_eq!(result, String::from("good 123 bye"));

    }


    #[test]
    fn test_split_commandline() {

        let (command, args) = split_commandline(&String::from("hello good bye"));

        assert_eq!(command, String::from("hello"));

        let v:Vec<String> = vec!["good", "bye"].into_iter().map(String::from).collect();
        assert_eq!(args, v);
    
    }


}