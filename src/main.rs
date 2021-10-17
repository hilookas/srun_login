mod srun_xxtea;
mod srun_base64;

use md5::Md5;
use hmac::{Hmac, Mac, NewMac};
type HmacMd5 = Hmac<Md5>;

use sha1::{Sha1, Digest};

use regex::Regex;

use std::{thread, time::Duration};

fn login(endpoint: &str, ac_id: &str, ip: &str, username: &str, password: &str) {
    let n = "200";
    let type_ = "1";
    let enc_ver = "srun_bx1";

    let body = reqwest::blocking::get(format!(
        "{}/cgi-bin/get_challenge?callback=jQuery1124007958736547802503_1111111111111&username={}&ip={}&_=1111111111111",
        endpoint,
        username,
        ip
    )).unwrap().text().unwrap();
    // println!("{:?}", body);

    let token = &Regex::new(r#""challenge":"([A-Za-z0-9]*?)""#).unwrap().captures_iter(&body).next().unwrap()[1];
    // println!("{}", token);

    let mut hmac = HmacMd5::new_from_slice(token.as_bytes()).unwrap();
    hmac.update(password.as_bytes());
    let hmac = hex::encode(&hmac.finalize().into_bytes());
    // println!("{}", hmac);

    let info = String::new() + "{\"username\":\"" + username + "\",\"password\":\"" + password + "\",\"ip\":\"" + ip + "\",\"acid\":\"" + ac_id + "\",\"enc_ver\":\"" + enc_ver + "\"}";
    let info = srun_base64::encode(&srun_xxtea::encrypt(&info, &token));
    // println!("{}", info);
    // println!("{}", std::str::from_utf8(&srun_xxtea::decrypt(&srun_xxtea::encrypt(&info, &token), &token)).unwrap())
    
    let chksum = String::new() + token + username + token + &hmac + token + ac_id + token + ip + token + n + token + type_ + token + "{SRBX1}" + &info;
    // println!("{}", chksum);
    let mut hasher = Sha1::new();
    hasher.update(chksum.as_bytes());
    let chksum = hex::encode(&hasher.finalize());
    // println!("{}", chksum);

    let body = reqwest::blocking::get(format!(
        "{}/cgi-bin/srun_portal?callback=jQuery1124007958736547802503_1111111111111&action=login&username={}&password=%7BMD5%7D{}&os=Mac+OS&name=Macintosh&double_stack=1&chksum={}&info=%7BSRBX1%7D{}&ac_id={}&ip={}&n={}&type={}&_=1111111111111",
        endpoint,
        username,
        hmac,
        chksum,
        urlencoding::encode(&info),
        ac_id,
        ip,
        n,
        type_
    )).unwrap().text().unwrap();
    println!("{:?}", body);

    let result = &Regex::new(r#""error":"(ok)""#).unwrap().captures_iter(&body).next().unwrap()[1];
}

fn main() {
    println!("srun login");

    let endpoint = std::env::args().nth(1).unwrap();
    let ac_id = std::env::args().nth(2).unwrap();
    let ip = std::env::args().nth(3).unwrap();
    let username = std::env::args().nth(4).unwrap();
    let password = std::env::args().nth(5).unwrap();

    login(&endpoint, &ac_id, &ip, &username, &password);
}