# ZKML-Robotics Architecture

## Overview
ZKML-Robotics menggabungkan Zero-Knowledge Proof (zk-SNARK / zk-STARK) dengan kontrol robot AI.  
Robot dapat mengeksekusi perintah dan memverifikasi aksi tanpa membuka model atau data sensor.

## Components
- **Rust API**: load model, feed command, run inference, generate & verify proof
- **Circuits**: zk-SNARK / zk-STARK untuk command dan navigation
- **Examples**: demo robot movement & path planning
- **Tests**: unit & integration test
- **Benchmarks**: ukur runtime & ukuran proof

## Data Flow
1. Model di-load lewat Rust API
2. Robot menerima `RobotCommand`
3. `execute_command()` generate zk proof
4. Proof diverifikasi lewat `verify_proof()`
