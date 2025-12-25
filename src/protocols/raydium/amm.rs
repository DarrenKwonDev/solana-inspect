use anyhow::{Result, anyhow};
use solana_transaction_status::{
  EncodedTransactionWithStatusMeta, UiPartiallyDecodedInstruction,
  option_serializer::OptionSerializer,
};

use crate::{
  MAGENTA, RESET,
  fetcher::{TOKEN_ALL_KEY, cache::TokenCacheType},
};

/*
  amm protocol codes : https://github.com/raydium-io/raydium-amm
  instruction : https://github.com/raydium-io/raydium-amm/blob/master/program/src/instruction.rs


  신경 써야 할 건 pool 공식 (xy = k) 에 영향을 미치는 것과 직접적인 swap 만
   * MonitorStep (tag 2): AMM이 OpenBook DEX에 제출한 오더들을 모니터링하고 처리하는 단계입니다. (정기적인 크랭크(crank) 작업의 일부)
   * Deposit (tag 3): AMM 풀에 유동성(토큰)을 예치합니다.
   * Withdraw (tag 4): AMM 풀에서 유동성(토큰)을 인출합니다.
   * WithdrawPnl (tag 7): AMM 풀에서 발생한 손익(PnL)을 인출합니다.
   * SwapBaseIn (tag 9): 투입할 토큰 수량을 기준으로 스왑을 실행합니다.
   * SwapBaseOut (tag 11): 받을 토큰 수량을 기준으로 스왑을 실행합니다.
   * SwapBaseInV2 (tag 16): 오더북을 사용하지 않는(off-chain) 투입 토큰 기준 스왑입니다.
   * SwapBaseOutV2 (tag 17): 오더북을 사용하지 않는(off-chain) 받을 토큰 기준 스왑입니다.

   하지만 직접적인 swap이 1차적으로 중요
*/

pub fn handle_raydium_amm_instr(
  instr: &UiPartiallyDecodedInstruction,
  tx_meta: &EncodedTransactionWithStatusMeta,
  token_cache: TokenCacheType,
) -> Result<()> {
  let decoded = bs58::decode(&instr.data).into_vec()?;
  if decoded.is_empty() {
    return Ok(());
  }

  if !decoded.is_empty() {
    let (&tag, rest) = decoded.split_first().ok_or(anyhow!("split_first failed"))?;

    match tag {
      2 => {
        // println!("MonitorStep")
      }
      3 => {
        // println!("Deposit")
      }
      4 => {
        // println!("Withdraw")
      }
      7 => {
        // println!("WithdrawPnl")
      }
      9 => {
        let (amount_in, _rest) = unpack_u64(rest).unwrap();

        let amm_account = instr.accounts.get(1).map(|s| s.as_str());

        println!(
          "{MAGENTA}[Raydium] AMM | SwapBaseIn | {} | pool {}{RESET}",
          amount_in,
          amm_account.unwrap_or("?")
        );
      }
      11 => {
        /*
        pub struct SwapInstructionBaseOut {
            // SOURCE amount to transfer, output to DESTINATION is based on the exchange rate
            pub max_amount_in: u64,
            /// Minimum amount of DESTINATION token to output, prevents excessive slippage
            pub amount_out: u64,
        }

        */
        println!("SwapBaseOut")
      }
      16 => println!("SwapBaseInV2"),
      17 => println!("SwapBaseOutV2"),
      _ => {}
    }
  }

  Ok(())
}

#[allow(dead_code)]
fn unpack_u8(input: &[u8]) -> Result<(u8, &[u8])> {
  if input.is_empty() {
    return Err(anyhow!("insufficient data for u8"));
  }
  Ok((input[0], &input[1..]))
}

#[allow(dead_code)]
fn unpack_u16(input: &[u8]) -> Result<(u16, &[u8])> {
  if input.len() < 2 {
    return Err(anyhow!("insufficient data for u16"));
  }
  let amount = u16::from_le_bytes([input[0], input[1]]);
  Ok((amount, &input[2..]))
}

#[allow(dead_code)]
fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8])> {
  if input.len() < 8 {
    return Err(anyhow!("insufficient data for u64"));
  }
  let amount = u64::from_le_bytes(input[..8].try_into()?);
  Ok((amount, &input[8..]))
}
