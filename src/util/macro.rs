macro_rules! ts {
    () => {
        TokenStream::new()
    };
    ($($tok: expr),* $(,)?) => {
        TokenStream::from(&[$($tok,)*][..])
    };
}
