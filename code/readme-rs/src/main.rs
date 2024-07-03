use macro_derive::{Hobbies, Languages, TechStack};

#[derive(Debug, Languages, Hobbies, TechStack)]
struct ReadMe {
    name: &'static str,
    blog: &'static str,
    email: &'static str,
}

const ME: ReadMe = ReadMe {
    name: "Erzbir",
    blog: "https://erzbir.com",
    email: "erzbir@mail.com",
};

trait TechStack {
    fn develop(&self) -> &str { "[Framework, backend, frontend, ...]" }
    fn security(&self) -> &str { "Attacking and defending" }
    fn network(&self) -> &str { "[BPG, OSPF, ...]" }
}

trait Languages {
    fn rust(&self) -> &str { "I'm learning now" }
    fn java(&self) -> &str { "Webapps and cross-platform apps" }
    fn python(&self) -> &str { "Scripts" }
    fn php(&self) -> &str { "I Focusing on security" }
    fn c(&self) -> &str { "My first programming language" }
}

trait Hobbies {
    fn ocg(&self) -> &str { "OCG is my favorite game" }
    fn irish_flute(&self) -> &str { "I've been learning the flute since 2017" }
    fn mtb(&self) -> &str { "Mountain biking!" }
}

fn main() {
    let languages = 'Languages: {
        [
            ME.rust(),
            ME.java(),
            ME.python(),
        ]
    };
    let hobbies = 'Hobbies: {
        [
            ME.ocg(),
            ME.irish_flute(),
            ME.mtb(),
        ]
    };

    let tech = 'TechStack: {
        [
            ME.develop(),
            ME.security(),
            ME.network(),
        ]
    };

    println!("{:#?}", ME);
    println!("{:#?}", languages);
    println!("{:#?}", hobbies);
    println!("{:#?}", tech);
}