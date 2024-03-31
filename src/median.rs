/// Intermediary median value which isn't converted to a single value yet.
#[must_use = "obtaining a median without using it makes no sense"]
pub enum RawMedian<T> {
    Odd(T),
    Even(T, T),
}
