// ------------------------------------------------------------
// ModInt 使用例（ac_library）
// ------------------------------------------------------------
// type Mint = ModInt998244353; // or ModInt1000000007
//
// let mut x = Mint::new(3);
// let y = Mint::new(4);
// let z = x + y;             // 通常の四則演算子がそのままMOD付きで使える
// let p = x.pow(10);         // 累乗
// let inv = x.inv();         // 逆元
// println!("{}", z);         // Displayで自動的に0..MOD範囲の値を表示

// ------------------------------------------------------------
// 数論系
// ------------------------------------------------------------

/// 最大公約数
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

/// 最小公倍数
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }

    (a / gcd(a, b)) * b
}

/// 組み合わせ（O(r)、1回きりのクエリ向け。大量にnCrを問われる問題ではComb構造体を使うこと）
pub fn combination(n: u64, r: u64, md: u64) -> u64 {
    if r > n {
        return 0;
    }
    let mut num = 1u64;
    let mut den = 1u64;
    for i in 0..r {
        num = num * ((n - i) % md) % md;
        den = den * ((i + 1) % md) % md;
    }
    num * power(den, md - 2, md) % md
}

/// 階乗・逆元階乗を前計算し、O(1)でnCr, nPrを求める構造体
/// 1問の中で何度もnCrを問われる数え上げ系問題向け
/// let comb = Comb::new(200000, MOD);
/// comb.ncr(n, r)
pub struct Comb {
    pub fact: Vec<u64>,
    pub inv_fact: Vec<u64>,
    pub modulo: u64,
}

impl Comb {
    pub fn new(max_n: usize, modulo: u64) -> Self {
        let mut fact = vec![1u64; max_n + 1];
        for i in 1..=max_n {
            fact[i] = fact[i - 1] * i as u64 % modulo;
        }
        let mut inv_fact = vec![1u64; max_n + 1];
        inv_fact[max_n] = power(fact[max_n], modulo - 2, modulo);
        for i in (0..max_n).rev() {
            inv_fact[i] = inv_fact[i + 1] * (i as u64 + 1) % modulo;
        }
        Self {
            fact,
            inv_fact,
            modulo,
        }
    }

    pub fn ncr(&self, n: usize, r: usize) -> u64 {
        if r > n {
            return 0;
        }
        self.fact[n] * self.inv_fact[r] % self.modulo * self.inv_fact[n - r] % self.modulo
    }

    pub fn npr(&self, n: usize, r: usize) -> u64 {
        if r > n {
            return 0;
        }
        self.fact[n] * self.inv_fact[n - r] % self.modulo
    }
}

/// 素因数分解
pub fn factorize(mut n: u64) -> std::collections::HashMap<u64, u64> {
    let mut map = std::collections::HashMap::new();
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            *map.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        map.insert(n, 1);
    }
    map
}

/// 10進法をn進法に変換
pub fn to_n_base(mut n: u64, base: u64) -> Vec<u64> {
    if n == 0 {
        return vec![0];
    }
    let mut res = Vec::new();
    while n > 0 {
        res.push(n % base);
        n /= base;
    }
    res.reverse();
    res
}

/// n進法を10新法に変換
pub fn from_n_base(digits: &[u64], base: u64) -> u64 {
    let mut res = 0;
    for &d in digits {
        res = res * base + d;
    }
    res
}

/// 桁数カウント（0 は 1 桁）
pub fn cnt_keta(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    let mut s = n;
    let mut ans = 0;
    while s > 0 {
        s /= 10;
        ans += 1;
    }
    ans
}

/// 桁和カウント
pub fn sum_keta(n: usize) -> usize {
    let mut s = n;
    let mut ans = 0;
    while s > 0 {
        ans += s % 10;
        s /= 10;
    }
    ans
}

/// 整数平方根 floor(sqrt(n))（usize全域で正しい）
pub fn isqrt(n: usize) -> usize {
    // f64 で概算してから u128 で誤差を補正する
    let mut x = (n as f64).sqrt() as u128;
    let n = n as u128;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x as usize
}

/// 整数三乗根 floor(cbrt(n))（usize全域で正しい）
pub fn cbrt(n: usize) -> usize {
    let mut x = (n as f64).cbrt() as u128;
    let n = n as u128;
    while x * x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x as usize
}

