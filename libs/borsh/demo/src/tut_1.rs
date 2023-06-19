//! Create a Tweet & then serialize/deserialize to save the data into DB.
//! Also, helpful to speed-up the process of transmitting data over the
//! network during downloading/uploading.

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug)]
struct Tweet {
    author: String,
    description: [u8; 64],
}

#[test]
fn test_struct() {
    // give a description & add zeros to fill the array
    let description_str = "The AI models are used for different purposes";
    let mut description_bytes = [0u8; 64]; // Initialize an array of zeros.
    description_bytes[..description_str.len()].copy_from_slice(description_str.as_bytes()); // Copy the bytes of the string into the array.

    // Create a new Tweet
    let t = Tweet {
        author: "alice".to_string(),
        description: description_bytes,
    };

    let encoded_t = t.try_to_vec().unwrap(); // encoded
    let decoded_t = Tweet::try_from_slice(&encoded_t).unwrap(); // decoded

    // Check that the decoded tweet is equal to the original one.
    assert_eq!(t, decoded_t);
}
