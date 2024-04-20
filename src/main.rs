use std::error::Error;
use std::{env, process};

use openidconnect::reqwest::http_client;
use openidconnect::{core::CoreProviderMetadata, IssuerUrl};
fn main() {
    let issuer_key = "OIDC_ISSUER";

    let issuer = match env::var(issuer_key) {
        Ok(val) => val,
        Err(err) => {
            println!("Error: {}:{}", err, issuer_key);
            process::exit(1);
        }
    };
    println!("Issuer is {}", issuer);
    let s = oidc(issuer);

    println!("{:?}", s)
}

fn oidc(issuer: String) -> Result<(), Box<dyn Error>> {
    let provider_metadata = CoreProviderMetadata::discover(&IssuerUrl::new(issuer)?, http_client)?;
    print!("{:?}", provider_metadata);

    Ok(())
}
