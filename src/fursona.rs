mod fursona_instance;

use crate::stage::Stage;

pub use fursona_instance::FursonaInstance;

pub struct Fursona {
    pub name: String,
}

impl Fursona {
    pub fn make_instance(&self, stage: &Stage) -> FursonaInstance {
        FursonaInstance::new(self, stage)
    }
}
