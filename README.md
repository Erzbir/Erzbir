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
![Code Time](http://img.shields.io/badge/Code%20Time-737%20hrs%2028%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-1-blue)

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
🌞 Morning                195 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.58 % 
🌆 Daytime                434 commits         █████████░░░░░░░░░░░░░░░░   34.66 % 
🌃 Evening                339 commits         ███████░░░░░░░░░░░░░░░░░░   27.08 % 
🌙 Night                  284 commits         ██████░░░░░░░░░░░░░░░░░░░   22.68 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   190 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.18 % 
Tuesday                  302 commits         ██████░░░░░░░░░░░░░░░░░░░   24.12 % 
Wednesday                168 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.42 % 
Thursday                 202 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.13 % 
Friday                   88 commits          ██░░░░░░░░░░░░░░░░░░░░░░░   07.03 % 
Saturday                 171 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.66 % 
Sunday                   131 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.46 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Python                   1 hr 14 mins        ████████████████████████░   96.52 % 
ObjectiveC               1 min               █░░░░░░░░░░░░░░░░░░░░░░░░   02.50 % 
C                        0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.69 % 
Markdown                 0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.26 % 
Text                     0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.02 % 

🔥 Editors: 
PyCharm                  1 hr 14 mins        ████████████████████████░   96.54 % 
CLion                    2 mins              █░░░░░░░░░░░░░░░░░░░░░░░░   03.19 % 
Intellijidea             0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.27 % 

🐱‍💻 Projects: 
ml                       1 hr 14 mins        ████████████████████████░   96.54 % 
untitled10               2 mins              █░░░░░░░░░░░░░░░░░░░░░░░░   03.19 % 
etcher-master            0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.26 % 
numeron-v2               0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.01 % 

💻 Operating System: 
Mac                      1 hr 17 mins        █████████████████████████   100.00 % 
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


 Last Updated on 06/04/2024 18:35:52 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  <b>Visitor Count</b><br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg"  alt="count"/>
</p>
