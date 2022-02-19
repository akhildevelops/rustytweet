mod defaults;
mod endpoints;
mod external;
mod search;

pub use search::search_tweets;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request() {
        assert_eq!(
            search_tweets("440322224407314432"),
            "If only Bradley's arm was longer. Best photo ever. #oscars http://t.co/C9U5NOtGap"
        )
    }
}
