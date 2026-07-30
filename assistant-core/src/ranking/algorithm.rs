use super::{bm25::BM25Ranker, frequency::FrequencyRanker, ranker::Ranker, tfidf::TFIDFRanker};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RankingAlgorithm {
    Frequency,
    TfIdf,
    BM25,
}

impl Default for RankingAlgorithm {
    fn default() -> Self {
        Self::BM25
    }
}

impl RankingAlgorithm {
    pub fn create(self) -> Box<dyn Ranker> {
        match self {
            Self::Frequency => Box::new(FrequencyRanker::new()),
            Self::TfIdf => Box::new(TFIDFRanker::new()),
            Self::BM25 => Box::new(BM25Ranker::new()),
        }
    }
}
