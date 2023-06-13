use rrrand::prelude::*;

/// # Examples
///
/// ```
/// use rust_doc_test_aliases::number;
/// let x = number();
/// ```

pub fn number() -> i32 {
    thread_rng().gen()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = number();
    }
}
