# CORE-COMPUTE_NATIVE
- this project native binding generator to core-compute api

## GETTING STARTED
- because core-compute api are Rust macro function , and we are not able to use Rust macros in C directly this project aims to give you
flexible binding generator
- first you have to set how many compute functions you will need in C/C++ side of your project by 
```shell
export NUM_CORE_COMPUTE_REQ=how much you want
```
** if you dont set any by default the build process will consider one function you will need **

- now you have to set for all of those compute functions starting from 0 , how much CInfo var you going to give as parameter to 
```shell
export NUM_CORE_{index of the function}
```
** if you dont set it by default it will set 2 for each of them **

- now you can build CORE-COMPUTE_NATIVE
```shell 
cargo build --release
```

** core_compute_native.h will be found at target dir **


