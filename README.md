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
![Code Time](http://img.shields.io/badge/Code%20Time-797%20hrs%2029%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-1-blue)

**🐱 My GitHub Data** 

> 📦 122.2 kB Used in GitHub's Storage 
 > 
> 🏆 136 Contributions in the Year 2024
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 33 Public Repositories 
 > 
> 🔑 7 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                212 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.29 % 
🌆 Daytime                341 commits         ███████░░░░░░░░░░░░░░░░░░   27.81 % 
🌃 Evening                362 commits         ███████░░░░░░░░░░░░░░░░░░   29.53 % 
🌙 Night                  311 commits         ██████░░░░░░░░░░░░░░░░░░░   25.37 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   183 commits         ████░░░░░░░░░░░░░░░░░░░░░   14.93 % 
Tuesday                  296 commits         ██████░░░░░░░░░░░░░░░░░░░   24.14 % 
Wednesday                161 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.13 % 
Thursday                 211 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.21 % 
Friday                   80 commits          ██░░░░░░░░░░░░░░░░░░░░░░░   06.53 % 
Saturday                 165 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.46 % 
Sunday                   130 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.60 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     30 hrs 13 mins      ███████████████████░░░░░░   77.31 % 
XML                      5 hrs 12 mins       ███░░░░░░░░░░░░░░░░░░░░░░   13.32 % 
HTTP Request             1 hr 14 mins        █░░░░░░░░░░░░░░░░░░░░░░░░   03.18 % 
Kotlin                   1 hr 11 mins        █░░░░░░░░░░░░░░░░░░░░░░░░   03.03 % 
YAML                     20 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   00.89 % 

🔥 Editors: 
Intellijidea             39 hrs 1 min        █████████████████████████   99.82 % 
RustRover                4 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.18 % 

🐱‍💻 Projects: 
student-sys              36 hrs              ███████████████████████░░   92.09 % 
StudentSys               1 hr 14 mins        █░░░░░░░░░░░░░░░░░░░░░░░░   03.17 % 
sigma2-main              29 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   01.25 % 
accountbook              29 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   01.24 % 
numeron-v2               11 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   00.49 % 

💻 Operating System: 
Mac                      39 hrs 5 mins       █████████████████████████   100.00 % 
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


 Last Updated on 04/06/2024 18:38:11 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  <b>Visitor Count</b><br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg"  alt="count"/>
</p>
