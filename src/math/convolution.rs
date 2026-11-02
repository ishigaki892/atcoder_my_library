use crate::prelude::*;
use ac_library::modint::ModInt998244353;

/// NTT (数論変換, mod = 998244353 決め打ち)
/// 畳み込みが必要な場合は ac_library::convolution::convolution / convolution_i64 を
/// 使うのが基本（内部でNTTが使われている）。以下は自前実装の参考用。
/// let c = ntt_convolution(&a, &b); // a, b は係数列 (下位桁が先頭)
pub fn ntt_convolution(a: &[i64], b: &[i64]) -> Vec<i64> {
    type Mint = ModInt998244353;
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut fa: Vec<Mint> = a.iter().map(|&x| Mint::new(x)).collect();
    let mut fb: Vec<Mint> = b.iter().map(|&x| Mint::new(x)).collect();

    let n = fa.len() + fb.len() - 1;
    let mut sz = 1usize;
    while sz < n {
        sz *= 2;
    }
    fa.resize(sz, Mint::new(0));
    fb.resize(sz, Mint::new(0));

    ntt(&mut fa, false);
    ntt(&mut fb, false);
    for i in 0..sz {
        fa[i] *= fb[i];
    }
    ntt(&mut fa, true); // 逆変換の 1/sz 倍は ntt 内で済んでいる

    fa.iter().take(n).map(|&x| x.val() as i64).collect()
}

pub fn ntt(f: &mut Vec<ModInt998244353>, invert: bool) {
    type Mint = ModInt998244353;
    let n = f.len();
    if n == 1 {
        return;
    }
    // ビット反転
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            f.swap(i, j);
        }
    }

    let g = Mint::new(3); // 998244353 の原始根
    let mut len = 2usize;
    while len <= n {
        let mut w = g.pow((MOD - 1) / len as u64);
        if invert {
            w = w.inv();
        }
        let mut i = 0usize;
        while i < n {
            let mut wn = Mint::new(1);
            for k in 0..len / 2 {
                let u = f[i + k];
                let v = f[i + k + len / 2] * wn;
                f[i + k] = u + v;
                f[i + k + len / 2] = u - v;
                wn *= w;
            }
            i += len;
        }
        len <<= 1;
    }

    if invert {
        let inv_n = Mint::new(n as i64).inv();
        for x in f.iter_mut() {
            *x *= inv_n;
        }
    }
}
