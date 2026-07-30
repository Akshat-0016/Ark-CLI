#[test]
fn stem_words() {
    let stemmer = Stemmer::new();

    assert_eq!(stemmer.stem("running"), "runn");
    assert_eq!(stemmer.stem("played"), "play");
    assert_eq!(stemmer.stem("boxes"), "box");
    assert_eq!(stemmer.stem("cars"), "car");
}
