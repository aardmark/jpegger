use clap::Parser;
use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom};
use std::process::ExitCode;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    file_name: String,
    #[arg(short, long)]
    delete: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match is_corrupted_jpeg(&cli.file_name) {
        Ok(is_corrupted) => {
            if is_corrupted {
                println!("{} is corrupted.", &cli.file_name);
                if cli.delete {
                    match fs::remove_file(&cli.file_name) {
                        Ok(_) => {
                            println!("{} deleted.", &cli.file_name);
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            return ExitCode::FAILURE;
                        }
                    }
                }
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}

fn is_corrupted_jpeg(file_name: &String) -> Result<bool, io::Error> {
    let file_info = fs::metadata(file_name)?;
    if !file_info.is_file() || file_info.len() < 4 {
        return Ok(false);
    }

    let mut file = File::open(file_name)?;

    let mut buffer = [0; 2];
    file.read_exact(&mut buffer)?;
    if buffer[0] == 0xff && buffer[1] == 0xd8 {
        file.seek(SeekFrom::End(-2))?;
        file.read_exact(&mut buffer)?;
        if !(buffer[0] == 0xff && buffer[1] == 0xd9) {
            return Ok(true);
        }
    }
    Ok(false)
}
