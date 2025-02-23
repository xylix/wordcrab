extern crate wordcrab;
use wordcrab::*;
use quickcheck;

mod analysis_options;
use analysis_options::*;

fn qc_weird_string(string: String) {
    // print!("Analyzed string: {}", &string);
    let stats = analyse_string(&string, ANALYSIS_OPTIONS_LWC);
    print!("{}", stats);
    assert!(stats.words >= Some(0));
}

#[test]
fn main() {
    let mut qc = quickcheck::QuickCheck::new()
        .min_tests_passed(100)
        // Attempt to run 1000 tests 
        .tests(200)
        // Max size of generated strings
        .gen(quickcheck::Gen::new(4000));
    qc.quickcheck(qc_weird_string as fn(String) -> ());
}
