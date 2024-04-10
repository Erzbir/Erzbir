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
![Code Time](http://img.shields.io/badge/Code%20Time-738%20hrs%2027%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-4-blue)

**🐱 My GitHub Data** 

> 📦 303.4 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 30 Public Repositories 
 > 
> 🔑 7 Private Repositories 
 > 
**I'm an Early 🐤** 

```text
🌞 Morning                203 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.22 % 
🌆 Daytime                466 commits         █████████░░░░░░░░░░░░░░░░   34.93 % 
🌃 Evening                360 commits         ███████░░░░░░░░░░░░░░░░░░   26.99 % 
🌙 Night                  305 commits         ██████░░░░░░░░░░░░░░░░░░░   22.86 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   202 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.14 % 
Tuesday                  309 commits         ██████░░░░░░░░░░░░░░░░░░░   23.16 % 
Wednesday                180 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.49 % 
Thursday                 228 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.09 % 
Friday                   97 commits          ██░░░░░░░░░░░░░░░░░░░░░░░   07.27 % 
Saturday                 173 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.97 % 
Sunday                   145 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.87 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Python                   1 hr 2 mins         █████████████░░░░░░░░░░░░   51.54 % 
Java                     56 mins             ████████████░░░░░░░░░░░░░   47.03 % 
protobuf                 1 min               ░░░░░░░░░░░░░░░░░░░░░░░░░   01.08 % 
Kotlin                   0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.20 % 
Protocol Buffer          0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.14 % 

🔥 Editors: 
PyCharm                  1 hr 2 mins         █████████████░░░░░░░░░░░░   51.56 % 
Intellijidea             58 mins             ████████████░░░░░░░░░░░░░   48.44 % 

🐱‍💻 Projects: 
ml                       1 hr 2 mins         █████████████░░░░░░░░░░░░   51.56 % 
numeron-v2               58 mins             ████████████░░░░░░░░░░░░░   48.44 % 

💻 Operating System: 
Mac                      2 hrs               █████████████████████████   100.00 % 
```

**I Mostly Code in Java** 

```text
Java                     12 repos            █████████████░░░░░░░░░░░░   52.17 % 
HTML                     2 repos             ██░░░░░░░░░░░░░░░░░░░░░░░   08.70 % 
C#                       1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.35 % 
JavaScript               1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.35 % 
C                        1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.35 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 10/04/2024 18:59:11 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  <b>Visitor Count</b><br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg"  alt="count"/>
</p>
