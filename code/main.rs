fn main() {
    let name = "erzbir";
    let languages = ["Java", "Python", "JavaScript", "Html", "C#", "C"];
    let hobbies = ["Irish flute", "Tin whistle", "Yu-Gi-Oh OCG", "Riding"];
    let learnings = ["JVM", "Spring Cloud", "Kotlin", "Vue, Assembly, everything, ......"];
    let blog = "https://erzbir.com";
    let email = "erzbir@mail.com";

    println!("About Me:");
    println!("  name: \n    {}", name);
    println!("  languages:    ");
    println!("    {:?}", languages);
    println!("  hobbies:    ");
    println!("    {:?}", hobbies);
    println!("  learnings:    ");
    println!("    {:?}", learnings);
    println!("  blog:\n    {}", blog);
    println!("  email:\n    {}", email);
}