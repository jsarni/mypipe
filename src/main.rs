extern crate clap;
use clap::{Arg,App};
use std::process::{Command,Stdio};

fn main() {

    let matches = App::new("MyPipe")
                    .version("1.0")
                    .author("Juba Saadi SARNI - 4IABD1")
                    .arg(
                        Arg::with_name("input")
                            .long("in")
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name("output")
                            .long("out")
                            .takes_value(true),
                    )
                    .get_matches();

    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();

    let input_cmd = Command::new(input)
                            .output()
                            .expect("Error input");

    let output_cmd = Command::new(output)
                                .arg(String::from_utf8_lossy(&input_cmd.stdout).to_string())
                                .output()
                                .expect("Error output");

    println!("{}", String::from_utf8_lossy(&output_cmd.stdout).to_string());
}
