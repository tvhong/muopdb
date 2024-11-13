use std::cmp::{Ord, Ordering};
use std::collections::HashSet;

use roaring::RoaringBitmap;

pub struct SearchContext {
    pub visited: RoaringBitmap,
    pub record_pages: bool,
    pub visited_pages: Option<HashSet<String>>,
}

impl SearchContext {
    pub fn new(record_pages: bool) -> Self {
        if !record_pages {
            Self {
                visited: RoaringBitmap::new(),
                record_pages: false,
                visited_pages: None,
            }
        } else {
            Self {
                visited: RoaringBitmap::new(),
                record_pages: true,
                visited_pages: Some(HashSet::new()),
            }
        }
    }

    pub fn num_pages_accessed(&self) -> usize {
        if !self.record_pages {
            return 0;
        }

        self.visited_pages.as_ref().unwrap().len()
    }
}

#[derive(Debug)]
pub struct IdWithScore {
    pub id: u64,
    pub score: f32,
}

impl Ord for IdWithScore {
    fn cmp(&self, other: &Self) -> Ordering {
        // Handle NaN cases first
        if self.score.is_nan() && other.score.is_nan() {
            self.id.cmp(&other.id) // Both are NaN, consider them equal, tie-break by id
        } else if self.score.is_nan() {
            Ordering::Greater // This instance is NaN, consider it greater
        } else if other.score.is_nan() {
            Ordering::Less // Other instance is NaN, consider it less
        } else {
            match self.score.partial_cmp(&other.score) {
                // Tie-break by id
                Some(Ordering::Equal) => self.id.cmp(&other.id),
                Some(order) => order,
                // Handle unexpected cases (shouldn't happen with valid inputs)
                None => Ordering::Equal,
            }
        }
    }
}

impl PartialOrd for IdWithScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for IdWithScore {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.id == other.id
    }
}

impl Eq for IdWithScore {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_with_score_ord() {
        let a = IdWithScore { id: 2, score: 1.0 };
        let b = IdWithScore { id: 1, score: 2.0 };
        let c = IdWithScore { id: 1, score: 1.0 };
        let d = IdWithScore { id: 2, score: 1.0 };
        let e = IdWithScore { id: 1, score: 1.0 };

        // Test basic comparison
        assert!(a < b); // a has a smaller score than b
        assert!(b > a); // b has a larger score than a

        // Test tie-breaking by id
        assert!(c < d); // c has the same score as d but a smaller id

        // Test equality
        assert_eq!(c, e); // c and e are equal in terms of score and id

        // Test NaN handling
        let f = IdWithScore {
            id: 3,
            score: f32::NAN,
        };
        let g = IdWithScore {
            id: 4,
            score: f32::NAN,
        };

        assert!(f < g); // f has a smaller id than g
        assert!(a < f); // f is NaN
    }

    #[test]
    fn test_id_with_score_sorting() {
        let mut scores = vec![
            IdWithScore {
                score: f32::NAN,
                id: 5,
            },
            IdWithScore { score: 1.0, id: 2 },
            IdWithScore { score: 1.0, id: 1 },
            IdWithScore { score: 3.0, id: 0 },
            IdWithScore {
                score: f32::NAN,
                id: 4,
            },
            IdWithScore { score: 2.0, id: 1 },
        ];

        scores.sort();

        let expected_order = vec![
            IdWithScore { score: 1.0, id: 1 },
            IdWithScore { score: 1.0, id: 2 },
            IdWithScore { score: 2.0, id: 1 },
            IdWithScore { score: 3.0, id: 0 },
            IdWithScore {
                score: f32::NAN,
                id: 4,
            },
            IdWithScore {
                score: f32::NAN,
                id: 5,
            },
        ];

        for (a, b) in scores.iter().zip(expected_order.iter()) {
            if a.score.is_nan() {
                assert!(b.score.is_nan());
                assert_eq!(a.id, b.id);
            } else {
                assert_eq!(a.score, b.score);
                assert_eq!(a.id, b.id);
            }
        }
    }
}