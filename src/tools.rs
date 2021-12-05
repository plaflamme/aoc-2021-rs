use itertools::Itertools;

/// Takes an iterator over lines and returns an iterator over batches of lines between empty lines.
pub fn empty_line_delimited_batches<'a>(
    it: impl Iterator<Item = &'a str>,
) -> impl Iterator<Item = Vec<&'a str>> {
    it.batching(|it| {
        let batch = it
            .skip_while(|l| l.trim().is_empty())
            .take_while(|l| !l.trim().is_empty())
            .collect_vec();

        if batch.is_empty() {
            None
        } else {
            Some(batch)
        }
    })
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    #[test]
    fn test_empty_line_delimited() {
        let sample = "batch1

batch2
batch2


batch3

";

        let batches = super::empty_line_delimited_batches(sample.lines()).collect_vec();
        assert_eq!(
            batches,
            vec![vec!["batch1"], vec!["batch2", "batch2"], vec!["batch3"]]
        );
    }
}
