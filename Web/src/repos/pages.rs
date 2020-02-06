use crate::pages::{
    page::Page,
    home::Home,
    alphabet::Alphabet,
    prepositions::Prepositions,
};

pub fn get() -> [Box<dyn Page<'static>>; 3] {
    [
        Box::new(Home { }),
        Box::new(Alphabet { }),
        Box::new(Prepositions { }),
    ]
}
