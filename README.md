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
![Code Time](http://img.shields.io/badge/Code%20Time-734%20hrs%203%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-1-blue)

**🐱 My GitHub Data** 

> 📦 303.4 kB Used in GitHub's Storage 
 > 
> 🏆 99 Contributions in the Year 2024
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 30 Public Repositories 
 > 
> 🔑 7 Private Repositories 
 > 
**I'm an Early 🐤** 

```text
🌞 Morning                194 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.48 % 
🌆 Daytime                433 commits         █████████░░░░░░░░░░░░░░░░   34.56 % 
🌃 Evening                339 commits         ███████░░░░░░░░░░░░░░░░░░   27.06 % 
🌙 Night                  287 commits         ██████░░░░░░░░░░░░░░░░░░░   22.91 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   189 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.08 % 
Tuesday                  302 commits         ██████░░░░░░░░░░░░░░░░░░░   24.10 % 
Wednesday                168 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.41 % 
Thursday                 202 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.12 % 
Friday                   88 commits          ██░░░░░░░░░░░░░░░░░░░░░░░   07.02 % 
Saturday                 170 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.57 % 
Sunday                   134 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.69 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Rust                     2 hrs 9 mins        █████████████░░░░░░░░░░░░   51.06 % 
Python                   1 hr 19 mins        ████████░░░░░░░░░░░░░░░░░   31.35 % 
Java                     24 mins             ██░░░░░░░░░░░░░░░░░░░░░░░   09.78 % 
PythonStub               9 mins              █░░░░░░░░░░░░░░░░░░░░░░░░   03.67 % 
Protocol Buffer          6 mins              █░░░░░░░░░░░░░░░░░░░░░░░░   02.48 % 

🔥 Editors: 
Rustrover                2 hrs 9 mins        █████████████░░░░░░░░░░░░   51.09 % 
PyCharm                  1 hr 28 mins        █████████░░░░░░░░░░░░░░░░   35.02 % 
Intellijidea             34 mins             ███░░░░░░░░░░░░░░░░░░░░░░   13.63 % 
CLion                    0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.26 % 

🐱‍💻 Projects: 
untitled1                1 hr 59 mins        ████████████░░░░░░░░░░░░░   47.29 % 
ml                       1 hr 28 mins        █████████░░░░░░░░░░░░░░░░   35.02 % 
numeron-v2               32 mins             ███░░░░░░░░░░░░░░░░░░░░░░   12.91 % 
ricq                     9 mins              █░░░░░░░░░░░░░░░░░░░░░░░░   03.80 % 
Numeron                  1 min               ░░░░░░░░░░░░░░░░░░░░░░░░░   00.72 % 

💻 Operating System: 
Mac                      4 hrs 13 mins       █████████████████████████   100.00 % 
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


 Last Updated on 27/03/2024 18:36:29 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  <b>Visitor Count</b><br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg"  alt="count"/>
</p>
