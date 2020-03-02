#![no_main]
#![no_std]

use cortex_m_rt as rt;
use panic_semihosting as _;

#[rt::entry]
fn main() -> ! {

    use chacha20poly1305::ChaCha20Poly1305; // Or `XChaCha20Poly1305`
    use chacha20poly1305::aead;
    use aead::{Aead, NewAead};
    use aead::generic_array::{GenericArray, typenum::U128};
    use aead::heapless::Vec;

    let key = GenericArray::clone_from_slice(b"an example very very secret key.");
    let aead = ChaCha20Poly1305::new(key);

    let nonce = GenericArray::from_slice(b"unique nonce"); // 128-bits; unique per message

    let mut buffer: Vec<u8, U128> = Vec::new();
    buffer.extend_from_slice(b"plaintext message");

    // Encrypt `buffer` in-place, replacing the plaintext contents with ciphertext
    aead.encrypt_in_place(nonce, b"", &mut buffer).expect("encryption failure!");

    // `buffer` now contains the message ciphertext
    assert_ne!(&buffer, b"plaintext message");

    // Decrypt `buffer` in-place, replacing its ciphertext context with the original plaintext
    aead.decrypt_in_place(nonce, b"", &mut buffer).expect("decryption failure!");
    assert_eq!(&buffer, b"plaintext message");

    loop {
        continue;
    }
}
