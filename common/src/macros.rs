/// Panics with an error
#[macro_export]
macro_rules! bail {
    ($report:expr) => {{
        let report: miette::Report = $report.into();
        panic!("{report:?}");
    }};
}

/// Panic with a bug error
#[macro_export]
macro_rules! bug {
    ($text:expr) => {{
        panic!("{:?}", miette::miette!($text));
    }};
}

/// Prints a warn
#[macro_export]
macro_rules! warn {
    ($text:expr) => {{
        println!("{:?}", miette::miette!($text));
    }};
}