/// 整数の床除算（a / b の床）
/// - 任意の符号の a, b に対して floor を返す（b != 0 を仮定）
pub fn div_floor(a: i64, b: i64) -> i64 {
    let q = a / b;
    let r = a % b;
    if r != 0 && ((r > 0) != (b > 0)) {
        q - 1
    } else {
        q
    }
}

/// 整数の天井除算（a / b の天井）
/// - 任意の符号の a, b に対して ceil を返す（b != 0 を仮定）
pub fn div_ceil(a: i64, b: i64) -> i64 {
    let q = a / b;
    let r = a % b;
    if r != 0 && ((r > 0) == (b > 0)) {
        q + 1
    } else {
        q
    }
}

/// usize 用の安全な天井除算（b > 0 を仮定）
pub fn ceil_div_usize(a: usize, b: usize) -> usize {
    assert!(b > 0);
    if a == 0 {
        0
    } else {
        (a - 1) / b + 1
    }
}

/// 約数の個数をカウント
pub fn count_divisors(mut n: u64) -> u64 {
    let mut res = 1;
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            let mut count = 0;
            while n % i == 0 {
                count += 1;
                n /= i;
            }
            res *= count + 1;
        }
        i += 1;
    }
    if n > 1 {
        res *= 2;
    }
    res
}

/// エラトステネスの篩
pub fn eratosthenes(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }
    for i in 2..=n {
        if is_prime[i] {
            let mut j = i * 2;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    is_prime
}

/// ユークリッドの互除法
pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x1, y1) = ext_gcd(b, a % b);
        let x = y1;
        let y = x1 - (a / b) * y1;
        (g, x, y)
    }
}

/// 逆元
pub fn mod_inv(a: u64, m: u64) -> u64 {
    let (g, x, _) = ext_gcd(a as i64, m as i64);
    if g != 1 {
        0
    } else {
        ((x % m as i64 + m as i64) % m as i64) as u64
    }
}

/// N!(Nの階乗)
/// usizeのまま計算するのでn>=21程度でオーバーフローする。MOD付きが必要ならComb構造体を使うこと
pub fn factorial(n: usize) -> usize {
    (1..=n).product()
}

/// 約数を全列挙
pub fn divisors(n: usize) -> Vec<usize> {
    let mut divs = Vec::new();
    let mut i = 1usize;
    while i * i <= n {
        if n % i == 0 {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
            }
        }
        i += 1;
    }
    divs.sort();
    divs
}

/// Miller-Rabin素数判定 (u64全域で決定的に正しい)
pub fn miller_rabin(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return false;
        }
    }
    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    let mul_mod = |a: u64, b: u64, m: u64| -> u64 { ((a as u128 * b as u128) % m as u128) as u64 };
    let pow_mod = |mut a: u64, mut e: u64, m: u64| -> u64 {
        let mut res = 1u64;
        a %= m;
        while e > 0 {
            if e & 1 == 1 {
                res = mul_mod(res, a, m);
            }
            a = mul_mod(a, a, m);
            e >>= 1;
        }
        res
    };

    for &a in &[2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if a >= n {
            continue;
        }
        let mut x = pow_mod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut ok = false;
        for _ in 0..r - 1 {
            x = mul_mod(x, x, n);
            if x == n - 1 {
                ok = true;
                break;
            }
        }
        if !ok {
            return false;
        }
    }
    true
}

/// Pollard's Rho による素因数分解 O(n^{1/4} polylog)
pub fn pollard_rho(n: u64) -> Vec<u64> {
    if n == 1 {
        return vec![];
    }
    if miller_rabin(n) {
        return vec![n];
    }
    let mul_mod = |a: u64, b: u64, m: u64| -> u64 { ((a as u128 * b as u128) % m as u128) as u64 };

    let mut c = 1u64;
    loop {
        let f = |x: u64| (mul_mod(x, x, n) + c) % n;
        let (mut x, mut y, mut d) = (2u64, 2u64, 1u64);
        while d == 1 {
            x = f(x);
            y = f(f(y));
            d = gcd(if x > y { x - y } else { y - x }, n);
        }
        if d != n {
            let mut res = pollard_rho(d);
            res.extend(pollard_rho(n / d));
            return res;
        }
        c += 1;
    }
}

