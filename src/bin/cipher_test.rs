use blake2::{Blake2b512, Blake2s256};
use hex::ToHex;
use hex_literal::hex;
use sm3::{Digest, Sm3};

fn main() {
    // create a hasher object, to use it do not forget to import `Digest` trait
    let mut sm3_hasher = Sm3::new();

    // write input message
    sm3_hasher.update(b"hello world");

    // read hash digest and consume hasher
    let res = sm3_hasher.finalize();
    println!("{}", res[..].iter().encode_hex::<String>());
    assert_eq!(
        res[..],
        hex!("44f0061e69fa6fdfc290c494654a05dc0c053da7e5c52b84ef93a9d67d3fff88")[..]
    );

    // create a Blake2b512 object
    let mut blake2_hasher = Blake2b512::new();

    // write input message
    blake2_hasher.update(b"hello world");

    // read hash digest and consume hasher
    let res = blake2_hasher.finalize();
    println!("{}", res[..].iter().encode_hex::<String>());
    assert_eq!(
        res[..],
        hex!("021ced8799296ceca557832ab941a50b4a11f83478cf141f51f933f653ab9fbcc05a037cddbed06e309bf334942c4e58cdf1a46e237911ccd7fcf9787cbc7fd0")[..]
    );

    // same example for Blake2s256:
    let mut hasher = Blake2s256::new();
    hasher.update(b"hello world");
    let res = hasher.finalize();
    println!("{}", res[..].iter().encode_hex::<String>());
    assert_eq!(
        res[..],
        hex!("9aec6806794561107e594b1f6a8a6b0c92a0cba9acf5e5e93cca06f781813b0b")[..]
    );
}
