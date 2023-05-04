pub fn index_to_row(index: usize) -> usize {
    let i = index;
    return (i / 8) as usize;
}

pub fn index_to_column(index: usize) -> usize {
    let i = index;
    return (i % 8) as usize;
}

pub fn fprofile<P>(name: &str, func: fn() -> P) -> P {
    use std::time::SystemTime;
    let mut duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let start = duration_since_epoch.as_nanos(); // u128
    let res = func();
    duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let end = duration_since_epoch.as_nanos(); // u128
    println!("{:?} took {:?} ns", name, end - start);
    return res;
}

#[cfg(test)]
mod tests {
    use crate::util::index_to_column;
    use crate::util::index_to_row;

    #[test]
    fn row_tests () {
        assert_eq!(1, index_to_row(8));
        assert_eq!(0, index_to_row(7));
        assert_eq!(3, index_to_row(31));
        assert_eq!(0, index_to_row(1));
    }
    #[test]
    fn column_tests () {
        assert_eq!(0, index_to_column(8));
        assert_eq!(7, index_to_column(7));
        assert_eq!(1, index_to_column(1));
    }
}
