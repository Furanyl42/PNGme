use crate::args::*;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use anyhow::Result;
use std::fs;
use std::os::unix::process;

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Encode(args) => {
            println!("Encoding...");
            encode(args)?;
            println!("Done.");
            Ok(())
        }
        Commands::Decode(args) => {
            decode(args)?;
            Ok(())
        }
        Commands::Remove(args) => {
            println!("Removing...");
            remove(args)?;
            println!("Done");
            Ok(())
        }
        Commands::Print(args) => {
            print(args)?;
            Ok(())
        }
    }
}

fn encode(args: EncodeArgs) -> Result<()> {
    let content: Vec<u8> = fs::read(args.file_path.as_path())?;
    let mut png: Png = Png::try_from(content.as_slice())?;

    let message_bytes = args.message.as_bytes();

    let chunk = Chunk::new(args.chunk_type, message_bytes.to_vec());
    let output_file = match args.output_file {
        Some(path) => path,
        None => args.file_path,
    };

    png.append_chunk(chunk);
    fs::write(output_file, png.as_bytes())?;
    Ok(())
}

fn decode(args: DecodeArgs) -> Result<()> {
    let content: Vec<u8> = fs::read(args.file_path.as_path())?;
    let mut png: Png = Png::try_from(content.as_slice())?;

    if let Some(chunk) = png.chunk_by_type(&args.chunk_type.to_string()) {
        println!("{}", str::from_utf8(chunk.data())?);
    } else {
        println!("Not found");
    }
    Ok(())
}

fn remove(args: RemoveArgs) -> Result<()> {
    let content: Vec<u8> = fs::read(args.file_path.as_path())?;
    let mut png: Png = Png::try_from(content.as_slice())?;
    png.remove_first_chunk(&args.chunk_type.to_string());
    fs::write(args.file_path, png.as_bytes())?;
    Ok(())
}

fn print(args: PrintArgs) -> Result<()> {
    let content: Vec<u8> = fs::read(args.file_path.as_path())?;
    let png: Png = Png::try_from(content.as_slice())?;
    println!("{}", png);
    Ok(())
}
