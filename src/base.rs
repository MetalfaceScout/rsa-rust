use num_bigint_dig::{BigUint, ToBigUint};
use num_traits::pow::Pow;

//Returns a string with everything not in the included alphabet parsed out
fn parse_string(input: &str, alphabet: &str) -> String {

    //Setup return value
    let mut new_string = String::new();
    
    //If our unwrap fails, an impossibly large alphabet value will mark it
    const ERROR_VALUE: usize = 0xfcedfced; 

    for (i, char) in input.to_string().chars().enumerate() {
        //Find the char given in the alphabet, otherwise index will be our magic error
        let index = alphabet.find(char).unwrap_or(ERROR_VALUE);
        if index == ERROR_VALUE {
            //Skip the loop if we couldn't find the number
            continue;
        }
        //If we did find a value, push it on to the new string as a lowercase value
        new_string.push(input.chars().nth(i).unwrap());
    }

    return new_string;
} 


pub fn to_base10(input: &str, alphabet: &str) -> BigUint {

    //Parse everything out of the string that's not in the alphabet
    let new_string = parse_string(input, alphabet);

    //Setup our adder
    let mut output: BigUint = 0.to_biguint().unwrap();

    //"Calculate" our base off of the alphabet
    let base = alphabet.len();

    for (i, char) in new_string.chars().enumerate() {
        
        //Get the index of the digit in the alphabet, ex b -> [abc] -> returns 1
        let digit = alphabet.find(char).unwrap();

        //Calculate how many times we need to exponentiate the base (string_len - 1 - iterations)
        let exponent: u32 = ((new_string.len()-1) - i).try_into().unwrap();

        //Exponentiate the base
        let multiplier: BigUint = base.to_biguint().unwrap().pow(exponent);

        //Add the digit to the multipler, then add our final value to the total
        output += multiplier * digit.to_biguint().unwrap();
    }

    return output;
}

pub fn from_base10(input: BigUint, alphabet: &str) -> String {

    let mut output = String::new();
    
    let base = alphabet.len();
    let mut q = input.clone();
    while q.clone() != BigUint::from(0u8) {
        let a_of_k_bigint = &q % BigUint::from(base);
        let a = a_of_k_bigint.to_string().parse::<usize>().unwrap();
        q = &q / BigUint::from(base);
        let new_char = alphabet.chars().nth(a).unwrap();
        output.push(new_char);
    }

    return output.chars().rev().collect::<String>();
}

#[test]
fn from_base10_decrypted_strings() {
    use std::str::FromStr;

    let al = ".,?! \t\n\rabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    let n1 = BigUint::from(291218903875553u64);
    let str1 = from_base10(n1, al);
    assert_eq!(str1, "Brother!".to_string());

    let n2 = BigUint::from_str("772543081451379805830512070459312014814237279158624631020").unwrap();
    let str2 = from_base10(n2, al);
    assert_eq!(str2, "All your base are belong to us.".to_string());
}

#[test]
fn parse_string_does_that() {
    let al = ".,?! \t\n\rabcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let unparsed = "This should be here, and this %^%%#$##$ should not.";

    let parsed = parse_string(unparsed, al);
    println!("{parsed}");
    assert_eq!(parsed, "This should be here, and this  should not.");
}


#[test]
fn to_base10_1_to_million() {
    for i in 0..(1<<10) {
        let output = to_base10(format!("{}",i).as_str(), "0123456789");
        assert_eq!(output, i.to_biguint().unwrap());
    }
}

#[test]
fn to_base10_check_alphabet() {
    for i in 0..9 {
        let output = to_base10(format!("{}",i).as_str(), i.to_string().as_str());
        assert_eq!(output, 0.to_biguint().unwrap());
    }
}
