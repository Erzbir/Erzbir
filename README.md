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
    char **hobbies;
    char **learnings;
    char *blog;
    char *email;
} AboutMe;

AboutMe *me = NULL;
char *name = "erzbir";
char *blog = "erzbir.com";
char *email = "erzbir@mail.com";
char *languages[] = {"Java", "Python", "JavaScript", "Html", "C#", "C", NULL};
char *hobbies[] = {"Irish flute", "Tin whistle", "Yu-Gi-Oh OCG", "Riding", NULL};
char *learnings[] = {"JVM", "Spring Cloud", "Kotlin", "Vue", NULL};

void init();
void print_arr(char **s);
void learn(char *s);
void clean();
int len_arr(char **s);

int main() {
    init();
    printf("About Me:\n");
    printf("  name: \n    %s\n", me->name);
    printf("  languages:\n    ");
    print_arr(me->languages);
    printf("  hobbies:\n    ");
    print_arr(me->hobbies);
    learn("Assembly");
    learn("everything");
    learn("......");
    printf("  learnings:\n    ");
    print_arr(me->learnings);
    printf("  blog:\n    %s\n", me->blog);
    printf("  email:\n    %s", me->email);
    clean();
    return 0;
}

void init() {
    me = malloc(sizeof(AboutMe));
    me->name = name;
    me->blog = blog;
    me->email = email;
    me->languages = malloc(sizeof(languages));
    me->hobbies = malloc(sizeof(hobbies));
    me->learnings = malloc(sizeof(learnings));
    memcpy(me->languages, languages, sizeof(languages));
    memcpy(me->hobbies, hobbies, sizeof(hobbies));
    memcpy(me->learnings, learnings, sizeof(learnings));
}

void learn(char *s) {
    if (s == NULL) return;
    char **ptr = me->learnings;
    int length = len_arr(me->learnings);
    me->learnings = realloc(ptr, sizeof(char *) * (length + 1));
    ptr = me->learnings;
    ptr[length] = s;
}

int len_arr(char **s) {
    if (s == NULL) return 0;
    int length = 0;
    char **ptr = s;
    while (*ptr++ != NULL) length++;
    return length;
}

void print_arr(char **s) {
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
    free(me->languages);
    free(me->hobbies);
    free(me->learnings);
    free(me);
}
```

<!-- </details> -->

<p>
<a href="https://git.io/typing-svg">
    <img src="https://readme-typing-svg.demolab.com?font=Fira+Code&size=30&duration=600&pause=500&multiline=true&repeat=true&width=1500&height=500&lines=About+Me%3A;ㅤ++name%3A;ㅤㅤ++++erzbir;ㅤ++++languages%3A;ㅤㅤ++++%5B'Java'%2C+'Python'%2C+'JavaScript'%2C+'Html'%2C+'C%23'%2C+'C'%5D;ㅤ++hobbies%3A;ㅤㅤ++++%5B'Irish+flute'%2C+'Tin+whistle'%2C+'Yu-Gi-Oh+OCG'%2C+'Riding'%5D;ㅤ++learnings%3A;ㅤㅤ++++%5B'JVM'%2C+'Spring+Cloud'%2C+'Kotlin'%2C+'Vue'%2C+'Assembly'%2C+'everything'%2C+'......'%5D;ㅤ++blog%3A;ㅤㅤ++++erzbir.com;ㅤ++email%3A;ㅤㅤ++++erzbir%40mail.com" alt="Typing SVG" />
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
<br>

![Erzbir's wakatime stats](https://github-readme-stats.vercel.app/api/wakatime?username=Erzbir\&layout=compact)

##

<!--START_SECTION:waka-->
![Code Time](http://img.shields.io/badge/Code%20Time-363%20hrs%2015%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-6-blue)

**🐱 My GitHub Data** 

> 📦 198.1 kB Used in GitHub's Storage 
 > 
> 🏆 318 Contributions in the Year 2023
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 18 Public Repositories 
 > 
> 🔑 8 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                266 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.88 % 
🌆 Daytime                629 commits         ████████░░░░░░░░░░░░░░░░░   30.46 % 
🌃 Evening                626 commits         ████████░░░░░░░░░░░░░░░░░   30.31 % 
🌙 Night                  544 commits         ███████░░░░░░░░░░░░░░░░░░   26.34 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   281 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.61 % 
Tuesday                  471 commits         ██████░░░░░░░░░░░░░░░░░░░   22.81 % 
Wednesday                268 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.98 % 
Thursday                 311 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.06 % 
Friday                   158 commits         ██░░░░░░░░░░░░░░░░░░░░░░░   07.65 % 
Saturday                 355 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.19 % 
Sunday                   221 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.70 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     6 hrs 38 mins       ████████████████████████░   95.04 % 
TOML                     9 mins              █░░░░░░░░░░░░░░░░░░░░░░░░   02.21 % 
XML                      3 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.94 % 
Markdown                 2 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.67 % 
GitIgnore file           2 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.49 % 

🔥 Editors: 
IntelliJ                 6 hrs 47 mins       ████████████████████████░   97.26 % 
RustRover                11 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.65 % 
Neovim                   0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.09 % 

🐱‍💻 Projects: 
DesignPattern            5 hrs 14 mins       ███████████████████░░░░░░   74.97 % 
java-design-patterns     42 mins             ███░░░░░░░░░░░░░░░░░░░░░░   10.13 % 
Unknown Project          38 mins             ██░░░░░░░░░░░░░░░░░░░░░░░   09.27 % 
untitled                 11 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.62 % 
DesignPattern-master     8 mins              █░░░░░░░░░░░░░░░░░░░░░░░░   02.10 % 

💻 Operating System: 
Mac                      6 hrs 48 mins       ████████████████████████░   97.35 % 
Windows                  11 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.65 % 
```

**I Mostly Code in Java** 

```text
Java                     7 repos             █████████░░░░░░░░░░░░░░░░   36.84 % 
Vue                      2 repos             ███░░░░░░░░░░░░░░░░░░░░░░   10.53 % 
HTML                     2 repos             ███░░░░░░░░░░░░░░░░░░░░░░   10.53 % 
C                        2 repos             ███░░░░░░░░░░░░░░░░░░░░░░   10.53 % 
JavaScript               1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   05.26 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 09/10/2023 18:39:04 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  Visitor count<br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg" />
</p>
