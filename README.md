# Typeberry Native

This repository is dedicated to compiling Rust libraries to WASM to be used in the typeberry JAM client implementation.

## Sub-packages

### 1. Ed25519-wasm

**Purpose:** Used for cryptographic signature verification and signing, exposed to WASM.

### 2. Reed-Solomon-wasm

**Purpose:** Provides Reed Solomon error-correcting capabilities to guarantee data integrity.

### 3. Bandersnatch

**Purpose:** Implements zero-knowledge VRF functionality using Bandersnatch elliptic curve.

## License

This project is licensed under MIT.
