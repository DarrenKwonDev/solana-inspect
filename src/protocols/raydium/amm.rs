use solana_transaction_status::UiPartiallyDecodedInstruction;

pub fn handle_raydium_amm_instr(instr: &UiPartiallyDecodedInstruction) {
  println!("amm {}", instr.program_id);
}
