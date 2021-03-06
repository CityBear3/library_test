mod test_lib;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        assert_eq!(1, test_lib::collatz_test(999));
    }

    #[test]
    fn it_work_2() {
        assert_ne!(2, test_lib::collatz_test(427970));
    }
}



