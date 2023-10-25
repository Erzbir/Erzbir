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
![Code Time](http://img.shields.io/badge/Code%20Time-412%20hrs%2037%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-0-blue)

**🐱 My GitHub Data** 

> 📦 294.0 kB Used in GitHub's Storage 
 > 
> 🏆 342 Contributions in the Year 2023
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 19 Public Repositories 
 > 
> 🔑 8 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                270 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.88 % 
🌆 Daytime                651 commits         ████████░░░░░░░░░░░░░░░░░   31.04 % 
🌃 Evening                628 commits         ███████░░░░░░░░░░░░░░░░░░   29.95 % 
🌙 Night                  548 commits         ███████░░░░░░░░░░░░░░░░░░   26.13 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   289 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.78 % 
Tuesday                  474 commits         ██████░░░░░░░░░░░░░░░░░░░   22.60 % 
Wednesday                271 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.92 % 
Thursday                 320 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.26 % 
Friday                   160 commits         ██░░░░░░░░░░░░░░░░░░░░░░░   07.63 % 
Saturday                 360 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.17 % 
Sunday                   223 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.63 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     28 hrs 22 mins      █████████████████████░░░░   84.85 % 
YAML                     3 hrs               ██░░░░░░░░░░░░░░░░░░░░░░░   08.99 % 
Rust                     21 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   01.07 % 
JavaScript               19 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   00.95 % 
JSON                     17 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   00.88 % 

🔥 Editors: 
IntelliJ                 32 hrs 54 mins      █████████████████████████   98.41 % 
CLion                    31 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   01.57 % 
RustRover                0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.02 % 

🐱‍💻 Projects: 
halo-plugin-sitepush     18 hrs 30 mins      ██████████████░░░░░░░░░░░   55.31 % 
untitled9                9 hrs               ███████░░░░░░░░░░░░░░░░░░   26.93 % 
DesignPattern            2 hrs 49 mins       ██░░░░░░░░░░░░░░░░░░░░░░░   08.45 % 
halo                     59 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.95 % 
demo                     37 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   01.87 % 

💻 Operating System: 
Mac                      33 hrs 26 mins      █████████████████████████   99.98 % 
Windows                  0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.02 % 
```

**I Mostly Code in Java** 

```text
Java                     8 repos             ██████████░░░░░░░░░░░░░░░   40.00 % 
Vue                      2 repos             ██░░░░░░░░░░░░░░░░░░░░░░░   10.00 % 
HTML                     2 repos             ██░░░░░░░░░░░░░░░░░░░░░░░   10.00 % 
C                        2 repos             ██░░░░░░░░░░░░░░░░░░░░░░░   10.00 % 
JavaScript               1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   05.00 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 24/10/2023 18:38:35 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  <b>Visitor Count</b><br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg" />
</p>
