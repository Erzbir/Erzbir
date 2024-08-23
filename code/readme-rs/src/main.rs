#[derive(Debug)]
struct ReadMe {
    name: &'static str,
    blog: &'static str,
    email: &'static str,
    focus: &'static str,
}

trait Hobbies {
    fn ocg(&self) {}
    fn irish_flute(&self) {}
    fn mtb(&self) {}
}

impl Hobbies for ReadMe {}

const ME: ReadMe = ReadMe {
    name: "Erzbir",
    blog: "https://erzbir.com",
    email: "erzbir@mail.com",
    focus: "Cyber Security",
};

fn main() {
    println!("{:#?}", ME);
}
