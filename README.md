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
![Code Time](http://img.shields.io/badge/Code%20Time-816%20hrs%2043%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-18-blue)

**🐱 My GitHub Data** 

> 📦 190.8 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 34 Public Repositories 
 > 
> 🔑 7 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                213 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.95 % 
🌆 Daytime                360 commits         ███████░░░░░░░░░░░░░░░░░░   28.64 % 
🌃 Evening                372 commits         ███████░░░░░░░░░░░░░░░░░░   29.59 % 
🌙 Night                  312 commits         ██████░░░░░░░░░░░░░░░░░░░   24.82 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   190 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.12 % 
Tuesday                  298 commits         ██████░░░░░░░░░░░░░░░░░░░   23.71 % 
Wednesday                166 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.21 % 
Thursday                 214 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.02 % 
Friday                   88 commits          ██░░░░░░░░░░░░░░░░░░░░░░░   07.00 % 
Saturday                 168 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.37 % 
Sunday                   133 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.58 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     3 hrs 11 mins       █████████████░░░░░░░░░░░░   51.86 % 
Python                   1 hr 19 mins        █████░░░░░░░░░░░░░░░░░░░░   21.47 % 
Rust                     1 hr 6 mins         █████░░░░░░░░░░░░░░░░░░░░   18.05 % 
Gradle                   14 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   03.84 % 
Jupyter                  2 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.76 % 

🔥 Editors: 
Intellijidea             3 hrs 39 mins       ███████████████░░░░░░░░░░   59.52 % 
PyCharm                  1 hr 22 mins        ██████░░░░░░░░░░░░░░░░░░░   22.26 % 
RustRover                1 hr 7 mins         █████░░░░░░░░░░░░░░░░░░░░   18.22 % 

🐱‍💻 Projects: 
ml                       1 hr 19 mins        █████░░░░░░░░░░░░░░░░░░░░   21.48 % 
untitled                 1 hr 15 mins        █████░░░░░░░░░░░░░░░░░░░░   20.56 % 
JavaThings               1 hr 11 mins        █████░░░░░░░░░░░░░░░░░░░░   19.38 % 
untitled11               48 mins             ███░░░░░░░░░░░░░░░░░░░░░░   13.20 % 
student-sys              37 mins             ███░░░░░░░░░░░░░░░░░░░░░░   10.28 % 

💻 Operating System: 
Mac                      6 hrs 9 mins        █████████████████████████   100.00 % 
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


 Last Updated on 18/06/2024 18:39:09 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  <b>Visitor Count</b><br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg"  alt="count"/>
</p>
