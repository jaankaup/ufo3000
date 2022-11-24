#!/bin/bash

scp ./target/wasm-examples/$1/* jaankaup@130.234.208.250:/home/jaankaup/public_html/wgpu_wasm/$1
