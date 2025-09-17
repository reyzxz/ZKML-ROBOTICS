#!/bin/bash

BUILD_DIR="../build"
CIRCUIT_NAME=$1

if [ -z "$CIRCUIT_NAME" ]; then
    echo "Usage: ./generate_proof.sh <circuit_name>"
    exit 1
fi

R1CS="$BUILD_DIR/$CIRCUIT_NAME.r1cs"
WASM="$BUILD_DIR/$CIRCUIT_NAME_js/$CIRCUIT_NAME.wasm"
ZKEY="$BUILD_DIR/$CIRCUIT_NAME.zkey"
PROOF="$BUILD_DIR/$CIRCUIT_NAME-proof.json"
PUBLIC="$BUILD_DIR/$CIRCUIT_NAME-public.json"

mkdir -p $BUILD_DIR

echo "Generating witness..."
node $BUILD_DIR/${CIRCUIT_NAME}_js/generate_witness.js $WASM input.json witness.wtns

echo "Creating zkey..."
snarkjs groth16 setup $R1CS pot12_final.ptau $ZKEY

echo "Generating proof..."
snarkjs groth16 prove $ZKEY witness.wtns $PROOF $PUBLIC

echo "Proof generated at: $PROOF"
echo "Public signals at: $PUBLIC"
