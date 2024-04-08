use base::to_base10;
use num_bigint_dig::BigUint;
use std::{io::Write, process::exit, str::FromStr};

mod millers;
mod base;
mod generate;
mod mainutil;

use clap::{Parser, Subcommand};
use clio::*;

use crate::{base::from_base10, mainutil::{parse_input_group, read_key, split_string_at_n}};


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
        #[clap(flatten)]
        group: InputArgGroup,

        #[clap(short, long, default_value="-")]
        output_file: Output,

        #[clap(short='P', long, default_value="./private.txt")]
        privkey: Input
    }
}


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
            group, 
            output_file, 
            privkey 
        } => decrypt(group, output_file, privkey)
    }
}

fn encrypt(input: InputArgGroup, mut output: Output, pubkey:Input) {

    //Have to do some matching to get the inpu
    let input_string = parse_input_group(input);

    //Split the string every 215 chars into a vec of strings
    let input_string_vec = split_string_at_n(215, input_string);

    let mut input_vec = Vec::new();
    
    for i in input_string_vec {
        let input_as_int = to_base10(&i, ".,?! \t\n\rabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
        input_vec.push(input_as_int);
    }

    //Parse pubkey
    let pubkey_text = read_key(pubkey);

    let keys = Vec::from_iter(pubkey_text.split('\n').into_iter());
    assert_eq!(keys.len(), 2);
    let n_string = keys.first().unwrap().to_string();
    let e_string = keys.last().unwrap().to_string();

    let n_res = BigUint::from_str(&n_string);
    let e_res = BigUint::from_str(&e_string);

    let e;
    let n;

    match n_res {
        Ok(i) => {
            n = i;
        }
        Err(e) => {
            panic!("Could not parse n from the provided pubkey file! Error: {e}");
        }
    }
    
    match e_res {
        Ok(i) => {
            e = i;
        }
        Err(e) => {
            panic!("Could not parse e from the provided pubkey file! Error: {e}");
        }
    }

    //Actually encrypt
    let mut encrypted = String::new();

    
    
    for block in input_vec {
        let out = block.modpow(&e, &n);
        let new_string = from_base10(out, ".,?! \t\n\rabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
        encrypted.push_str(new_string.as_str());
        encrypted.push('$');
    }

    let res = output.write(encrypted.to_string().as_bytes());
    match res {
        Ok(u) => {
            if output.path().to_string() == "\"-\"" {
                exit(0);
            } else {
                println!("Wrote {u} bytes to output file.");
                exit(0);
            }
        },
        Err(e) => {
            println!("Failed to write to output file. Error: {e}");
        }
    }

}


fn decrypt(input: InputArgGroup, mut output_file: Output, privkey: Input){ 
    let input_string = parse_input_group(input);

    let privkey_text = read_key(privkey);

    let keys = Vec::from_iter(privkey_text.split('\n').into_iter());
    assert_eq!(keys.len(), 2);
    let n_string = keys.first().unwrap().to_string();
    let d_string = keys.last().unwrap().to_string();

    let n_res = BigUint::from_str(&n_string);
    let d_res = BigUint::from_str(&d_string);

    let d;
    let n;

    match n_res {
        Ok(i) => {
            n = i;
        }
        Err(e) => {
            panic!("Could not parse n from the provided pubkey file! Error: {e}");
        }
    }
    
    match d_res {
        Ok(i) => {
            d = i;
        }
        Err(e) => {
            panic!("Could not parse e from the provided pubkey file! Error: {e}");
        }
    }

    let mut decrypted_string = String::new();

    let input_vec: Vec<&str> = input_string.split('$').collect();

    for s in input_vec {
        if s.len() != 0 {
            let as_base_10 = to_base10(s, ".,?! \t\n\rabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
            let decrypted = as_base_10.modpow(&d, &n);
            let decrypted_as_text = from_base10(decrypted, ".,?! \t\n\rabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
            decrypted_string.push_str(&decrypted_as_text);
        }
    }


    let res = output_file.write(format!("{decrypted_string}").as_bytes());
    match res {
        Ok(r) => {
            if output_file.path().to_string() == "\"-\"" {
                exit(0);
            } else {
                println!("Wrote {r} bytes to output file.");
                exit(0);
            }
        }
        Err(e) => {
            panic!("Failed to write to the output. Error: {}", e);
        }
    }
}