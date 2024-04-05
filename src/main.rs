use base::to_base10;
use std::io::Read;

mod millers;
mod base;
mod generate;
//mod inverse; -- Maybe

use clap::{Parser, Subcommand};
use clio::*;


#[derive(Parser,Debug)]
#[clap(author="Logan Mathis", version, about="Use Bart's RSA algorithm to encrypt and decrypt messages.")]
struct Arguments {
    #[clap(subcommand)]
    command: SubCommand,
}


#[derive(Debug, clap::Args)]
#[group(required=true, multiple=false)]
struct InputArgGroup {
    
    #[clap(short, long)]
    file: Option<Input>,

    #[clap(short, long)]
    input: Option<String>
}

#[derive(Subcommand, Debug)]
enum SubCommand {

    /// Generate keys required to encrypt. By default, keys go next to the executable.
    GenerateKeys {

        /// Specify a directory to put the keys.
        #[clap(short='d', long)]
        key_directory: Option<ClioPath>,
        
        /// Specify a file for the input strings. They're separated by a newline.
        #[clap(short, long)]
        file: Option<Input>,
        
        /// The first string, enclosed in quotes.
        #[clap(requires="input_string_2")]
        input_string_1: Option<String>,

        /// The second string, enclosed in quotes.
        #[clap(requires="input_string_1")]
        input_string_2: Option<String>
    },

    Encrypt {
        /// Specify a file to read from. Defaults to stdin.
        #[clap(flatten)]
        group: InputArgGroup,

        /// Specift a file to write the encrypted output to. Defaults to stdout.
        #[clap(short, long, default_value="-")]
        output_file: Output,

        /// Specify a private key to use for encrypting. Defaults to "./private.txt"
        #[clap(short='p', long, default_value="./public.txt")]
        pubkey: Input
    },

    Decrypt {
        #[clap(short, long, default_value="-")]
        input_file: Input,

        #[clap(short, long, default_value="-")]
        output_file: Output,

        #[clap(short='P', long, default_value="./private.txt")]
        privkey: Input
    }
}
//rs-rsa generate-keys <optional dir for private and publickey>
//rs-rsa encrypt -f <file> <use default path for pub and private key>
//rs-rsa encrypt "Thing to encrypt" -p <pubkey file path> -P <private file path>
//rs-rsa decrypt -f <file> -P <private-key>
//rs-rsa decrype --file <file> --pub-key <path to pubkey>


fn main() {
    let arg = Arguments::parse();

    match arg.command {
        SubCommand::GenerateKeys { 
            key_directory,
            file,
            input_string_1,
            input_string_2 
        } => generate::generate_keys(key_directory, file, input_string_1, input_string_2),
        SubCommand::Encrypt { 
            group,
            output_file, 
            pubkey 
        } => encrypt(group, output_file, pubkey),
        SubCommand::Decrypt { 
            input_file, 
            output_file, 
            privkey
        } => (),
    }
}

fn encrypt(input: InputArgGroup, output: Output, mut pubkey:Input) {
    //if we don't have text, we must have a file

    let input_string;

    match input.file {
        Some(mut f) => {
            let mut string_buf = String::new();
            let res = f.read_to_string(&mut string_buf);
            match res {
                Ok(_) => input_string = string_buf,
                Err(e) => {panic!("Failed to read input, either specify a file or include input on stdin. Error: {e}")}
            }
        }
        None => {
            match input.input {
                Some(s) => {
                    input_string = s;
                }
                None => {
                    panic!("Somehow, there was no file or input specified.");
                }
            }
        }
    }
    
    let input_as_int = to_base10(&input_string, ".,?! \t\n\rabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");

    if input_as_int.to_string().len() > 216 {
        panic!("Not implemented")
        // TODO: split into blocks 
    }

    let mut pubkey_text = String::new();
    let res = pubkey.read_to_string(&mut pubkey_text);
    match res {
        Ok(_) => {}
        Err(e) => {
            panic!("Could not read from pubkey file!\n{e}")
        }
    }

    let keys = Vec::from_iter(pubkey_text.split('\n').into_iter());
    assert_eq!(keys.len(), 2);
    let n = keys.first().unwrap().to_string();
    let e = keys.last().unwrap().to_string();



}



//RSA!

