#[cfg(feature = "feature")]
const VAL: &str = "feature";
#[cfg(not(feature = "feature"))]
const VAL: &str = "no feature";

pub fn hello() -> &'static str {
    VAL
}
