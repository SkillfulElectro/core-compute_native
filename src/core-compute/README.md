# core-compute : NATIVE EDITION
- fast , simple and cross-platform parallel computing library native edition
- this edition aims to provide more performance for C API

## what's different?
- in Rusty API , we just replace the pointer with pointer which data is written to but in C API our goal is to write back to CPU memory ; so by introducing multithreading in CPU side we speed up writing data back to the CPU
- so using this API for Rusty way is not feasible because you will lose ownership of your data unless you get the address of it back and retrieve the ownership


for Rust check out : 
https://docs.rs/core-compute/latest/core_compute/

for native bindings :
https://github.com/SkillfulElectro/core-compute_native.git
