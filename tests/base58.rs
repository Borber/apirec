use base58_monero::encode;

#[test]
fn test_base58() {
    let e = "1~2-3_4.5";
    let d = "9H9LDS3DZM71v";
    assert_eq!(d, encode(e.as_bytes()).unwrap());
}
