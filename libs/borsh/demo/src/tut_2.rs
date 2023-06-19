//! Create a block & then serialize/deserialize for
//! adding to chain
//! Also, helpful to speed-up the transmitting data over the
//! network during downloading/uploading.
//!
//! Here, additionally we add `init` method to the struct
//! so that after deserialization, `init` method is executed.
//!
//! Normally, `init` method is used for validating the signature when received.
//!
//! For instance, the transaction (token transfer from A to B) is sent to the block author in serialized format.
//! And then the block author deserializes the transaction & validates the signature.
//!
//! So, we just need to add the `init` method to the struct & then automatically, after deserialization it validates the signature.
//! And then the block author can accept/reject the transaction based on the verification result.

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
#[borsh_init(validate)]
struct Block {
    timestamp: u32,
    block_number: u64,
    block_data: [u8; 32], // 32 bytes i.e. 32 chars
}

#[allow(unused)]
impl Block {
    fn validate(&mut self) {
        // NOTE: Ideally it is used as validation step for the deserialized data like signature verification
        // It is run right after the deserialization step.
        // Alice --send-txn-in-serialized-format--> Author node --deserialize--> original data (with `validate` method run automatically).
        // if any error, then during deserialization process itself, panics.

        assert!(self.block_number > 0);
    }
}

#[test]
fn test_add_block() {
    // give a description & add zeros to fill the array
    let block_data_str = "set_value(u8)";
    let mut block_data_bytes = [0u8; 32]; // Initialize an array of zeros.
    block_data_bytes[..block_data_str.len()].copy_from_slice(block_data_str.as_bytes()); // Copy the bytes of the string into the array.

    // Create a new Block with all fields except `total_msg`
    // NOTE: auto-computed in `init` after deserilization
    let b = Block {
        timestamp: 1630480000,
        block_number: 1,
        block_data: block_data_bytes,
    };
    println!("{:?}", b);

    let encoded_b = b.try_to_vec().unwrap(); // encoded
    println!("encoded_b: {:?}", encoded_b);
    let decoded_b = Block::try_from_slice(&encoded_b).unwrap(); // decoded
    println!("decoded_b: {:?}", decoded_b);

    // Check that the decoded block timestamp is equal to the original one's timestamp.
    // The entire block info won't match because there has been additional
    // `init` step not done during serialization.
    assert_eq!(b.timestamp, decoded_b.timestamp);
}

#[test]
#[should_panic]
fn test_panic() {
    // give a description & add zeros to fill the array
    let block_data_str = "set_value(u8)";
    let mut block_data_bytes = [0u8; 32]; // Initialize an array of zeros.
    block_data_bytes[..block_data_str.len()].copy_from_slice(block_data_str.as_bytes()); // Copy the bytes of the string into the array.

    // Create a new Block with all fields except `total_msg`
    // NOTE: auto-computed in `init` after deserilization
    let b = Block {
        timestamp: 1630480000,
        block_number: 0,
        block_data: block_data_bytes,
    };
    // println!("{:?}", b);

    let encoded_b = b.try_to_vec().unwrap(); // encoded
                                             // println!("encoded_b: {:?}", encoded_b);
    let decoded_b = Block::try_from_slice(&encoded_b)
        .unwrap_or_else(|e| panic!("Failed to decode block: {}", e)); // decoded
}
