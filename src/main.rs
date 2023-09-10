use std::error::Error;
use std::sync::Arc;

use ethers::abi::AbiEncode;
use ethers::core::utils::hex;
use ethers::prelude::*;
use rayon::prelude::*;
use sha2::Digest;

const USER_ADDRESS: &str = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"; // change this to your address
const SHA256_ADDRESS: &str = "0x0000000000000000000000000000000000000002";
const RPC_URI: &str = "https://eth.llamarpc.com";

// Generates the binding `SolveThePuzzleOfCoastWithImpressionInNightAndSquallOnCayAndEndToVictoryCall`
// Need to run `forge build` before `cargo build`.
abigen!(IKey, "./out/IKey.sol/IKey.json");

fn to_password(i: u128) -> [u8; 32] {
    let mut password = [0u8; 32];

    for (j, byte) in password.iter_mut().enumerate().take(16) {
        *byte = ((i >> ((15 - j) * 8)) & 0xFF) as u8;
    }

    password
}

fn abi_encode(owner: H160, password: [u8; 32]) -> Vec<u8> {
    SolveThePuzzleOfCoastWithImpressionInNightAndSquallOnCayAndEndToVictoryCall { owner, password }
        .encode()
}

fn get_client() -> Arc<Provider<Http>> {
    let provider: Provider<Http> = Provider::<Http>::try_from(RPC_URI).expect("Valid URL");
    Arc::new(provider)
}

fn get_address_two(client: &Arc<Provider<Http>>) -> IKey<Provider<Http>> {
    let address: Address = SHA256_ADDRESS.parse().expect("Valid address");
    IKey::new(address, Arc::clone(client))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let owner: Address = USER_ADDRESS.parse().expect("Valid address");

    // 0x00000000
    let sig = hex::decode("00000000").unwrap();
    let res = (0..u128::MAX).into_par_iter().find_any(|i| {
        let password = to_password(*i);
        let digest = sha2::Sha256::digest(&abi_encode(owner, password));
        digest.starts_with(&sig[..])
    });

    if let Some(i) = res {
        println!("i: {}", i);
        let password = to_password(i);
        println!("password: {:#?}", password);

        // perform an test call to keccak address
        let client = get_client();
        let hash_address = get_address_two(&client);
        let res = hash_address.solve_the_puzzle_of_coast_with_impression_in_night_and_squall_on_cay_and_end_to_victory(owner, password).call().await.unwrap();

        // Print the response
        println!("Response data: {:#?}", res);
    } else {
        println!("Crunching failed. Bigger range?");
    }
    Ok(())
}
