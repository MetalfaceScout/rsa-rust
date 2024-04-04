use num_bigint_dig::{BigInt, BigUint, ModInverse, ToBigInt, ToBigUint};
use num_traits::Pow;
use std::{fs::File, io::{Read, Write}, string};

mod millers;
mod base;
//mod inverse; -- Maybe

use clap::{Parser, Subcommand};
use clio::*;
#[derive(Parser,Debug)]
#[clap(author="Logan Mathis", version, about="Use Bart's RSA algorithm to encrypt and decrypt messages.")]
struct Arguments {
    #[clap(subcommand)]
    command: SubCommand,
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
        #[clap(short, long)]
        file: Option<Input>,

        #[clap(short, long)]
        input: Option<String>,

        #[clap(short='P', long)]
        privkey_path: Option<Input>
    },

    Decrypt {
        #[clap(short, long)]
        file: Option<Input>,

        #[clap(short, long)]
        input: Option<String>,

        #[clap(short='p', long)]
        pubkey_path: Option<Input>
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
        } => generate_keys(key_directory, file, input_string_1, input_string_2),
        SubCommand::Decrypt { 
            file, 
            input, 
            pubkey_path 
        } => (),
        SubCommand::Encrypt { 
            file, 
            input, 
            privkey_path 
        } => (),
    }

    


}

fn generate_keys(
    key_dir: Option<ClioPath>, 
    file: Option<Input>, 
    input_string_1: Option<String>, 
    input_string_2: Option<String>) 
{
    let mut string_1 = String::new();
    let mut string_2 = String::new();

    let strings_parsed;

    match file {
        None => strings_parsed = false,
        Some(mut f) => {
            let mut buf = String::new();
            let res = f.read_to_string(&mut buf);
            let mut _bytes_read = 0;
            match res {
                Ok(b) => _bytes_read = b,
                Err(e) => panic!("Unable to read file: {e}")
            }
            let keys = Vec::from_iter(buf.split('\n').into_iter());
            assert_eq!(keys.len(), 2);
            string_1 = keys.first().unwrap().to_string();
            string_2 = keys.last().unwrap().to_string();
            strings_parsed = true;
        }
    }

    if strings_parsed == false {
        match input_string_1 {
            None => {panic!("Input string one does not exist, and a file was not passed!")},
            Some(s) => {
                string_1 = s;
            }
        }
        match input_string_2 {
            None => {panic!("Input string two does not exist, and a file was not passed!")},
            Some(s) => {
                string_2 = s;
            }
        }
    }

    let pubkey_file;
    let privkey_file;

    match key_dir {
        Some(mut d) => {
            pubkey_file = d.join("public.txt")
            .create()
            .unwrap_or(Output::std_err());
            privkey_file = d.join("private.txt")
            .create()
            .unwrap_or(Output::std_err());
        }
        None => {
            let res = Output::new("./public.txt");
            match res {
                Ok(p) => {
                    pubkey_file = p;
                },
                Err(e) => {panic!("Unable to create local public.txt: {e}")} 
            }
            
            let res = Output::new("./private.txt");
            match res {
                Ok(p) => {
                    privkey_file = p;
                },
                Err(e) => {panic!("Unable to create local public.txt: {e}")} 
            }
        }
    }
    
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

    let mut string_1_base_10 = base::to_base10(&string_1, ALPHABET);
    let mut string_2_base_10 = base::to_base10(&string_2, ALPHABET);

    if string_1_base_10.clone() % BigUint::from(2u8) == BigUint::from(0u8) {
        string_1_base_10 += BigUint::from(1u8);
    }
    if string_2_base_10.clone() % BigUint::from(2u8) == BigUint::from(0u8) {
        string_2_base_10 += BigUint::from(1u8);
    }

    while millers::is_prime_miller(&string_1_base_10) == false {
        string_1_base_10 += BigUint::from(2u8);
    }
    while millers::is_prime_miller(&string_2_base_10) == false {
        string_2_base_10 += BigUint::from(2u8);
    }

    let p: BigUint = string_1_base_10.clone();
    let q: BigUint = string_2_base_10.clone();


    let ten_to_200 = 10u8.to_biguint().unwrap().pow(200u8);

    if q < ten_to_200 || p < ten_to_200 {
        panic!("Input strings are too short");
    }

    let n: BigUint = p.clone() * q.clone();
    let r: BigUint = (p.clone() - BigUint::from(1u8)) * (q.clone() - BigUint::from(1u8));

    const E: u32 = 65537;

    let d = BigUint::from(E).mod_inverse(r).unwrap();

    let res = write_to_output(pubkey_file, n.to_bigint().unwrap(), E.to_bigint().unwrap());
    match res {
        Ok(_) => (),
        Err(e) => {panic!("Could not write output: {e}")}
    }

    let res = write_to_output(privkey_file, n.to_bigint().unwrap(), d);
    match res {
        Ok(_) => (),
        Err(e) => {panic!("Could not write output: {e}")}
    }

}

//RSA!
fn write_to_output(mut file: Output, a: BigInt, b: BigInt) -> std::io::Result<()> {
    file.write(format!("{a}\n").as_bytes())?;
    file.write(format!("{b}").as_bytes())?;
    Ok(())
}

