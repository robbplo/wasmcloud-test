wit_bindgen::generate!({ generate_all });

use wasmcloud_component::wasi::random::random;
use crate::exports::robbin::random_number::random_number::Guest;

struct RandomNumber;

impl Guest for RandomNumber {
    fn test() -> u64 {
        // random::get_random_u64()
        return 1;
    }
}

export!(RandomNumber);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        assert_eq!(RandomNumber::test(), 1);
    }
}
