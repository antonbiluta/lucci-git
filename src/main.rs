use clap::{Command, Arg};

fn main() {
    let matches = Command::new("Lucci")
        .version("0.1.0")
        .about("A simple git-like version control system")
        .subcommand_required(true)
        .subcommand(Command::new("add")
            .about("Add file contents to the index")
            .arg(Arg::new("FILE")
                .help("Files to add")
                .required(true)
            )
        )
        .subcommand(Command::new("status")
            .about("Show the working tree status")
        )
        .subcommand(Command::new("push")
            .about("Update remote refs along with associated objects")
        )
        .subcommand(Command::new("pull")
            .about("Fetch from and integrate with another repository or a local branch")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let files: Vec<_> = sub_matches.get_many::<String>("FILE").unwrap().collect();
            println!("Adding files: {:?}", files);
            // Здесь потом как-нибудь логику добавления файлов сделаю
        },
        Some(("status", _)) => {
            println!("Showing status");
            // Так же про статус
        },
        Some(("push", _)) => {
            println!("Pushing changes");
            // TODO
        },
        Some(("pull", _)) => {
            println!("Pulling changes");
            // TODO
        },
        _ => unreachable!(), // cla[
    }
}
