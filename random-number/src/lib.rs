wit_bindgen::generate!({ generate_all });

use wasmcloud_component::wasi::random::random;
use crate::exports::robbin::random_number::random_number::Guest;

export!(RandomNumber);

struct RandomNumber;

impl Guest for RandomNumber {
    fn test() -> u64 {
        random::get_random_u64()
    }
}
