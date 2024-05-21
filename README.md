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
![Code Time](http://img.shields.io/badge/Code%20Time-757%20hrs%2023%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-11-blue)

**🐱 My GitHub Data** 

> 📦 303.5 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 7 Private Repositories 
 > 
**I'm an Early 🐤** 

```text
🌞 Morning                204 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.26 % 
🌆 Daytime                469 commits         █████████░░░░░░░░░░░░░░░░   35.08 % 
🌃 Evening                359 commits         ███████░░░░░░░░░░░░░░░░░░   26.85 % 
🌙 Night                  305 commits         ██████░░░░░░░░░░░░░░░░░░░   22.81 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   202 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.11 % 
Tuesday                  311 commits         ██████░░░░░░░░░░░░░░░░░░░   23.26 % 
Wednesday                180 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.46 % 
Thursday                 228 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.05 % 
Friday                   99 commits          ██░░░░░░░░░░░░░░░░░░░░░░░   07.40 % 
Saturday                 173 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.94 % 
Sunday                   144 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.77 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Python                   17 mins             ██████████████░░░░░░░░░░░   57.44 % 
Rust                     12 mins             ██████████░░░░░░░░░░░░░░░   41.97 % 
Java                     0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.18 % 
GitIgnore file           0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.14 % 
XML                      0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.14 % 

🔥 Editors: 
PyCharm                  17 mins             ██████████████░░░░░░░░░░░   57.55 % 
Rustrover                12 mins             ██████████░░░░░░░░░░░░░░░   41.98 % 
Intellijidea             0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.46 % 

🐱‍💻 Projects: 
pythonProject            13 mins             ███████████░░░░░░░░░░░░░░   45.48 % 
penetration              12 mins             ██████████░░░░░░░░░░░░░░░   41.98 % 
ml                       3 mins              ███░░░░░░░░░░░░░░░░░░░░░░   11.01 % 
Unknown Project          0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   01.35 % 
untitled                 0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.18 % 

💻 Operating System: 
Mac                      29 mins             █████████████████████████   100.00 % 
```

**I Mostly Code in Java** 

```text
Java                     11 repos            ████████████░░░░░░░░░░░░░   50.00 % 
Rust                     2 repos             ██░░░░░░░░░░░░░░░░░░░░░░░   09.09 % 
C#                       1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.55 % 
JavaScript               1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.55 % 
HTML                     1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.55 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 21/05/2024 18:37:39 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  <b>Visitor Count</b><br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg"  alt="count"/>
</p>
