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
![Code Time](http://img.shields.io/badge/Code%20Time-809%20hrs%208%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-1-blue)

**🐱 My GitHub Data** 

> 📦 173.3 kB Used in GitHub's Storage 
 > 
> 🏆 152 Contributions in the Year 2024
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 34 Public Repositories 
 > 
> 🔑 7 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                212 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.00 % 
🌆 Daytime                351 commits         ███████░░░░░░░░░░░░░░░░░░   28.15 % 
🌃 Evening                372 commits         ███████░░░░░░░░░░░░░░░░░░   29.83 % 
🌙 Night                  312 commits         ██████░░░░░░░░░░░░░░░░░░░   25.02 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   188 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.08 % 
Tuesday                  296 commits         ██████░░░░░░░░░░░░░░░░░░░   23.74 % 
Wednesday                165 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.23 % 
Thursday                 213 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.08 % 
Friday                   87 commits          ██░░░░░░░░░░░░░░░░░░░░░░░   06.98 % 
Saturday                 166 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.31 % 
Sunday                   132 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.59 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     7 hrs 14 mins       ████████████░░░░░░░░░░░░░   49.74 % 
XML                      1 hr 33 mins        ███░░░░░░░░░░░░░░░░░░░░░░   10.74 % 
Rust                     1 hr 32 mins        ███░░░░░░░░░░░░░░░░░░░░░░   10.57 % 
TOML                     1 hr 5 mins         ██░░░░░░░░░░░░░░░░░░░░░░░   07.47 % 
HTTP Request             33 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   03.84 % 

🔥 Editors: 
Intellijidea             10 hrs 26 mins      ██████████████████░░░░░░░   71.81 % 
RustRover                2 hrs 54 mins       █████░░░░░░░░░░░░░░░░░░░░   20.01 % 
CLion                    1 hr 3 mins         ██░░░░░░░░░░░░░░░░░░░░░░░   07.29 % 
PyCharm                  7 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.88 % 

🐱‍💻 Projects: 
student-sys              7 hrs 19 mins       █████████████░░░░░░░░░░░░   50.37 % 
dispatcher               1 hr 33 mins        ███░░░░░░░░░░░░░░░░░░░░░░   10.74 % 
clash-verge-rev          1 hr 13 mins        ██░░░░░░░░░░░░░░░░░░░░░░░   08.40 % 
shellcode                1 hr 3 mins         ██░░░░░░░░░░░░░░░░░░░░░░░   07.29 % 
untitled3                50 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   05.73 % 

💻 Operating System: 
Mac                      14 hrs 32 mins      █████████████████████████   100.00 % 
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


 Last Updated on 10/06/2024 18:38:40 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  <b>Visitor Count</b><br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg"  alt="count"/>
</p>
