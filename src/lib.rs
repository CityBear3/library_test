use std::u32;

use pyo3::{prelude::*, wrap_pyfunction};

//コラッツ予想を試行する
#[pyfunction]
fn rs_collatz(arg: u128) -> PyResult<u128> {
    let mut n = arg;
    while n > 1 {
        n = if n % 2 == 0 {
            n / 2
        } else {
            3 * n + 1
        };

        println!("{}", n);
    };

    Ok(n)
}


#[pymodule]
fn test_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(rs_collatz))?;
    Ok(())
}

//単体テスト用関数
pub fn collatz_test(arg: u128) -> u32 {
    let n = rs_collatz(arg).unwrap() as u32;
    n
}


/*
    GitHub Actions用テストコード
    macOSでテストする場合はコメントアウトしてください
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(1, collatz_test(999));
    }

    #[test]
    fn it_work_2() {
        assert_ne!(2, collatz_test(427970));
    }
}