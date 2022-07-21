#[cfg(not(feature = "gecko"))]
mod metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}

#[cfg(feature = "gecko")]
mod metrics {
    pub use ::fog::metrics::is_awesome;
}

pub fn is_awesome() -> bool {
    metrics::is_awesome::awesomeness.add(1);
    // it probably is
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_it() {
        assert!(is_awesome());
    }
}
