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
![Code Time](http://img.shields.io/badge/Code%20Time-821%20hrs%2028%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-17-blue)

**🐱 My GitHub Data** 

> 📦 207.6 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 34 Public Repositories 
 > 
> 🔑 7 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                213 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.85 % 
🌆 Daytime                367 commits         ███████░░░░░░░░░░░░░░░░░░   29.03 % 
🌃 Evening                372 commits         ███████░░░░░░░░░░░░░░░░░░   29.43 % 
🌙 Night                  312 commits         ██████░░░░░░░░░░░░░░░░░░░   24.68 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   191 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.11 % 
Tuesday                  299 commits         ██████░░░░░░░░░░░░░░░░░░░   23.66 % 
Wednesday                167 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.21 % 
Thursday                 215 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.01 % 
Friday                   89 commits          ██░░░░░░░░░░░░░░░░░░░░░░░   07.04 % 
Saturday                 169 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.37 % 
Sunday                   134 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.60 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Rust                     2 hrs 2 mins        ███████████░░░░░░░░░░░░░░   43.01 % 
Python                   1 hr 18 mins        ███████░░░░░░░░░░░░░░░░░░   27.43 % 
C++                      39 mins             ███░░░░░░░░░░░░░░░░░░░░░░   13.71 % 
PHP                      17 mins             ██░░░░░░░░░░░░░░░░░░░░░░░   06.26 % 
C                        11 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   03.94 % 

🔥 Editors: 
RustRover                2 hrs 3 mins        ███████████░░░░░░░░░░░░░░   43.44 % 
PyCharm                  1 hr 18 mins        ███████░░░░░░░░░░░░░░░░░░   27.43 % 
CLion                    59 mins             █████░░░░░░░░░░░░░░░░░░░░   20.72 % 
IntelliJ IDEA            23 mins             ██░░░░░░░░░░░░░░░░░░░░░░░   08.07 % 
Intellijidea             0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.34 % 

🐱‍💻 Projects: 
pythonProject            1 hr 17 mins        ███████░░░░░░░░░░░░░░░░░░   27.12 % 
learn-main               1 hr 12 mins        ██████░░░░░░░░░░░░░░░░░░░   25.31 % 
untitled                 51 mins             ████░░░░░░░░░░░░░░░░░░░░░   17.97 % 
untitled12               26 mins             ██░░░░░░░░░░░░░░░░░░░░░░░   09.36 % 
cms                      16 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   05.86 % 

💻 Operating System: 
Mac                      4 hrs 45 mins       █████████████████████████   100.00 % 
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


 Last Updated on 25/06/2024 18:39:41 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  <b>Visitor Count</b><br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg"  alt="count"/>
</p>
