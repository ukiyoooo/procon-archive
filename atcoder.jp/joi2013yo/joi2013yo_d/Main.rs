#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use itertools_num::ItertoolsNum;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::iter;
#[allow(unused_imports)]
use superslice::*;

fn run() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let d: usize = sc.read();
    let n: usize = sc.read();
    let t = sc.read_vec::<i64>(d);
    let abc = sc.read_matrix::<i64>(n, 3);

    let mut dp = vec![vec![0; n]; d + 1];
    for i in 1..d {
        for j in 0..n {
            if t[i] < abc[j][0] || t[i] > abc[j][1] {
                continue;
            }
            for k in 0..n {
                if t[i - 1] < abc[k][0] || t[i - 1] > abc[k][1] {
                    continue;
                }
                dp[i + 1][j] = cmp::max(dp[i + 1][j], dp[i][k] + (abc[j][2] - abc[k][2]).abs());
            }
        }
    }
    let ans = dp[d].iter().max().unwrap();
    println!("{}", ans);
}

fn main() {
    std::thread::Builder::new()
        .name("run".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: std::ops::Deref<Target = str>>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn read_vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn read_pairs<T: std::str::FromStr>(&mut self, n: usize) -> Vec<(T, T)> {
        (0..n).map(|_| (self.read(), self.read())).collect()
    }
    pub fn read_pairs_1_indexed(&mut self, n: usize) -> Vec<(usize, usize)> {
        (0..n)
            .map(|_| (self.read::<usize>() - 1, self.read::<usize>() - 1))
            .collect()
    }
    pub fn read_chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
    pub fn read_char_grid(&mut self, n: usize) -> Vec<Vec<char>> {
        (0..n).map(|_| self.read_chars()).collect()
    }
    pub fn read_matrix<T: std::str::FromStr>(&mut self, n: usize, m: usize) -> Vec<Vec<T>> {
        (0..n)
            .map(|_| (0..m).map(|_| self.read()).collect())
            .collect()
    }
}