use crate::pages::{
    page::Page,
    home::Home,
    alphabet::Alphabet,
    prepositions::Prepositions,
    the_home::TheHome,
};

pub fn get() -> [Box<dyn Page<'static>>; 4] {
    [
        Box::new(Home { }),
        Box::new(Alphabet { }),
        Box::new(Prepositions { }),
        Box::new(TheHome { }),
    ]
}
