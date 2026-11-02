// ------------------------------------------------------------
// 便利マクロ
// ------------------------------------------------------------

/// chmin!(a, b) : a = min(a, b) を行い、更新されたら true を返す
#[macro_export]
macro_rules! chmin {
    ($a:expr, $b:expr) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

/// chmax!(a, b) : a = max(a, b) を行い、更新されたら true を返す
#[macro_export]
macro_rules! chmax {
    ($a:expr, $b:expr) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

/// デバッグ出力
#[macro_export]
macro_rules! debug {
    ($($a:expr),*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    };
}

/// Yes or No
#[macro_export]
macro_rules! yn {
    ($cond:expr) => {
        println!("{}", if $cond { "Yes" } else { "No" });
    };
}

/// YES or NO
#[macro_export]
macro_rules! YN {
    ($cond:expr) => {
        println!("{}", if $cond { "YES" } else { "NO" });
    };
}
