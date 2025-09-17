#!/bin/bash

mkdir -p build

for circuit in ../circuits/*.circom; do
    filename=$(basename -- "$circuit")
    name="${filename%.*}"
    circom "$circuit" --r1cs --wasm --sym -o build/
    echo "Compiled $name"
done
