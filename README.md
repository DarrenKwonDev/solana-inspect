# solana-inspect

```
🏗️ in progress
```

cli for inspect solana blockchain    

## tools 
- [x] rpc_check   
- [x] dex_count   
- [x] idl_parse
  - print instruction's name and descriminator in anchor convention
  - ⚠️ Caution! Some projects (e.g., Raydium AMM) were not built with the Anchor framework. In those cases, parsing the IDL is pointless—you should look at their source code instead.
- [ ] token_list
- [ ] swap_watch
  - swap pool을 소유한 곳만 대상 (raydium, meteora, orca, pump)
  - aggregator(jupyter, okx dex)와 aggregator를 대상으로 한 private pool(solfi) 등은 제외 

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
