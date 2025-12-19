use solana_sdk::pubkey::Pubkey;

pub const RAYDIUM: &str = "9W957wfaKvq2mdCA2kweFBBYeCHryan1QX3meXQawUo";
pub const ORCA: &str = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";
pub const JUPITER: &str = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";
pub const DRIFT: &str = "dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH";
pub const KAMINO: &str = "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK";
pub const CPAMDP: &str = "cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG";

pub fn is_dex_program(program_id: &Pubkey) -> Option<String> {
    let program_str = program_id.to_string();

    match program_str.as_str() {
        RAYDIUM => Some("Raydium".to_string()),
        ORCA => Some("Orca".to_string()),
        JUPITER => Some("Jupiter".to_string()),
        DRIFT => Some("Drift".to_string()),
        KAMINO => Some("Kamino".to_string()),
        CPAMDP => Some("Cpamdp".to_string()),
        _ => None,
    }
}
