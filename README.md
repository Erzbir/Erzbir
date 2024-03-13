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
    let abilities = 'Expertises: {
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
    println!("{:#?}", abilities);
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
![Code Time](http://img.shields.io/badge/Code%20Time-718%20hrs%2035%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-2-blue)

**🐱 My GitHub Data** 

> 📦 303.4 kB Used in GitHub's Storage 
 > 
> 🏆 98 Contributions in the Year 2024
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 30 Public Repositories 
 > 
> 🔑 7 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                192 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.36 % 
🌆 Daytime                432 commits         █████████░░░░░░░░░░░░░░░░   34.56 % 
🌃 Evening                339 commits         ███████░░░░░░░░░░░░░░░░░░   27.12 % 
🌙 Night                  287 commits         ██████░░░░░░░░░░░░░░░░░░░   22.96 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   188 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.04 % 
Tuesday                  302 commits         ██████░░░░░░░░░░░░░░░░░░░   24.16 % 
Wednesday                168 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.44 % 
Thursday                 202 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.16 % 
Friday                   87 commits          ██░░░░░░░░░░░░░░░░░░░░░░░   06.96 % 
Saturday                 169 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.52 % 
Sunday                   134 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.72 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Rust                     11 hrs 35 mins      ████████████████████░░░░░   81.90 % 
CSS                      45 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   05.36 % 
Java                     32 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   03.82 % 
Python                   24 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.84 % 
TOML                     19 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.27 % 

🔥 Editors: 
Rustrover                12 hrs 25 mins      ██████████████████████░░░   87.85 % 
Intellijidea             1 hr 19 mins        ██░░░░░░░░░░░░░░░░░░░░░░░   09.31 % 
PyCharm                  24 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.84 % 

🐱‍💻 Projects: 
untitled                 11 hrs 32 mins      ████████████████████░░░░░   81.63 % 
halo-theme-higan         45 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   05.37 % 
ricq                     31 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   03.76 % 
ml                       24 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.84 % 
servo                    20 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.42 % 

💻 Operating System: 
Mac                      14 hrs 8 mins       █████████████████████████   100.00 % 
```

**I Mostly Code in Java** 

```text
Java                     13 repos            ██████████████░░░░░░░░░░░   54.17 % 
HTML                     2 repos             ██░░░░░░░░░░░░░░░░░░░░░░░   08.33 % 
C#                       1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.17 % 
JavaScript               1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.17 % 
C                        1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.17 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 13/03/2024 18:36:24 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  <b>Visitor Count</b><br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg"  alt="count"/>
</p>
