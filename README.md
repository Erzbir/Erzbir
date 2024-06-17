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
![Code Time](http://img.shields.io/badge/Code%20Time-816%20hrs%2010%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-18-blue)

**🐱 My GitHub Data** 

> 📦 183.9 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 34 Public Repositories 
 > 
> 🔑 7 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                213 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.96 % 
🌆 Daytime                359 commits         ███████░░░░░░░░░░░░░░░░░░   28.58 % 
🌃 Evening                372 commits         ███████░░░░░░░░░░░░░░░░░░   29.62 % 
🌙 Night                  312 commits         ██████░░░░░░░░░░░░░░░░░░░   24.84 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   190 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.13 % 
Tuesday                  297 commits         ██████░░░░░░░░░░░░░░░░░░░   23.65 % 
Wednesday                166 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.22 % 
Thursday                 214 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.04 % 
Friday                   88 commits          ██░░░░░░░░░░░░░░░░░░░░░░░   07.01 % 
Saturday                 168 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.38 % 
Sunday                   133 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.59 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     4 hrs 9 mins        ███████████████░░░░░░░░░░   58.95 % 
Python                   1 hr 19 mins        █████░░░░░░░░░░░░░░░░░░░░   18.74 % 
Rust                     53 mins             ███░░░░░░░░░░░░░░░░░░░░░░   12.63 % 
Gradle                   14 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   03.35 % 
HTTP Request             7 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   01.78 % 

🔥 Editors: 
Intellijidea             4 hrs 45 mins       █████████████████░░░░░░░░   67.51 % 
PyCharm                  1 hr 22 mins        █████░░░░░░░░░░░░░░░░░░░░   19.43 % 
RustRover                55 mins             ███░░░░░░░░░░░░░░░░░░░░░░   13.07 % 

🐱‍💻 Projects: 
student-sys              1 hr 44 mins        ██████░░░░░░░░░░░░░░░░░░░   24.71 % 
ml                       1 hr 19 mins        █████░░░░░░░░░░░░░░░░░░░░   18.75 % 
JavaThings               1 hr 11 mins        ████░░░░░░░░░░░░░░░░░░░░░   16.91 % 
untitled11               48 mins             ███░░░░░░░░░░░░░░░░░░░░░░   11.52 % 
untitled                 43 mins             ███░░░░░░░░░░░░░░░░░░░░░░   10.28 % 

💻 Operating System: 
Mac                      7 hrs 2 mins        █████████████████████████   100.00 % 
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


 Last Updated on 17/06/2024 18:38:11 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  <b>Visitor Count</b><br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg"  alt="count"/>
</p>
