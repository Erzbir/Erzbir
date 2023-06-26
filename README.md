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

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct AboutMe {
    char *name;
    char **languages;
    char **likes;
    char **learnings;
    char *blog;
    char *email;
} AboutMe;

AboutMe *User = NULL;
char *name = "erzbir";
char *blog = "erzbir.com";
char *email = "erzbir@mail.com";
char *languages[] = {"Java", "Python", "JavaScript", "Html", "C#", "C", NULL};
char *likes[] = {"Irish flute", "Tin whistle", "Yu-Gi-Oh OCG", "Riding", NULL};
char *learnings[] = {"JVM", "Spring Cloud", "Kotlin", "Vue", NULL};

void init();
void prints(char **s);
void learn(char *s);
void clean();
int len(char **s);

int main() {
    init();
    printf("About Me:\n");
    printf("  name: \n    %s\n", User->name);
    printf("  languages:\n    ");
    prints(User->languages);
    printf("  likes:\n    ");
    prints(User->likes);
    learn("Assembly");
    learn("everything");
    learn("......");
    printf("  learnings:\n    ");
    prints(User->learnings);
    printf("  blog:\n    %s\n", User->blog);
    printf("  email:\n    %s", User->email);
    clean();
    return 0;
}

void init() {
    User = malloc(sizeof(AboutMe));
    User->name = name;
    User->blog = blog;
    User->email = email;
    User->languages = malloc(sizeof(languages));
    User->likes = malloc(sizeof(likes));
    User->learnings = malloc(sizeof(learnings));
    memcpy(User->languages, languages, sizeof(languages));
    memcpy(User->likes, likes, sizeof(likes));
    memcpy(User->learnings, learnings, sizeof(learnings));
}

void learn(char *s) {
    if (s == NULL) return;
    char **ptr = User->learnings;
    int length = len(User->learnings);
    User->learnings = realloc(ptr, sizeof(char *) * (length + 1));
    ptr = User->learnings;
    ptr[length] = s;
}

int len(char **s) {
    if (s == NULL) return 0;
    int length = 0;
    char **ptr = s;
    while (*ptr++ != NULL) length++;
    return length;
}

void prints(char **s) {
    if (s == NULL) return;
    char **ptr = s;
    printf("[");
    while (1) {
        printf("'%s'", *ptr);
        ptr++;
        if (*ptr != NULL) printf(", ");
        else break;
    }
    printf("]\n");
}

