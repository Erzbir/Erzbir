const ME: ReadMe = ReadMe {
    name: "Erzbir",
    blog: "https://erzbir.com",
    email: "erzbir@mail.com",
    focus: "Cyber Security",
};

trait TechStack {
    fn security(&self) {}
    fn develop(&self) {}
}

trait Languages {
    fn java(&self) {}
    fn php(&self) {}
    fn python(&self) {}
    fn rust(&self) {}
    fn c(&self) {}
}

trait Hobbies {
    fn ocg(&self) {}
    fn irish_flute(&self) {}
    fn mtb(&self) {}
}

use macro_derive::{Hobbies, Languages, TechStack};

#[derive(Debug, Languages, Hobbies, TechStack)]
struct ReadMe {
    name: &'static str,
    blog: &'static str,
    email: &'static str,
    focus: &'static str,
}

fn main() {
    println!("{:#?}", ME);
}
