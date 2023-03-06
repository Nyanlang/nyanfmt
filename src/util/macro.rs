macro_rules! ts {
    () => {
        TokenStream::new()
    };
    ($($tok: expr),* $(,)?) => {
        TokenStream::from(&[$($tok,)*][..])
    };
}

macro_rules! head_option {
	($h: ident,) => {
		None
	};
	($h: ident, $($tok: expr),*) => {
		Some($h(vec![$($tok),*]))
	};
}

macro_rules! word {
    (
        $([$($head: expr),* $(,)?])?,
        $([$($body: expr),* $(,)?])?,
        $([$($tail: expr),* $(,)?])?
        $(,)?
    ) => {
        Word {
            head: head_option!(Head, $($($head),*)?),
            body: head_option!(Body, $($($body),*)?),
            tail: head_option!(Tail, $($($tail),*)?),
        }
    };
}
