use algonaut::algod::v2::Algod;
use algonaut::transaction::account::Account;
use algonaut::transaction::builder::DeleteApplication;
use algonaut::transaction::TxnBuilder;
use dotenv::dotenv;

use std::error::Error;

extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let url = String::from("https://node.testnet.algoexplorerapi.io");
    //let token = String::from("");
    
    let alice_mnemonic = String::from("tank game arrive train bring taxi tackle popular bacon gasp tell pigeon error step leaf zone suit chest next swim luggage oblige opinion about execute");
   
    let user = String::from("User-Agent");
    let pass = String::from("DoYouLoveMe?");
    let headers :  Vec<(&str, &str)> = vec![(&user, &pass)];
    
    
    println!("creating algod client");
    let algod = Algod::with_headers(&url, headers)?;

    println!("creating account for alice");
    let alice = Account::from_mnemonic(&alice_mnemonic)?;

    println!("retrieving suggested params");
    let params = algod.suggested_transaction_params().await?;

    println!("building DeleteApplication transaction");
    
    
    let app_id : u64 = 116639568;
    let app_arg : Option<String> = Some(String::from("inc"));
    
    let t = TxnBuilder::with(
        &params,
        DeleteApplication::new(alice.address(), app_id)
                        .app_arguments(vec![app_arg.expect("REASON").into_bytes()])
                        .build(),
    )
    .build()?;

    println!("signing transaction");
    println!("{:?}",t);
    
    let signed_t = alice.sign_transaction(t)?;

    println!("broadcasting transaction");
    let send_response = algod.broadcast_signed_transaction(&signed_t).await?;
    println!("response: {:?}", send_response);

    Ok(())
}
