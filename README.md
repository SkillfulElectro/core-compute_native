# CORE-COMPUTE_NATIVE
- this project is native binding generator for core-compute api

## GETTING STARTED
- because core-compute apis are Rust macro functions , and we are not able to use Rust macros in C directly this project aims to give you
flexible binding generator for its api
- first you have to set how many compute functions you will need in C/C++ side of your project by 
```shell
export NUM_CORE_COMPUTE_REQ={how many functions for api you need}
```
** if you dont set any by default the build process will consider one function you will need **

- now you have to set for all of those compute functions starting from 0 , how much CInfo var you going to give as parameter to 
```shell
export NUM_CORE_{index of the function}={number of CInfo paras you want to pass to func}
```
** if you dont set it by default it will set 2 for each of them **

- now you can build CORE-COMPUTE_NATIVE
```shell 
cargo build --release
```

** core_compute_native.h will be found at target dir **


## NOTE 
- this project is bundled with core-compute v0.5.0 for now
