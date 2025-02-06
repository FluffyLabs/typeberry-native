/* tslint:disable */
/* eslint-disable */
/**
* @returns {boolean}
*/
export function verify_safrole(): boolean;
/**
* @param {Uint8Array} keys
* @returns {Uint8Array}
*/
export function ring_commitment(keys: Uint8Array): Uint8Array;
/**
* @param {Uint8Array} signatures
* @returns {Uint8Array}
*/
export function entropy_hash(signatures: Uint8Array): Uint8Array;
/**
* @param {Uint8Array} keys
* @param {Uint8Array} tickets_data
* @param {number} context_length
* @returns {Uint8Array}
*/
export function verify_ticket(keys: Uint8Array, tickets_data: Uint8Array, context_length: number): Uint8Array;
