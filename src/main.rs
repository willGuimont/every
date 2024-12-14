use clap::{arg, value_parser, Arg, ArgAction, Command};
use std::io::Write;
use std::process::Command as StdCommand;
use std::{io, thread};

fn main() {
    let matches = Command::new("every")
        .version("0.1.0")
        .author("William Guimont-Martin william.guimont-martin@norlab.ulaval.ca")
        .about("Run a command every n seconds")
        .arg(
            arg!(<INTERVAL>)
                .help("Interval in seconds")
                .required(true)
                .value_parser(value_parser!(u64).range(0..)),
        )
        .arg(arg!(<COMMAND>).help("Command to run").required(true))
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Print more information")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("async").short('a').long("async").help(
            "Run the command asynchronously. Do not wait for the command to finish before running it again",
        ).action(ArgAction::SetTrue))
        .get_matches();

    let verbose: bool = matches.get_flag("verbose");
    let sync: bool = !matches.get_flag("async");
    let interval: u64 = matches.get_one::<u64>("INTERVAL").unwrap().clone();
    let command: &str = matches.get_one::<String>("COMMAND").unwrap().as_str();

    ctrlc::set_handler(move || {
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        if verbose {
            println!("Running command: {}", command);
        }

        if sync {
            run_sync(command);
        } else {
            run_async(command.to_string());
        }

        thread::sleep(std::time::Duration::from_secs(interval));
    }
}

fn run_sync(command: &str) {
    let output = StdCommand::new("sh").arg("-c").arg(command).output();

    match output {
        Ok(output) => {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
        }
        Err(e) => {
            eprintln!("Error running command: {}", e);
            return;
        }
    }
}

fn run_async(command: String) {
    thread::spawn(move || {
        run_sync(command.as_str());
    });
}
