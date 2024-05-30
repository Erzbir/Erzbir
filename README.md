## About Me:

<p>
<a href="https://erzbir.com">
    <img src="https://img.shields.io/badge/Website-erzbir.com-red?style=flat-square">
</a>  
<a href="mailto:erzbir@mail.com">
    <img src="https://img.shields.io/badge/-Email-red?style=flat-square&logo=gmail&logoColor=white">
</a>
</p>

<!-- <details>
<summary>Code</summary>
</br> -->

```rust
#[derive(Debug)]
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

pub trait Expertise {
    fn rust(&self) -> &str { "Rust is my new language" }
    fn java(&self) -> &str { "Java is my main language" }
    fn python(&self) -> &str { "I use python in ai or scripts" }
    fn csharp(&self) -> &str { "C# for Unity3D" }
    fn javascript(&self) -> &str { "JavaScript for frontend" }
    fn typescript(&self) -> &str { "TypeScript for frontend" }
}

pub trait Hobby {
    fn ocg(&self) -> &str { "OCG is my favorite game" }
    fn irish_flute(&self) -> &str { "I have been learning flute since 2017" }
    fn mtb(&self) -> &str { "Mountain biking!" }
}

impl Expertise for ReadMe {}

impl Hobby for ReadMe {}


fn main() {
    let expertises = 'Expertises: {
        [
            ME.rust(),
            ME.java(),
            ME.python(),
            ME.csharp(),
            ME.javascript(),
            ME.typescript(),
        ]
    };
    let hobbies = 'Hobbies: {
        [
            ME.ocg(),
            ME.irish_flute(),
            ME.mtb(),
        ]
    };

    println!("{:#?}", ME);
    println!("{:#?}", expertises);
    println!("{:#?}", hobbies);
}
```

<details>
<summary>Console Print</summary>

```text
ReadMe {
    name: "Erzbir",
    blog: "https://erzbir.com",
    email: "erzbir@mail.com",
}
[
    "Rust is my new language",
    "Java is my main language",
    "I use python in ai or scripts",
    "C# for Unity3D",
    "JavaScript for frontend",
    "TypeScript for frontend",
]
[
    "OCG is my favorite game",
    "I have been learning flute since 2017",
    "Mountain biking!",
]
```

</details>

##

<details>
<summary>GitHub Stats</summary>
<br>

<a href="https://github.com/Erzbir">
<img src="https://github-profile-summary-cards.vercel.app/api/cards/profile-details?username=Erzbir&theme=tokyonight" width="95%" alt="summary">
</a>

<a href="https://github.com/Erzbir">
    <img src="https://github-readme-stats.vercel.app/api?username=Erzbir&layout=compact&show_icons=true&theme=tokyonight" width="50%" alt="stats">
</a>

<a href="https://github.com/Erzbir">
    <img src="https://github-readme-stats.vercel.app/api/top-langs/?username=Erzbir&layout=compact&theme=tokyonight" width="45%" alt="language">
</a>

</details>


##

<details>
<summary>📈 Wakatime Stats</summary>
<br>

![Erzbir's wakatime stats](https://github-readme-stats.vercel.app/api/wakatime?username=Erzbir\&layout=compact)

##

<!--START_SECTION:waka-->
![Code Time](http://img.shields.io/badge/Code%20Time-770%20hrs%2035%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-1-blue)

**🐱 My GitHub Data** 

> 📦 115.7 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 33 Public Repositories 
 > 
> 🔑 7 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                204 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.09 % 
🌆 Daytime                325 commits         ███████░░░░░░░░░░░░░░░░░░   27.22 % 
🌃 Evening                360 commits         ████████░░░░░░░░░░░░░░░░░   30.15 % 
🌙 Night                  305 commits         ██████░░░░░░░░░░░░░░░░░░░   25.54 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   181 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.16 % 
Tuesday                  289 commits         ██████░░░░░░░░░░░░░░░░░░░   24.20 % 
Wednesday                160 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.40 % 
Thursday                 211 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.67 % 
Friday                   78 commits          ██░░░░░░░░░░░░░░░░░░░░░░░   06.53 % 
Saturday                 152 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.73 % 
Sunday                   123 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.30 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     9 hrs 20 mins       ███████████████████░░░░░░   76.55 % 
XML                      1 hr 7 mins         ██░░░░░░░░░░░░░░░░░░░░░░░   09.25 % 
Kotlin                   25 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   03.47 % 
HTTP Request             22 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   03.12 % 
YAML                     19 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.61 % 

🔥 Editors: 
Intellijidea             12 hrs 8 mins       █████████████████████████   99.44 % 
RustRover                4 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.56 % 

🐱‍💻 Projects: 
student-sys              10 hrs 29 mins      █████████████████████░░░░   85.91 % 
StudentSys               30 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   04.17 % 
sigma2-main              29 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   04.01 % 
hotel                    8 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   01.21 % 
account-book             8 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   01.10 % 

💻 Operating System: 
Mac                      12 hrs 12 mins      █████████████████████████   100.00 % 
```

**I Mostly Code in Java** 

```text
Java                     12 repos            ██████████████░░░░░░░░░░░   54.55 % 
HTML                     2 repos             ██░░░░░░░░░░░░░░░░░░░░░░░   09.09 % 
Rust                     2 repos             ██░░░░░░░░░░░░░░░░░░░░░░░   09.09 % 
C#                       1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.55 % 
C                        1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.55 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 30/05/2024 18:39:17 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  <b>Visitor Count</b><br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg"  alt="count"/>
</p>
