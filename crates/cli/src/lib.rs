pub struct Command {
    pub usage: &'static str,
    pub parameters: &'static [&'static str],
    pub description: &'static str,
}

#[macro_export]
macro_rules! command {
    ($usage:expr, $description:expr) => {
        ::ferrite_cli::Command {
            usage: $usage,
            parameters: &[],
            description: $description,
        }
    };
    ($usage:expr, $parameters:expr, $description:expr) => {
        ::ferrite_cli::Command {
            usage: $usage,
            parameters: &$parameters,
            description: $description,
        }
    };
}
