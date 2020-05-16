const LOCATION_ENV: &str = "HUMM_PROVENANCE_ROUGHTIME_ECOSYSTEM";

pub struct Location(Path);

impl Location {
    pub fn from_env() -> Result<Self {


pub fn location() ->

#[cfg(test)]
pub mod test {

    #[test]
    /// need an ecosystem.json file to test against
    fn ecosystem_location() {
        let location = std::env::var();

        match location {
            Ok(_) => println!("roughtime ecosystem location: {:?}", location),
            Err(_) => panic!(format!("HUMM_PROVENANCE_ROUGHTIME_ECOSYSTEM env not set: {:?}", location)),
        }
    }

    #[test]
    /// need to be able to load and parse an ecosystem
    fn ecosystem_load()
}
