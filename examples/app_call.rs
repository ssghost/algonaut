use algonaut::algod::v2::Algod;
use algonaut::transaction::account::Account;
use algonaut::transaction::builder::CallApplication;
use algonaut::transaction::TxnBuilder;
use dotenv::dotenv;
use dict::{ Dict, DictIface };

use std::error::Error;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let url = String::from("https://node.testnet.algoexplorerapi.io");
    let token = String::from("");
    
    let alice_mnemonic = String::from("tank game arrive train bring taxi tackle popular bacon gasp tell pigeon error step leaf zone suit chest next swim luggage oblige opinion about execute");
    let mut headers = Dict::<String>::new(); 
    assert_eq!( headers.add( "User-Agent".to_string(), "DoYouLoveMe?".to_string() ), true );
    
    //= {'User-Agent': 'DoYouLoveMe?}?;
    
    info!("creating algod client");
    //let algod = Algod::new(&url, &token)?;
    let algod = Algod::with_headers(&url, &headers)
    
    info!("creating account for alice");
    let alice = Account::from_mnemonic(&alice_mnemonic)?;

    info!("retrieving suggested params");
    let params = algod.suggested_transaction_params().await?;

    info!("building transaction");
    let t = TxnBuilder::with(
        &params,
        CallApplication::new(alice.address(), 3)
            .app_arguments(vec![vec![1, 0], vec![255]])
            .build(),
    )
    .build()?;

    info!("signing transaction");
    let signed_t = alice.sign_transaction(t)?;

    info!("broadcasting transaction");
    let send_response = algod.broadcast_signed_transaction(&signed_t).await?;
    info!("response: {:?}", send_response);

    Ok(())
}
