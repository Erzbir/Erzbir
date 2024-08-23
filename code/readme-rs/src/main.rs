#[derive(Debug)]
struct ReadMe {
    name: &'static str,
    blog: &'static str,
    email: &'static str,
    focus: &'static str,
    hobbies: &'static str,
}

const ERZBIR: ReadMe = ReadMe {
    name: "Erzbir",
    blog: "https://erzbir.com",
    email: "erzbir@mail.com",
    focus: "Cyber Security",
    hobbies: "[Irish Flute, OCG, Biking]"
};

fn main() {
    println!("{:#?}", ERZBIR);
}
