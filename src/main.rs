use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use core::result::Result;

use oscore::edhoc::{
    error::{OwnError, OwnOrPeerError},
    PartyI, PartyR,
};

use x25519_dalek_ng::{PublicKey, StaticSecret};

use rand::rngs::OsRng;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Hello Wolrd");

    let a = vec![1, 2, 3, 4];
    let b = vec![5, 6, 7, 8];
    let c = xor(&a, &b);
    println!("{:?}", c);
    test();
}

pub fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    if a.len() != b.len() {
        panic!("Attempting to xor vec's of unequal length");
    }

    let c = a.iter().zip(b.iter()).map(|(&x1, &x2)| x1 ^ x2).collect();

    c
}

fn test() {
    //let mut vec = Vec::new();
    let a = [42;20000];
    //let mut number = 0;
    a.to_vec();
    /*loop{
        println!("{}", vec.len());
        vec.push(number);
        number += 1;
    }*/
}

const SUITE_I: isize = 3;
const METHOD_TYPE_I: isize = 0;

fn run() {
    let i_static_priv: StaticSecret = StaticSecret::new(OsRng);
    let i_static_pub = PublicKey::from(&i_static_priv);
    println!("first");
    let mut r: StdRng = StdRng::from_entropy();
    let i_priv = r.gen::<[u8; 32]>();

    // Choose a connection identifier
    let i_c_i = [0x1].to_vec();

    let i_kid = [0xA2].to_vec();
    let msg1_sender = PartyI::new(i_c_i, i_priv, i_static_priv, i_static_pub, i_kid);
    println!("second");
    let (msg1_bytes, msg2_receiver) = msg1_sender
        .generate_message_1(METHOD_TYPE_I, SUITE_I)
        .unwrap();
    println!("third");
    let r_static_priv: StaticSecret = StaticSecret::new(OsRng);
    let r_static_pub = PublicKey::from(&r_static_priv);

    let r_kid = [0xA3].to_vec();

    // create keying material

    let mut r2: StdRng = StdRng::from_entropy();
    let r_priv = r2.gen::<[u8; 32]>();

    let msg1_receiver = PartyR::new(r_priv, r_static_priv, r_static_pub, r_kid);
    println!("Fourth");
    let msg2_sender = match msg1_receiver.handle_message_1(msg1_bytes) {
        Err(OwnError(b)) => {
            panic!("{:?}", b)
        }
        Ok(val) => val,
    };
    println!("Fifth");
    // TODO: FIND FIX FOR THIS STACK OVERFLOW
    let (msg2_bytes, msg3_receiver) = match msg2_sender.generate_message_2() {
        Err(OwnOrPeerError::PeerError(s)) => {
            panic!("Received error msg: {}", s)
        }
        Err(OwnOrPeerError::OwnError(b)) => {
            panic!("Send these bytes: {}", hexstring(&b))
        }
        Ok(val) => val,
    };
    println!("six");
}

fn hexstring(slice: &[u8]) -> String {
    String::from("0x")
        + &slice
            .iter()
            .map(|n| format!("{:02X}", n))
            .collect::<Vec<String>>()
            .join(", 0x")
}
