fn main() {
    println!("Hello, world!");
}

struct Studio {
    instrument: Option<Guitar>,
}

struct Guitar();

impl Studio {
    pub(crate) fn isGuitarPlugged(&self) -> bool {
        self.instrument.is_some()
    }

    pub(crate) fn plug(&mut self, guitar: Guitar) {
        self.instrument = Some(guitar);
    }

    pub(crate) fn makeSound(&self) -> bool {
        false
    }
}

#[test]
fn should_make_no_sound_when_nothing() {
    let studio = Studio { instrument: None };
    assert_eq!(false, studio.makeSound());
}

#[test]
fn should_make_no_sound_when_guitar_is_plugged() {
    let mut studio = Studio { instrument: None };
    studio.plug(Guitar());
    assert_eq!(true, studio.isGuitarPlugged());
    assert_eq!(false, studio.makeSound());
}

