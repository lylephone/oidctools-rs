use std::{env, process};

fn main() {

    let issuer_key = "OIDC_ISSUER";

    let issuer = match env::var(issuer_key) {
        Ok(val) =>val,
        Err(err) => {
            println!("Error: {}:{}",err,issuer_key);
            process::exit(1);
        },

    };
    println!("Issuer is {}",issuer);
    println!("Hello, world!");
}
