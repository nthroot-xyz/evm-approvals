
use std::{env};
use std::io;
use std::io::Write;

use rusqlite::{Connection};
use alloy_primitives::{Address, U256};
use alloy_provider::ProviderBuilder;
use alloy_sol_types::sol;

use rotki::{Location, config::default_directory};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ERC20,
    "erc20.json"
);

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    contract_address: String,

}


#[derive(Debug)]
struct Approval{
    contract_address: Address,
    location_label: Address,
    chain: Location,
    asset: Address,
}

async fn query_approval_amount(
    contract_addr: Address,
    asset_addr: Address,
    user_addr: Address,
    location: Location
) -> U256 {
    let rpc_url = match location {
        Location::ETHEREUM => "https://eth.merkle.io",
        Location::ARBITRUMONE => "https://arbitrum.llamarpc.com",
        Location::BASE => "https://base.llamarpc.com",
        Location::GNOSIS => "https://gnosis.drpc.org",
        Location::OPTIMISM => "https://optimism.llamarpc.com",
        Location::SCROLL => "https://rpc.scroll.io",
        _ => ""
    };

    if rpc_url.is_empty(){
        return U256::ZERO
    }

    let provider = ProviderBuilder::new().on_http(rpc_url.parse().unwrap());
    let contract = ERC20::new(asset_addr, provider);
    
    contract.allowance(user_addr, contract_addr).call().await.unwrap()._0
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // ask for user information
    print!("Enter username: ");
    std::io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");
    let password = rpassword::prompt_password("Enter db password: ").unwrap();

    // connect to the database
    let path_db = default_directory().join("users").join(username.trim()).join("rotkehlchen.db");
    let conn_setup = Connection::open(path_db);
    let conn = match conn_setup {
        Ok(conn) => conn,
        Err(e) => panic!("Error opening db: {:?}", e)
    };
    conn.prepare(format!(r#"PRAGMA KEY="{}""#, password.replace('"', r#""""#)).as_str()).unwrap();

    // query the database for the approvals
    let address = Address::parse_checksummed("0xc37b40ABdB939635068d3c5f13E7faF686F03B65", None).unwrap();
    let mut stmt = conn.prepare(r#"SELECT address, location, location_label, asset FROM evm_events_info JOIN history_events ON
        evm_events_info.identifier=history_events.identifier WHERE type="informational"
        AND subtype="approve" AND location_label=?1 AND address=?2
    "#).unwrap();
    
    let target = &args[1];
    let approval_iter = stmt.query_map([address.to_string(), target.clone()], |row| {
        Ok(Approval{
            contract_address: Address::parse_checksummed(row.get::<usize, String>(0).unwrap(), None).unwrap(),
            location_label: Address::parse_checksummed(row.get::<usize, String>(2).unwrap(), None).unwrap(),
            chain: Location::from(row.get::<usize, String>(1).unwrap()),
            asset:  Address::parse_checksummed(row.get::<usize, String>(3).unwrap().split(':').last().unwrap(), None).unwrap(),
        })
    }).unwrap();

    // check on chain the approval values
    for approval_opt in approval_iter {
        if approval_opt.is_err() {
            continue;
        }
        let approval = approval_opt.unwrap();
        // println!("target: {:?}", target);
        // println!("{:?}", approval);
        println!("Approval of token {:?} at {:?} set to {:?} for {:?} ",
            approval.asset,
            approval.chain.clone(),
            query_approval_amount(
                approval.contract_address,
                approval.asset,
                approval.location_label,
                approval.chain,
            ).await,
            approval.contract_address,
        );
    }
}
