fn main() {
    println!("Hello, world!");
}

struct Studio();

impl Studio {
    pub(crate) fn makeSound(&self) -> bool {
        false
    }
}

#[test]
fn should_make_no_sound_when_nothing(){
    let studio = Studio();
    assert_eq!(false, studio.makeSound());
}