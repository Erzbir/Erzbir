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
![Code Time](http://img.shields.io/badge/Code%20Time-273%20hrs-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-9-blue)

**🐱 My GitHub Data** 

> 📦 159.5 kB Used in GitHub's Storage 
 > 
> 🏆 312 Contributions in the Year 2023
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 18 Public Repositories 
 > 
> 🔑 6 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                261 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.14 % 
🌆 Daytime                563 commits         ███████░░░░░░░░░░░░░░░░░░   28.33 % 
🌃 Evening                621 commits         ████████░░░░░░░░░░░░░░░░░   31.25 % 
🌙 Night                  542 commits         ███████░░░░░░░░░░░░░░░░░░   27.28 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   266 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.39 % 
Tuesday                  462 commits         ██████░░░░░░░░░░░░░░░░░░░   23.25 % 
Wednesday                259 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.03 % 
Thursday                 302 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.20 % 
Friday                   146 commits         ██░░░░░░░░░░░░░░░░░░░░░░░   07.35 % 
Saturday                 345 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.36 % 
Sunday                   207 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.42 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     9 hrs 58 mins       ███████░░░░░░░░░░░░░░░░░░   27.29 % 
TypeScript               9 hrs 40 mins       ███████░░░░░░░░░░░░░░░░░░   26.46 % 
Docker                   6 hrs 57 mins       █████░░░░░░░░░░░░░░░░░░░░   19.02 % 
YAML                     3 hrs 8 mins        ██░░░░░░░░░░░░░░░░░░░░░░░   08.58 % 
XML                      2 hrs 21 mins       ██░░░░░░░░░░░░░░░░░░░░░░░   06.45 % 

🔥 Editors: 
IntelliJ                 26 hrs 57 mins      ██████████████████░░░░░░░   73.78 % 
Neovim                   9 hrs 33 mins       ███████░░░░░░░░░░░░░░░░░░   26.18 % 
CLion                    0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.04 % 

🐱‍💻 Projects: 
halo                     11 hrs 45 mins      ████████░░░░░░░░░░░░░░░░░   32.17 % 
Unknown Project          9 hrs 51 mins       ███████░░░░░░░░░░░░░░░░░░   26.97 % 
mircroservice            6 hrs 8 mins        ████░░░░░░░░░░░░░░░░░░░░░   16.82 % 
hotel                    2 hrs 41 mins       ██░░░░░░░░░░░░░░░░░░░░░░░   07.38 % 
springboot               2 hrs 17 mins       ██░░░░░░░░░░░░░░░░░░░░░░░   06.28 % 

💻 Operating System: 
Mac                      36 hrs 32 mins      █████████████████████████   100.00 % 
Windows                  0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.00 % 
```

**I Mostly Code in Java** 

```text
Java                     7 repos             ██████████░░░░░░░░░░░░░░░   38.89 % 
HTML                     2 repos             ███░░░░░░░░░░░░░░░░░░░░░░   11.11 % 
C                        2 repos             ███░░░░░░░░░░░░░░░░░░░░░░   11.11 % 
Vue                      1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   05.56 % 
JavaScript               1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   05.56 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 05/08/2023 18:37:10 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  Visitor count<br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg" />
</p>
