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
![Code Time](http://img.shields.io/badge/Code%20Time-829%20hrs%2015%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-17-blue)

**🐱 My GitHub Data** 

> 📦 206.5 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 34 Public Repositories 
 > 
> 🔑 7 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                212 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.75 % 
🌆 Daytime                370 commits         ███████░░░░░░░░░░░░░░░░░░   29.23 % 
🌃 Evening                372 commits         ███████░░░░░░░░░░░░░░░░░░   29.38 % 
🌙 Night                  312 commits         ██████░░░░░░░░░░░░░░░░░░░   24.64 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   190 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.01 % 
Tuesday                  299 commits         ██████░░░░░░░░░░░░░░░░░░░   23.62 % 
Wednesday                168 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.27 % 
Thursday                 216 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.06 % 
Friday                   90 commits          ██░░░░░░░░░░░░░░░░░░░░░░░   07.11 % 
Saturday                 169 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.35 % 
Sunday                   134 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.58 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Rust                     4 hrs 24 mins       ████████████░░░░░░░░░░░░░   49.03 % 
Python                   1 hr 32 mins        ████░░░░░░░░░░░░░░░░░░░░░   17.10 % 
C++                      1 hr 25 mins        ████░░░░░░░░░░░░░░░░░░░░░   15.95 % 
Java                     23 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   04.31 % 
PHP                      17 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   03.33 % 

🔥 Editors: 
Neovim                   5 hrs 4 mins        ██████████████░░░░░░░░░░░   56.61 % 
CLion                    1 hr 42 mins        █████░░░░░░░░░░░░░░░░░░░░   19.04 % 
PyCharm                  1 hr 32 mins        ████░░░░░░░░░░░░░░░░░░░░░   17.21 % 
IntelliJ IDEA            24 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   04.50 % 
RustRover                13 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.46 % 

🐱‍💻 Projects: 
learn-main               4 hrs 2 mins        ███████████░░░░░░░░░░░░░░   45.03 % 
untitled12               1 hr 41 mins        █████░░░░░░░░░░░░░░░░░░░░   18.87 % 
pythonProject            1 hr 15 mins        ███░░░░░░░░░░░░░░░░░░░░░░   13.94 % 
Unknown Project          38 mins             ██░░░░░░░░░░░░░░░░░░░░░░░   07.19 % 
homebrew                 24 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   04.48 % 

💻 Operating System: 
Mac                      8 hrs 58 mins       █████████████████████████   100.00 % 
```

**I Mostly Code in Java** 

```text
Java                     12 repos            ██████████████░░░░░░░░░░░   57.14 % 
HTML                     2 repos             ██░░░░░░░░░░░░░░░░░░░░░░░   09.52 % 
Rust                     2 repos             ██░░░░░░░░░░░░░░░░░░░░░░░   09.52 % 
C#                       1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.76 % 
C                        1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.76 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 29/06/2024 18:37:02 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  <b>Visitor Count</b><br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg"  alt="count"/>
</p>
