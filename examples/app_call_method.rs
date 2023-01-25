use algonaut::atomic_transaction_composer::{AtomicTransactionComposer, AddMethodCallParams, transaction_signer::TransactionSigner::BasicAccount};
use algonaut::transaction::{account::Account,
    transaction::ApplicationCallOnComplete::NoOp,
};
use algonaut_transaction::builder::TxnFee::Fixed;

use algonaut_abi::abi_interactions::AbiReturnType::Void;
//
//use algonaut::atomic_transaction_composer::AbiMethodReturnValue::Void;

use algonaut_abi::abi_interactions::AbiReturn;

use algonaut_abi::abi_interactions::{AbiMethod,AbiMethodArg};


use algonaut::core::{CompiledTeal, MicroAlgos};
use algonaut_crypto::HashDigest;
use algonaut::algod::v2::Algod;

use std::error::Error;
#[macro_use]

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
 
 let url = String::from("https://node.testnet.algoexplorerapi.io");
 
 let user = String::from("User-Agent");
 let pass = String::from("DoYouLoveMe?");
 let headers :  Vec<(&str, &str)> = vec![(&user, &pass)];

 let algod = Algod::with_headers(&url, headers)?;
 
 let acct1 = Account::from_mnemonic("degree feature waste gospel screen near subject boost wreck proof caution hen adapt fiber fault level blind entry also embark oval board bunker absorb garage
")?;
 
 println!("retrieving suggested params");
 let params = algod.suggested_transaction_params().await?;
 
 
 let val = String::from("");
 let pages: u32 = 0;
  
 let method_name1 : Option<String> = Some("amount".to_string());
 let method_name2 : Option<String> = Some("account".to_string());
 let description1 : Option<String> = None;
 let description2 : Option<String> = None;
    
 //should ideally read from .json file
 let _method : AbiMethod = AbiMethod{
     name: String::from("withdraw"),
     description: description1,
     args: vec![AbiMethodArg{name: method_name1, description: description1,}, 
         AbiMethodArg{name: method_name2, description: description2,},
     ],
     returns: AbiReturn::type_(Void),
    }; 
 let arg1 : u64 = 0;
 let arg2 : u64 = 1000;//= &acct1.address();
    
 let _note : Option<Vec<u8>> = Some(vec![0]);
 
 
let mut ATC = AtomicTransactionComposer::add_method_call( &mut self , &mut AddMethodCallParams{
    app_id: 155672004, method: _method, method_args: [arg1, arg2], fee:  Fixed(MicroAlgos(2500)), sender: acct1.address(), suggested_params: params, on_complete: NoOp,
    approval_program: None, clear_program: None, global_schema: None, local_schema: None, extra_pages: pages, 
    note: _note, lease: None, rekey_to: None, signer: BasicAccount(acct1)
    });
    
//println!("{}",&mut AtomicTransactionComposer);
//AtomicTransactionComposer::build_group(&mut ATC.unwrap());
 
//AtomicTransactionComposer::execute( &mut ATC ,&algod);
Ok(())
 
 
}