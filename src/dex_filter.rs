use solana_sdk::pubkey::Pubkey;

// https://drift-labs.github.io/v2-teacher/#terms-of-use
pub const DRIFT_V2: &str = "dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH";

// https://docs.raydium.io/raydium/protocol/developers/addresses
pub const RAYDIUM_CPMM: &str = "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C";
pub const RAYDIUM_LEGACY_AMM: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
pub const RAYDIUM_CLMM: &str = "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK";
pub const RAYDIUM_STABLE: &str = "5quBtoiQqxF9Jv6KYKctB59NT3gtJD2Y65kdnB1Uev3h";
pub const RAYDIUM_ROUTING: &str = "routeUGWgWzqBWFcrCfv8tritsqukccJPu3q5GPP3xS";

// https://dev.orca.so/Architecture%20Overview/Whirlpool%20Parameters/
pub const ORCA_WHIRLPOOL: &str = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";

// https://dev.jup.ag/get-started
pub const JUPITER_SWAP: &str = "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4";
pub const JUPITER_PERPETUALS: &str = "PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu";
pub const JUPITER_REFERRAL: &str = "REFER4ZgmyYx9c6He5XfaTMiGfdLwRnkV4RPp9t9iF3";
pub const JUPITER_DOVES: &str = "DoVEsk76QybCEHQGzkvYPWLQu9gzNoZZZt3TPiL597e";
pub const JUPITER_LEND_EARN: &str = "jup3YeL8QhtSx1e253b2FDvsMNC87fDrgQZivbrndc9";
pub const JUPITER_LEND_BORROW: &str = "jupr81YtYssSyPt8jbnGuiWon5f6x9TcDEFxYe3Bdzi";
pub const JUPITER_LEND_EARN_REWARDS: &str = "jup7TthsMgcR9Y3L277b8Eo9uboVSmu1utkuXHNUKar";
pub const JUPITER_LEND_LIQUIDITY: &str = "jupeiUmn818Jg1ekPURTpr4mFo29p46vygyykFJ3wZC";
pub const JUPITER_LEND_BORROW_ORACLE: &str = "jupnw4B6Eqs7ft6rxpzYLJZYSnrpRgPcr589n5Kv4oc";
pub const JUPITER_LIMIT_ORDER_V2: &str = "j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X";
pub const JUPITER_DCA: &str = "DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M";
pub const JUPITER_LOCK: &str = "LocpQgucEQHbqNABEYvBvwoxCPsSbG91A1QaQhQQqjn";
pub const JUPITER_GOVERNANCE: &str = "GovaE4iu227srtG2s3tZzB4RmWBzw8sTwrCLZz7kN7rY";
pub const JUPITER_VOTER: &str = "voTpe3tHQ7AjQHMapgSue2HJFAh2cGsdokqN3XqmVSj";

// https://docs.kamino.finance/build-on-kamino/sdk-and-smart-contracts
pub const KAMINO_LEND: &str = "GzFgdRJXmawPhGeBsyRCDLx4jAKPsvbUqoqitzppkzkW";
pub const KAMINO_LIQUIDITY: &str = "E35i5qn7872eEmBt15e5VGhziUBzCTm43XCSWvDoQNNv";
pub const KAMINO_VAULTS: &str = "Cyjb5r4P1j1YPEyUemWxMZKbTpBiyNQML1S1YpPvi9xE";

// https://docs.meteora.ag/developer-guide/home
// Meteora - Active Programs
pub const METEORA_DLMM: &str = "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo";
pub const METEORA_DAMM_V2: &str = "cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG";
pub const METEORA_DBC: &str = "dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN";
pub const METEORA_PRESALE_VAULT: &str = "presSVxnf9UU8jMxhgSMqaRwNiT36qeBdNeTRKjTdbj";
pub const METEORA_ALPHA_VAULT: &str = "vaU6kP7iNEGkbmPkLmZfGwiGxd4Mob24QQCie5R9kd2";
pub const METEORA_DYNAMIC_FEE_SHARING: &str = "dfsdo2UqvwfN8DuUVrMRNfQe11VaiNoKcMqLHVvDPzh";
pub const METEORA_ZAP: &str = "zapvX9M3uf5pvy4wRPAbQgdQsM1xmuiFnkfHKPvwMiz";

// Meteora - Legacy Programs
pub const METEORA_DAMM_V1: &str = "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB";
pub const METEORA_DYNAMIC_VAULT: &str = "24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi";
pub const METEORA_STAKE2EARN: &str = "FEESngU3neckdwib9X3KWqdL7Mjmqk9XNp3uh5JbP4KP";
pub const METEORA_FARM: &str = "FarmuwXPWXvefWUeqFAa5w6rifLkq5X6E8bimYvrhCB1";
pub const METEORA_MERCURIAL_STABLE_SWAP: &str = "MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky";

