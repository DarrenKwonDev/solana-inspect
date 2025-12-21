use solana_transaction_status::UiPartiallyDecodedInstruction;

pub fn handle_raydium_clmm_instr(instr: &UiPartiallyDecodedInstruction) {
  println!("clmm {}", instr.program_id);
}