void clean() {
    free(User->languages);
    free(User->likes);
    free(User->learnings);
    free(User);
}
```

<!-- </details> -->

<p>
<a href="https://git.io/typing-svg">
    <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&size=30&duration=600&pause=500&multiline=true&repeat=true&width=1500&height=500&lines=About+Me%3A;ㅤ++name%3A;ㅤㅤ++++erzbir;ㅤ++++languages%3A;ㅤㅤ++++%5B'Java'%2C+'Python'%2C+'JavaScript'%2C+'Html'%2C+'C%23'%2C+'C'%5D;ㅤ++likes%3A;ㅤㅤ++++%5B'Irish+flute'%2C+'Tin+whistle'%2C+'Yu-Gi-Oh+OCG'%2C+'Riding'%5D;ㅤ++learnings%3A;ㅤㅤ++++%5B'JVM'%2C+'Spring+Cloud'%2C+'Kotlin'%2C+'Vue'%2C+'Assembly'%2C+'everything'%2C+'......'%5D;ㅤ++blog%3A;ㅤㅤ++++erzbir.com;ㅤ++email%3A;ㅤㅤ++++erzbir%40mail.com" alt="Typing SVG" />
</a>
</p>

##

<!-- 
<p>
<a href="https://github.com/Erzbir">
    <img src="https://github-stats-alpha.vercel.app/api?username=Erzbir&cc=22272e&tc=37BCF6&ic=fff&bc=0000">
</a>
<a href="https://github.com/Erzbir">
    <img src="https://github-readme-stats.vercel.app/api?username=Erzbir&layout=compact&show_icons=true&theme=tokyonight">
</a>
<a href="https://github.com/Erzbir">
    <img src="https://github-readme-stats.vercel.app/api/top-langs/?username=Erzbir&layout=compact&theme=tokyonight">
</a>
</p> 
-->

<!--
<details>
<summary>📈 Github Stats</summary>
</br>
-->

![](http://github-profile-summary-cards.vercel.app/api/cards/profile-details?username=Erzbir&theme=dracula) 

![](https://github-readme-stats.vercel.app/api?username=Erzbir&layout=compact&show_icons=true&theme=dracula)
![](https://github-readme-stats.vercel.app/api/top-langs/?username=Erzbir&layout=compact&theme=dracula)

<!--
</br>
</details>
-->

##

<details>
<summary>📈 Wakatime Stats</summary>
</br>

![Erzbir's wakatime stats](https://github-readme-stats.vercel.app/api/wakatime?username=Erzbir\&layout=compact)

##

<!--START_SECTION:waka-->
![Code Time](http://img.shields.io/badge/Code%20Time-123%20hrs%2017%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-42-blue)

**🐱 My GitHub Data** 

> 📦 117.3 kB Used in GitHub's Storage 
 > 
> 🏆 265 Contributions in the Year 2023
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 17 Public Repositories 
 > 
> 🔑 6 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                253 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.37 % 
🌆 Daytime                490 commits         ██████░░░░░░░░░░░░░░░░░░░   25.88 % 
🌃 Evening                606 commits         ████████░░░░░░░░░░░░░░░░░   32.01 % 
🌙 Night                  544 commits         ███████░░░░░░░░░░░░░░░░░░   28.74 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   235 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.41 % 
Tuesday                  436 commits         ██████░░░░░░░░░░░░░░░░░░░   23.03 % 
Wednesday                245 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.94 % 
Thursday                 296 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.64 % 
Friday                   140 commits         ██░░░░░░░░░░░░░░░░░░░░░░░   07.40 % 
Saturday                 337 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.80 % 
Sunday                   204 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.78 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     28 hrs 49 mins      ████████████████████░░░░░   80.97 % 
Kotlin                   4 hrs 11 mins       ███░░░░░░░░░░░░░░░░░░░░░░   11.79 % 
YAML                     43 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.04 % 
Properties               25 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   01.21 % 
Markdown                 18 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   00.85 % 

🔥 Editors: 
IntelliJ                 35 hrs 36 mins      █████████████████████████   100.00 % 

🐱‍💻 Projects: 
Numeron                  30 hrs 44 mins      ██████████████████████░░░   86.32 % 
untitled7                1 hr 55 mins        █░░░░░░░░░░░░░░░░░░░░░░░░   05.41 % 
mirai                    1 hr 8 mins         █░░░░░░░░░░░░░░░░░░░░░░░░   03.23 % 
untitled3                36 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   01.73 % 
untitled6                31 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   01.46 % 

💻 Operating System: 
Mac                      32 hrs 39 mins      ███████████████████████░░   91.70 % 
Windows                  2 hrs 57 mins       ██░░░░░░░░░░░░░░░░░░░░░░░   08.30 % 
```

**I Mostly Code in Java** 

```text
Java                     7 repos             ███████████░░░░░░░░░░░░░░   43.75 % 
HTML                     2 repos             ███░░░░░░░░░░░░░░░░░░░░░░   12.50 % 
Vue                      1 repo              ██░░░░░░░░░░░░░░░░░░░░░░░   06.25 % 
JavaScript               1 repo              ██░░░░░░░░░░░░░░░░░░░░░░░   06.25 % 
C                        1 repo              ██░░░░░░░░░░░░░░░░░░░░░░░   06.25 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 25/06/2023 18:36:52 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  Visitor count<br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg" />
</p>
