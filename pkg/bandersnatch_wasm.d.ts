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
* Seal verification as defined in:
* https://graypaper.fluffylabs.dev/#/68eaa1f/0eff000eff00?v=0.6.4
* or
* https://graypaper.fluffylabs.dev/#/68eaa1f/0e54010e5401?v=0.6.4
* @param {Uint8Array} keys
* @param {number} signer_key_index
* @param {Uint8Array} signer_pub_key
* @param {Uint8Array} seal_data
* @param {Uint8Array} aux_data
* @returns {Uint8Array}
*/
export function verify_seal(keys: Uint8Array, signer_key_index: number, signer_pub_key: Uint8Array, seal_data: Uint8Array, aux_data: Uint8Array): Uint8Array;
/**
* Verify multiple tickets at once as defined in:
* https://graypaper.fluffylabs.dev/#/68eaa1f/0f3e000f3e00?v=0.6.4
*
* NOTE: the aux_data of VRF function is empty!
* @param {Uint8Array} keys
* @param {Uint8Array} tickets_data
* @param {number} vrf_input_data_len
* @returns {Uint8Array}
*/
export function batch_verify_tickets(keys: Uint8Array, tickets_data: Uint8Array, vrf_input_data_len: number): Uint8Array;
