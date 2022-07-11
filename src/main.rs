use std::{
    collections::HashMap,
    env, fs,
    io::{self, stdout, BufRead, Write},
    path::PathBuf,
};

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
struct CommandArgs {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    Setup,
    Post(Post),
}

#[derive(Args, Debug)]
struct Post {
    #[clap(short, long)]
    pub text: String,
}

fn config_file() -> String {
    let home = env::var("HOME").ok().unwrap();
    let dobato_dir = PathBuf::from(home).join(".config/dobato");
    let dobato_config = dobato_dir.join("webhook.txt");
    dobato_config.to_str().unwrap().to_string()
}

#[tokio::main]
async fn main() {
    let args = CommandArgs::parse();
    match args.action {
        Action::Setup => {
            print!("webhook URL: ");
            stdout().flush().ok();

            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();

            let home = env::var("HOME").ok().unwrap();
            let dobato_dir = PathBuf::from(home).join(".config/dobato");
            fs::create_dir_all(&dobato_dir).ok();

            let config_file = config_file();
            let mut f = fs::File::create(&config_file).unwrap();
            f.write_all(buf.as_bytes()).ok();
            println!("Stored webhook to {}", config_file);
        }
        Action::Post(p) => {
            let config_file = config_file();

            if let Ok(f) = fs::File::open(config_file) {
                let mut f = io::BufReader::new(f);
                let mut buf = String::new();
                f.read_line(&mut buf).ok();

                let client = reqwest::Client::new();
                let mut payload = HashMap::new();
                payload.insert("content", p.text);

                let webhook = buf.trim_end();
                let response = client.post(webhook).json(&payload).send().await;

                match response {
                    Ok(response) => {
                        println!("Finished.");
                        println!("- status: {}", response.status());
                    }
                    Err(e) => println!("Failed with {:?}", e),
                };
            } else {
                println!("You need to setup dobato first: `dobato setup`");
            }
        }
    }
}
