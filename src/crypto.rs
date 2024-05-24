use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

fn encrypt_aes_cbc(key: &[u8], data: &[u8]) -> Vec<u8> {
    let iv = &[0u8; 16];
    let res = Aes128CbcEnc::new(key.into(), iv.into()).encrypt_padded_vec_mut::<Pkcs7>(data);
    res.to_vec()
}

fn decrypt_aes_cbc(key: &[u8], data: &[u8]) -> Vec<u8> {
    let iv = &[0u8; 16];
    let res = Aes128CbcDec::new(key.into(), iv.into())
        .decrypt_padded_vec_mut::<Pkcs7>(data)
        .unwrap();
    res.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_aes_test() {
        let key = b"1234567890abcdef";
        let res = encrypt_aes_cbc(key, b"hello world");
        println!("aes encrypt is {:?}", res);
    }

    #[test]
    fn decrypt_aes_test() {
        let key = b"1234567890abcdef";
        let xxx = encrypt_aes_cbc(key, b"hello world");
        let res = decrypt_aes_cbc(key, xxx.as_slice());
        println!("aes decrypt is {:?}", String::from_utf8(res));
    }
}
