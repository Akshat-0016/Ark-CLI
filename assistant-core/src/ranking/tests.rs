use crate::{ranking::ranker::Ranker, search::result::SearchResult};

#[test]
fn rank_results() {
    let ranker = Ranker::new();

    let results = vec![
        SearchResult::new(1, 2),
        SearchResult::new(2, 7),
        SearchResult::new(3, 1),
    ];

    let ranked = ranker.rank(&results);

    assert_eq!(ranked[0].document_id(), 2);
}
