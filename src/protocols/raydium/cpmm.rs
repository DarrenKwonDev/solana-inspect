use solana_transaction_status::UiPartiallyDecodedInstruction;

pub fn handle_raydium_cpmm_instr(instr: &UiPartiallyDecodedInstruction) {
  println!("cpmm {}", instr.program_id);
}