// https://solana.stackexchange.com/questions/16485/pump-dot-fun-official-program-ids
pub const PUMP_PROGRAM: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
pub const PUMP_AMM: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";
pub const PUMP_FEES: &str = "pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ";

// https://docs.marinade.finance/developers/contract-addresses
pub const MARINADE_LIQUID_STAKING: &str = "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD";
pub const MARINADE_SPL_GOVERNANCE: &str = "GovMaiHfpVPw8BAM1mbdzgmSZYDw2tdP32J2fapoQoYs";
pub const MARINADE_TOKADAPT: &str = "tokdh9ZbWPxkFzqsKqeAwLDk6J6a8NBZtQanVuuENxa";
pub const MARINADE_ESCROW_RELOCKER: &str = "tovt1VkTE2T4caWoeFP6a2xSFoew5mNpd7FWidyyMuk";
pub const MARINADE_VALIDATOR_GAUGES: &str = "va12L6Z9fa5aGJ7gxtJuQZ928nySAk5UetjcGPve3Nu";
pub const MARINADE_LIQUIDITY_GAUGES: &str = "LigadctxNRkZied3WuhX525vUhDkuhXNK5DyeijeDnh";
pub const MARINADE_REFERRAL: &str = "MR2LqxoSbw831bNy68utpu5n4YqBH3AzDmddkgk9LQv";
pub const MARINADE_DIRECTED_STAKE: &str = "dstK1PDHNoKN9MdmftRzsEbXP5T1FTBiQBm1Ee3meVd";
pub const MARINADE_VOTER_STAKE_REGISTRY: &str = "VoteMBhDCqGLRgYpp9o7DGyq81KNmwjXQRAHStjtJsS";
pub const MARINADE_NATIVE_STAKING_PROXY: &str = "mnspJQyF1KdDEs5c6YJPocYdY1esBgVQFufM2dY9oDk";

pub fn is_swap(program_id: &Pubkey) -> Option<String> {
  let program_str = program_id.to_string();

  match program_str.as_str() {
    RAYDIUM_CLMM | RAYDIUM_LEGACY_AMM | RAYDIUM_CPMM => Some("Raydium".to_string()),
    _ => None,
  }
}

pub fn is_dex_program(program_id: &Pubkey) -> Option<String> {
  let program_str = program_id.to_string();

  match program_str.as_str() {
    // Raydium
    RAYDIUM_CPMM | RAYDIUM_LEGACY_AMM | RAYDIUM_CLMM | RAYDIUM_STABLE | RAYDIUM_ROUTING => {
      Some("Raydium".to_string())
    }

    // Orca
    ORCA_WHIRLPOOL => Some("Orca".to_string()),

    // Jupiter
    JUPITER_SWAP
    | JUPITER_PERPETUALS
    | JUPITER_REFERRAL
    | JUPITER_DOVES
    | JUPITER_LEND_EARN
    | JUPITER_LEND_BORROW
    | JUPITER_LEND_EARN_REWARDS
    | JUPITER_LEND_LIQUIDITY
    | JUPITER_LEND_BORROW_ORACLE
    | JUPITER_LIMIT_ORDER_V2
    | JUPITER_DCA
    | JUPITER_LOCK
    | JUPITER_GOVERNANCE
    | JUPITER_VOTER => Some("Jupiter".to_string()),

    // Drift
    DRIFT_V2 => Some("Drift".to_string()),

    // Kamino
    KAMINO_LEND | KAMINO_LIQUIDITY | KAMINO_VAULTS => Some("Kamino".to_string()),

    // Meteora
    METEORA_DLMM
    | METEORA_DAMM_V2
    | METEORA_DBC
    | METEORA_PRESALE_VAULT
    | METEORA_ALPHA_VAULT
    | METEORA_DYNAMIC_FEE_SHARING
    | METEORA_ZAP
    | METEORA_DAMM_V1
    | METEORA_DYNAMIC_VAULT
    | METEORA_STAKE2EARN
    | METEORA_FARM
    | METEORA_MERCURIAL_STABLE_SWAP => Some("Meteora".to_string()),

    // Pump.fun
    PUMP_PROGRAM | PUMP_AMM | PUMP_FEES => Some("Pump.fun".to_string()),

    // Marinade
    MARINADE_LIQUID_STAKING
    | MARINADE_SPL_GOVERNANCE
    | MARINADE_TOKADAPT
    | MARINADE_ESCROW_RELOCKER
    | MARINADE_VALIDATOR_GAUGES
    | MARINADE_LIQUIDITY_GAUGES
    | MARINADE_REFERRAL
    | MARINADE_DIRECTED_STAKE
    | MARINADE_VOTER_STAKE_REGISTRY
    | MARINADE_NATIVE_STAKING_PROXY => Some("Marinade".to_string()),

    _ => None,
  }
}
