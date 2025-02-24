/* tslint:disable */
/* eslint-disable */
/**
 *
 * * Verify Ed25519 signatures one by one using strict verification.
 * *
 * * This function is slower but does strict verification.
 * 
 */
export function verify_ed25519(data: Uint8Array): Uint8Array;
/**
 *
 * * Verify Ed25519 signatures using build-in batch verification.
 * *
 * * This function is faster but does not do strict verification.
 * * See https://crates.io/crates/ed25519-dalek#batch-verification for more information.
 * 
 */
export function verify_ed25519_batch(data: Uint8Array): boolean;
