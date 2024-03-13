#[derive(Debug)]
struct ReadMe {
    name: &'static str,
    blog: &'static str,
    email: &'static str,
}

trait Ability {
    fn rust(&self) -> &str { "https://www.rust-lang.org" }
    fn java(&self) -> &str { "https://www.java.com" }
    fn python(&self) -> &str { "https://www.python.org" }
    fn csharp(&self) -> &str { "https://learn.microsoft.com/dotnet/csharp/" }
    fn javascript(&self) -> &str { "https://www.javascript.com" }
    fn typescript(&self) -> &str { "https://www.typescriptlang.org" }
}

trait Hobby {
    fn ocg(&self) -> &str { "https://yugioh.fandom.com/wiki/Yu-Gi-Oh!_Official_Card_Game" }
    fn irish_flute(&self) -> &str { "https://en.wikipedia.org/wiki/Irish_flute" }
    fn riding(&self) {}
}

impl Ability for ReadMe {}

impl Hobby for ReadMe {}

macro_rules! info {
    () => {
        print!("\n")
    };
    ($($arg:tt)*) => {
        println!("{:#?}", $($arg)*)
    };
}

fn main() {
    let user = ReadMe {
        name: "Erzbir",
        blog: "https://erzbir.com",
        email: "erzbir@mail.com",
    };
    info!(user);
}
