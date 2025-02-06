#[salsa::db]
pub trait Db: salsa::Database {}

#[salsa::db]
#[derive(Default, Clone)]
pub struct DbImpl {
    storage: salsa::Storage<Self>,
}

#[salsa::db]
impl salsa::Database for DbImpl {
    fn salsa_event(&self, event: &dyn Fn() -> salsa::Event) {
        let event = event();
        eprintln!("Event: {event:?}");
    }
}

#[salsa::db]
impl Db for DbImpl {}