/// floor_sum: sum_{i=0}^{n-1} floor((a*i+b)/m) をO(log(min(a,m)))で求める
/// AtCoder Library の floor_sum と同一のアルゴリズム
/// 制約: n >= 0, m >= 1 (a, b は負でもよい)
pub fn floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64 {
    let mut ans: i64 = 0;
    let (mut n, mut m, mut a, mut b) = (n, m, a, b);
    if a < 0 {
        let a2 = a.rem_euclid(m);
        ans -= n * (n - 1) / 2 * ((a2 - a) / m);
        a = a2;
    }
    if b < 0 {
        let b2 = b.rem_euclid(m);
        ans -= n * ((b2 - b) / m);
        b = b2;
    }
    loop {
        if a >= m {
            ans += n * (n - 1) / 2 * (a / m);
            a %= m;
        }
        if b >= m {
            ans += n * (b / m);
            b %= m;
        }
        let y_max = a * n + b;
        if y_max < m {
            break;
        }
        n = y_max / m;
        b = y_max % m;
        std::mem::swap(&mut m, &mut a);
    }
    ans
}

/// 中国剰余定理 (CRT)
/// x ≡ r1 (mod m1), x ≡ r2 (mod m2) を満たす x を法 lcm(m1, m2) の範囲で求める
/// ## 出力
/// `Some((x, lcm))` - 解が存在する場合。x は 0 <= x < lcm を満たす
/// `None` - 矛盾していて解が存在しない場合
pub fn crt(r1: i64, m1: i64, r2: i64, m2: i64) -> Option<(i64, i64)> {
    let (g, p, _q) = ext_gcd(m1, m2);
    if (r2 - r1) % g != 0 {
        return None;
    }
    let lcm = m1 / g * m2;
    // x = r1 + m1 * ((r2 - r1) / g * p mod (m2 / g))
    let tmp = (r2 - r1) / g % (m2 / g) * p % (m2 / g);
    let mut x = r1 + m1 * tmp;
    x = x.rem_euclid(lcm);
    Some((x, lcm))
}

/// 複数の合同式 x ≡ rs[i] (mod ms[i]) をまとめて解く CRT
/// ## 出力
/// `Some((x, lcm))` - 0 <= x < lcm を満たす解
/// `None` - 矛盾がある場合
pub fn crt_multi(rs: &[i64], ms: &[i64]) -> Option<(i64, i64)> {
    let mut r = 0i64;
    let mut m = 1i64;
    for (&ri, &mi) in rs.iter().zip(ms.iter()) {
        match crt(r, m, ri, mi) {
            Some((nr, nm)) => {
                r = nr;
                m = nm;
            }
            None => return None,
        }
    }
    Some((r, m))
}

// ------------------------------------------------------------
// MOD演算用
// ------------------------------------------------------------

pub fn power(mut base: u64, mut exp: u64, modulo: u64) -> u64 {
    let mut result = 1u64;
    base %= modulo;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modulo;
        }
        base = base * base % modulo;
        exp >>= 1;
    }
    result
}

/// n の下から m 桁目（0-indexed）の数字を取り出す
/// 例: get_nth(12345, 0) = 5, get_nth(12345, 2) = 3
pub fn get_nth(n: usize, m: usize) -> usize {
    let base = 10_usize.pow(m as u32);
    (n / base) % 10
}

/// max_digits 桁以下の回文数（0 を含む）を昇順に全列挙する
pub fn generate_palindromes(max_digits: u32) -> Vec<u64> {
    let mut res: Vec<u64> = (0..10).collect();

    for length in 2..=max_digits {
        let is_odd = length % 2 != 0;
        let half_len = (length + 1) / 2;

        let start = 10_u64.pow(half_len - 1);
        let end = 10_u64.pow(half_len);

        for i in start..end {
            let s = i.to_string();
            let rev_s: String = s.chars().rev().collect();

            let full_s = if is_odd {
                format!("{}{}", s, &rev_s[1..])
            } else {
                format!("{}{}", s, rev_s)
            };

            if let Ok(num) = full_s.parse::<u64>() {
                res.push(num);
            }
        }
    }

    res.sort();
    res
}
