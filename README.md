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
![Code Time](http://img.shields.io/badge/Code%20Time-820%20hrs%2017%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-17-blue)

**🐱 My GitHub Data** 

> 📦 200.4 kB Used in GitHub's Storage 
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
🌞 Morning                213 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.88 % 
🌆 Daytime                365 commits         ███████░░░░░░░░░░░░░░░░░░   28.92 % 
🌃 Evening                372 commits         ███████░░░░░░░░░░░░░░░░░░   29.48 % 
🌙 Night                  312 commits         ██████░░░░░░░░░░░░░░░░░░░   24.72 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   190 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.06 % 
Tuesday                  298 commits         ██████░░░░░░░░░░░░░░░░░░░   23.61 % 
Wednesday                167 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.23 % 
Thursday                 215 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.04 % 
Friday                   89 commits          ██░░░░░░░░░░░░░░░░░░░░░░░   07.05 % 
Saturday                 169 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.39 % 
Sunday                   134 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.62 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Rust                     2 hrs 34 mins       ████████████████░░░░░░░░░   62.58 % 
C++                      39 mins             ████░░░░░░░░░░░░░░░░░░░░░   15.89 % 
PHP                      17 mins             ██░░░░░░░░░░░░░░░░░░░░░░░   07.25 % 
C                        11 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   04.56 % 
CMake                    7 mins              █░░░░░░░░░░░░░░░░░░░░░░░░   02.88 % 

🔥 Editors: 
RustRover                2 hrs 35 mins       ████████████████░░░░░░░░░   63.08 % 
CLion                    59 mins             ██████░░░░░░░░░░░░░░░░░░░   24.00 % 
IntelliJ IDEA            24 mins             ██░░░░░░░░░░░░░░░░░░░░░░░   09.74 % 
PyCharm                  6 mins              █░░░░░░░░░░░░░░░░░░░░░░░░   02.78 % 
Intellijidea             0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.40 % 

🐱‍💻 Projects: 
untitled                 1 hr 23 mins        ████████░░░░░░░░░░░░░░░░░   33.97 % 
learn-main               1 hr 12 mins        ███████░░░░░░░░░░░░░░░░░░   29.33 % 
untitled12               26 mins             ███░░░░░░░░░░░░░░░░░░░░░░   10.84 % 
cms                      16 mins             ██░░░░░░░░░░░░░░░░░░░░░░░   06.78 % 
ygopro-core-master       8 mins              █░░░░░░░░░░░░░░░░░░░░░░░░   03.55 % 

💻 Operating System: 
Mac                      4 hrs 6 mins        █████████████████████████   100.00 % 
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


 Last Updated on 23/06/2024 18:36:41 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  <b>Visitor Count</b><br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg"  alt="count"/>
</p>
