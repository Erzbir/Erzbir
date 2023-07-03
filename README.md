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
![Code Time](http://img.shields.io/badge/Code%20Time-150%20hrs%2054%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-159-blue)

**🐱 My GitHub Data** 

> 📦 122.3 kB Used in GitHub's Storage 
 > 
> 🏆 297 Contributions in the Year 2023
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 18 Public Repositories 
 > 
> 🔑 6 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                255 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.19 % 
🌆 Daytime                520 commits         ███████░░░░░░░░░░░░░░░░░░   26.89 % 
🌃 Evening                616 commits         ████████░░░░░░░░░░░░░░░░░   31.85 % 
🌙 Night                  543 commits         ███████░░░░░░░░░░░░░░░░░░   28.08 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   250 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.93 % 
Tuesday                  457 commits         ██████░░░░░░░░░░░░░░░░░░░   23.63 % 
Wednesday                249 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.87 % 
Thursday                 297 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.36 % 
Friday                   141 commits         ██░░░░░░░░░░░░░░░░░░░░░░░   07.29 % 
Saturday                 338 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.48 % 
Sunday                   202 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.44 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     13 hrs 45 mins      ███████████████░░░░░░░░░░   59.59 % 
sh                       3 hrs 26 mins       ████░░░░░░░░░░░░░░░░░░░░░   14.90 % 
Kotlin                   2 hrs 3 mins        ██░░░░░░░░░░░░░░░░░░░░░░░   08.93 % 
ObjectiveC               53 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   03.90 % 
C                        32 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.36 % 

🔥 Editors: 
IntelliJ                 17 hrs 22 mins      ███████████████████░░░░░░   75.21 % 
Zsh                      3 hrs 26 mins       ████░░░░░░░░░░░░░░░░░░░░░   14.90 % 
CLion                    1 hr 43 mins        ██░░░░░░░░░░░░░░░░░░░░░░░   07.44 % 
Neovim                   33 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.45 % 

🐱‍💻 Projects: 
Numeron                  16 hrs 30 mins      ██████████████████░░░░░░░   71.47 % 
Terminal                 2 hrs 54 mins       ███░░░░░░░░░░░░░░░░░░░░░░   12.59 % 
untitled4                1 hr 25 mins        ██░░░░░░░░░░░░░░░░░░░░░░░   06.15 % 
nvim                     37 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.72 % 
Unknown Project          28 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.07 % 

💻 Operating System: 
Mac                      20 hrs 19 mins      ██████████████████████░░░   88.00 % 
Windows                  2 hrs 46 mins       ███░░░░░░░░░░░░░░░░░░░░░░   12.00 % 
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


 Last Updated on 03/07/2023 18:38:45 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  Visitor count<br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg" />
</p>
