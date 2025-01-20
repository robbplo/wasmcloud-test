wit_bindgen::generate!({ generate_all });

use wasmcloud_component::wasi::random::random;
use crate::exports::wasmcloud::random_number::random_number::Guest;

struct RandomNumber;

impl Guest for RandomNumber {
    fn random_int() -> u64 {
        random::get_random_u64()
    }
}
