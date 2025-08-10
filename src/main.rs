use std::fs;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::process::ExitCode;
use std::{env, fs::File};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: jpegger filename");
        std::process::exit(1);
    }

    let delete_it: bool = false;
    match is_corrupted_jpeg(&args[1]) {
        Ok(is_corrupted) => {
            if is_corrupted {
                println!("{}", &args[1]);
                if delete_it {
                    fs::remove_file(&args[1]);
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
