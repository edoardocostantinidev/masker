use masker::Mask;

#[derive(Mask)]
struct ToMask {
    #[mask]
    mask_it: String,
    dont_mask_it: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn masker_test() {
        let to_mask = ToMask {
            mask_it: "Abc".to_string(),
            dont_mask_it: "Abc".to_string(),
        };
        //assert_eq!(format!("{to_mask}"), "");
        assert_eq!(
            format!("{:?}", to_mask),
            "ToMask { mask_it: \"***\", dont_mask_it: \"Abc\" }"
        );
    }
}
