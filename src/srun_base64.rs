const CHAR_SET_DEFAULT: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const CHAR_SET_ALT: &[u8] = b"LVoJPiCN2R8G90yg+hmFHuacZ1OWMnrsSTXkYpUq/3dlbfKwv6xztjI7DeBE45QA";

fn transform(data: &Vec<u8>, char_set_from: &[u8], char_set_to: &[u8]) -> Vec<u8> {
  data.iter().map(
    |&x| char_set_to[char_set_from.iter().position(|&r| r == x).unwrap() as usize]
  ).collect()
}

pub fn encode(data: &Vec<u8>) -> String {
  String::from_utf8(transform(&base64::encode(&data).into_bytes(), CHAR_SET_DEFAULT, CHAR_SET_ALT)).unwrap()
}