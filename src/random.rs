/// 実行ごとに変わる乱数を1つ返す（ハッシュの基数などに使う）
pub fn random_u64() -> u64 {
    use std::hash::{BuildHasher, Hasher};
    let mut h = std::collections::hash_map::RandomState::new().build_hasher();
    h.write_u64(0x9E37_79B9_7F4A_7C15);
    h.finish()
}
