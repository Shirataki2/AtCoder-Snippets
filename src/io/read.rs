#![allow(dead_code)]
use cargo_snippet::snippet;


#[snippet("io")]
/// ## Example
/// ```ignore
/// let mut io = kyoproio::IO::new();
/// let (x, y): (i64, i64) = io.scan();
///	io.println(format!(("x + y = {}", x + y));
/// ```
pub mod io {
    use std::io::{stdout, BufWriter, Read, StdoutLock, Write};

    pub struct IO {
        iter: std::str::SplitAsciiWhitespace<'static>,
        buf: BufWriter<StdoutLock<'static>>,
    }
    
    impl IO {
        pub fn new() -> Self {
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            let input = Box::leak(input.into_boxed_str());
            let out = Box::new(stdout());
            IO {
                iter: input.split_ascii_whitespace(),
                buf: BufWriter::new(Box::leak(out).lock()),
            }
        }
        fn scan_str(&mut self) -> &'static str {
            self.iter.next().unwrap()
        }
        fn scan_raw(&mut self) -> &'static [u8] {
            self.scan_str().as_bytes()
        }
        pub fn scan<T: Scan>(&mut self) -> T {
            T::scan(self)
        }
        pub fn scan_vec<T: Scan>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.scan()).collect()
        }
    }
    
    impl IO {
        pub fn print<T: Print>(&mut self, x: T) {
            T::print(self, x);
        }
        pub fn println<T: Print>(&mut self, x: T) {
            self.print(x);
            self.print("\n");
        }
        pub fn iterln<T: Print, I: Iterator<Item = T>>(&mut self, mut iter: I, delim: &str) {
            if let Some(v) = iter.next() {
                self.print(v);
                for v in iter {
                    self.print(delim);
                    self.print(v);
                }
            }
            self.print("\n");
        }
        pub fn flush(&mut self) {
            self.buf.flush().unwrap();
        }
    }
    
    pub trait Scan {
        fn scan(io: &mut IO) -> Self;
    }
    
    macro_rules! impl_parse_int {
        ($($t:tt),*) => {
            $(
                impl Scan for $t {
                    fn scan(s: &mut IO) -> Self {
                        let mut res = 0;
                        for d in s.scan_raw() {
                            res *= 10;
                            res += (*d - b'0') as $t;
                        }
                        res
                    }
                }
            )*
        };
    }
    
    impl_parse_int!(i32, i64, isize, u32, u64, usize);
    
    impl<T: Scan, U: Scan> Scan for (T, U) {
        fn scan(s: &mut IO) -> Self {
            (T::scan(s), U::scan(s))
        }
    }
    
    impl<T: Scan, U: Scan, V: Scan> Scan for (T, U, V) {
        fn scan(s: &mut IO) -> Self {
            (T::scan(s), U::scan(s), V::scan(s))
        }
    }
    
    pub trait Print {
        fn print(w: &mut IO, x: Self);
    }
    
    macro_rules! impl_print_int {
        ($($t:ty),*) => {
            $(
                impl Print for $t {
                    fn print(w: &mut IO, x: Self) {
                        w.buf.write_all(x.to_string().as_bytes()).unwrap();
                    }
                }
            )*
        };
    }
    
    impl_print_int!(i32, i64, isize, u32, u64, usize);
    
    impl Print for u8 {
        fn print(w: &mut IO, x: Self) {
            w.buf.write_all(&[x]).unwrap();
        }
    }
    
    impl Print for &[u8] {
        fn print(w: &mut IO, x: Self) {
            w.buf.write_all(x).unwrap();
        }
    }
    
    impl Print for &str {
        fn print(w: &mut IO, x: Self) {
            w.print(x.as_bytes());
        }
    }

    impl Print for String {
        fn print(w: &mut IO, x: Self) {
            w.print(x.as_bytes());
        }
    }
    
    impl<T: Print, U: Print> Print for (T, U) {
        fn print(w: &mut IO, (x, y): Self) {
            w.print(x);
            w.print(" ");
            w.print(y);
        }
    }
    
    impl<T: Print, U: Print, V: Print> Print for (T, U, V) {
        fn print(w: &mut IO, (x, y, z): Self) {
            w.print(x);
            w.print(" ");
            w.print(y);
            w.print(" ");
            w.print(z);
        }
    }
}