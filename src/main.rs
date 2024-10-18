use bip39::{Language, Mnemonic};
use fuels::prelude::*;
use tokio::runtime::Runtime;

fn generate_wallet_info() -> (String, String, String, String) {
    let mnemonic = Mnemonic::generate_in(Language::English, 12).unwrap();
    let mnemonic_phrase: String = mnemonic.words().collect::<Vec<&str>>().join(" ");

    let rt = Runtime::new().unwrap();

    let (bech32_address, fuel_address_hex, private_key) = rt.block_on(async {
        let provider = setup_test_provider(vec![], vec![], None, None)
            .await
            .unwrap();

        let wallet = WalletUnlocked::new_from_mnemonic_phrase_with_path(
            &mnemonic_phrase,
            Some(provider.clone()),
            "m/44'/1179993420'/0'/0/0",
        )
        .unwrap();

        let bech32_address = wallet.address().to_string();

        let address_hash = wallet.address().hash();
        let fuel_address_bytes = address_hash.as_ref();
        let fuel_address_hex = hex::encode(fuel_address_bytes);

        let private_key_str = format!("{:?}", wallet);
        let private_key = if let Some(private_key_start) = private_key_str.find("private_key: ") {
            private_key_str[private_key_start + 13..private_key_start + 77].to_string()
        } else {
            String::new()
        };

        (bech32_address, fuel_address_hex, private_key)
    });

    (
        mnemonic_phrase,
        bech32_address,
        fuel_address_hex,
        private_key,
    )
}

fn main() {
    let (mnemonic_phrase, bech32_address, fuel_address_hex, private_key) = generate_wallet_info();

    println!("Generated BIP-39 seed phrase: {}\n", mnemonic_phrase);
    println!("Fuel wallet Bech32 address: {}\n", bech32_address);
    println!("Fuel wallet hex address: 0x{}\n", fuel_address_hex);
    println!("Fuel wallet private key: 0x{}\n", private_key);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_wallet_info() {
        let (mnemonic_phrase, bech32_address, fuel_address_hex, private_key) =
            generate_wallet_info();

        assert!(
            !mnemonic_phrase.is_empty(),
            "Mnemonic phrase should not be empty."
        );

        assert!(
            bech32_address.starts_with("fuel"),
            "Bech32 address should start with 'fuel'."
        );

        assert!(
            !fuel_address_hex.is_empty(),
            "Fuel hex address should not be empty."
        );

        assert!(!private_key.is_empty(), "Private key should not be empty.");
    }
}
