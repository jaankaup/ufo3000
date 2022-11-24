#!/bin/bash

RUST_BACKTRACE=1 cargo run --features gpu_debug --example $1 -- $2 
#RUST_BACKTRACE=1 cargo run --example $1  
