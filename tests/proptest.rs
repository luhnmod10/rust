extern crate proptest;
use proptest::prelude::*;

proptest! {
    #[test]
    fn doesnt_crash(s in r"[0-9]{13,16}") {
        let valid = luhnmod10::valid(&s);
        println!("{} = {}", s, valid);
    }
}
