fn main() {
    println!("Hello, world!");
}

struct Studio {
    soundcard: Option<SoundCard>,
}

#[derive(Copy, Clone)]
struct Guitar();


#[derive(Copy, Clone)]
struct SoundCard {
    instrument: Option<Guitar>,
}

impl SoundCard {
    pub(crate) fn plug(&mut self, guitar: Guitar) {
        self.instrument = Some(guitar);
    }

    pub(crate) fn isGuitarPlugged(&self) -> bool {
        self.instrument.is_some()
    }
}

impl Studio {
    pub(crate) fn isSoundcardPlugged(&self) -> bool {
        self.soundcard.is_some()
    }

    pub(crate) fn plug(&mut self, soundcard: SoundCard) {
        self.soundcard = Some(soundcard);
    }

    pub(crate) fn makeSound(&self) -> bool {
        self.isSoundcardPlugged() && self.soundcard.unwrap().isGuitarPlugged()
    }
}

#[test]
fn should_make_no_sound_when_nothing() {
    let studio = Studio { soundcard: None };
    assert_eq!(false, studio.makeSound());
}


#[test]
fn should_plug_guitar_in_sound_card() {
    let mut sound_card = SoundCard { instrument: None };
    sound_card.plug(Guitar());
    assert_eq!(true, sound_card.isGuitarPlugged());
}

#[test]
fn should_plug_sound_card_to_studio() {
    let mut studio = Studio { soundcard: None };
    let sound_card = SoundCard { instrument: None };
    studio.plug(sound_card);
    assert!(studio.isSoundcardPlugged());
    assert!(!studio.makeSound());
}

#[test]
fn should_plug_guitar_and_soundcard_and_make_sound() {
    let mut studio = Studio { soundcard: None };
    let mut sound_card = SoundCard { instrument: None };
    sound_card.plug(Guitar());
    studio.plug(sound_card);
    assert!(studio.makeSound());
}
