use clap::Arg;

pub(crate) fn arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name(super::constant::NAME)
        .required(true)
        .require_equals(true)
        .multiple(false)
        .number_of_values(1)
        .case_insensitive(true)
        .takes_value(true)
        .value_name(super::constant::NAME)
        .long(super::constant::NAME)
        .short(super::constant::SHORT)
        .help(super::constant::HELP)
}
