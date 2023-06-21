use bs58::encode;

#[test]
fn test_base58() {
    let e = "1~2-3_4.5";
    let d = "dYFmosWnRmkQ";
    assert_eq!(d, encode(e.as_bytes()).into_string());
}
