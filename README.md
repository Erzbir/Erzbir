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
![Code Time](http://img.shields.io/badge/Code%20Time-816%20hrs%2044%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-18-blue)

**🐱 My GitHub Data** 

> 📦 193.4 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 34 Public Repositories 
 > 
> 🔑 7 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                213 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.93 % 
🌆 Daytime                361 commits         ███████░░░░░░░░░░░░░░░░░░   28.70 % 
🌃 Evening                372 commits         ███████░░░░░░░░░░░░░░░░░░   29.57 % 
🌙 Night                  312 commits         ██████░░░░░░░░░░░░░░░░░░░   24.80 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   190 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.10 % 
Tuesday                  298 commits         ██████░░░░░░░░░░░░░░░░░░░   23.69 % 
Wednesday                167 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.28 % 
Thursday                 214 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.01 % 
Friday                   88 commits          ██░░░░░░░░░░░░░░░░░░░░░░░   07.00 % 
Saturday                 168 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.35 % 
Sunday                   133 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.57 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     1 hr 54 mins        ███████████░░░░░░░░░░░░░░   42.30 % 
Python                   1 hr 10 mins        ███████░░░░░░░░░░░░░░░░░░   26.19 % 
Rust                     1 hr 1 min          ██████░░░░░░░░░░░░░░░░░░░   22.62 % 
Gradle                   14 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   05.26 % 
Jupyter                  2 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   01.04 % 

🔥 Editors: 
IntelliJ IDEA            2 hrs 14 mins       ████████████░░░░░░░░░░░░░   49.74 % 
PyCharm                  1 hr 13 mins        ███████░░░░░░░░░░░░░░░░░░   27.27 % 
RustRover                1 hr 1 min          ██████░░░░░░░░░░░░░░░░░░░   22.63 % 
Intellijidea             0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.36 % 

🐱‍💻 Projects: 
untitled                 1 hr 17 mins        ███████░░░░░░░░░░░░░░░░░░   28.61 % 
JavaThings               1 hr 11 mins        ███████░░░░░░░░░░░░░░░░░░   26.51 % 
ml                       1 hr 10 mins        ███████░░░░░░░░░░░░░░░░░░   26.22 % 
spring-framework         15 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   05.90 % 
untitled11               14 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   05.20 % 

💻 Operating System: 
Mac                      4 hrs 29 mins       █████████████████████████   100.00 % 
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


 Last Updated on 19/06/2024 18:39:07 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  <b>Visitor Count</b><br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg"  alt="count"/>
</p>
