# solana-inspect

```
🏗️ in progress
```

cli for inspect solana blockchain    

## tools

### general

- [x] rpc_check   
- [x] dex_count   
- [x] idl_parse
  - print instruction's name and descriminator in anchor convention
  - ⚠️ Caution! Some projects (e.g., Raydium AMM) were not built with the Anchor framework. In those cases, parsing the IDL is pointless—you should look at their source code instead.
- [x] token_list  
- [ ] block_viwer  
  - block_viwer --block $num  
  - block_viwer --block $num --filter dex
  - block_viwer --block $num --json
  - block_viwer --block $num --json --filter dex
- [ ] signer_scan  
  - signer_scan --block $num
  
### pump

- [ ] pump_bonding_curve
  - pump_bonding_curve $CA
  - Create 된 토큰 감지 후 초기 3분 간의 buy, sell 이벤트마다 bonding curve에 의해 계산된 가격을 출력합니다 
- [ ] pump_sniper_watch
  - pump_sniper_watch $CA --count $num
  - 특정 CA 가 Create 된 이후 $num 명의 buyer들을 추적합니다 

### others
- [ ] swap_watch
  - swap pool을 소유한 곳만 대상 (raydium, meteora, orca, pump)
  - aggregator(jupiter, okx dex)와 aggregator를 대상으로 한 private pool(solfi) 등은 제외 

## can I handle solana data in single core? do napkin math. 

yes

- given numbers
  - slot per : 400ms
  - block size in peak time : 1MB
  - so, should handle `1MB/0.4sec` (throughput)  

- Using i7-14700K (assume 3GHz)
  - 3.0 × 10^9 cycles/sec * 0.4 sec = 1.2 * 10^9 cycles
  - so, 400ms in single core can handle 1.2 * 10^9 cycles
  - let's assume 1 instruction takes 4 cycle. then you can 300M instructions per slot  
  - 1MB data can't be set into cpu cache and reuse it pointless(data is flow!)
  - in napkin math, `Read 1 MB sequentially from memory takes ~250 μs` (source : [Numbers Every Programmer Should Know](https://gist.github.com/jboner/2841832))
    - data read from RAM and can take advantage HW prefetch, cache line streaming. so, 1MB/250μs is quite conservative assume.
  - RAM latency doesn't matter. it provide GB per seconds!
  - 250 μs × 3 GHz = 750,000 cycles.

- conclusion
  - despite a severely conservative assumption (IPC = 4, memory read 250 μs), can handle data!
  - but, in logics, deserialization -> signal process -> trigger action should below 400ms.
  - may signal processing takes more time, then you should copy and offload to queue or something.

## known issues

- Cache files can be polluted due to race conditions. Currently, this project assumes the CLI is executed one at a time.
- `Not provide jito bundle identification`, because even if it is a bundle, the transaction is represented individually
- `standard ws may lag 30~50 blocks` use enhanced websocket or shred takes a lot of money. 
