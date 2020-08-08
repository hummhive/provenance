use clap::App;

pub(crate) fn app<'a, 'b>() -> App<'a, 'b> {
    App::new(super::constant::NAME)
        .version(super::constant::VERSION)
        .author(super::constant::AUTHOR)
        .about(super::constant::ABOUT)
        .arg(crate::jwt::cli::arg())
        .arg(crate::pubkey::cli::arg())
}
