pub struct Currency(pub String);

#[allow(dead_code)]
pub struct Rate {
    from: Currency,
    to: Currency,
    rate: f32,
}
