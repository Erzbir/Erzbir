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
![Code Time](http://img.shields.io/badge/Code%20Time-387%20hrs%2054%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-6-blue)

**🐱 My GitHub Data** 

> 📦 287.6 kB Used in GitHub's Storage 
 > 
> 🏆 329 Contributions in the Year 2023
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 19 Public Repositories 
 > 
> 🔑 8 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                270 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.95 % 
🌆 Daytime                641 commits         ████████░░░░░░░░░░░░░░░░░   30.74 % 
🌃 Evening                626 commits         ████████░░░░░░░░░░░░░░░░░   30.02 % 
🌙 Night                  548 commits         ███████░░░░░░░░░░░░░░░░░░   26.28 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   283 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.57 % 
Tuesday                  473 commits         ██████░░░░░░░░░░░░░░░░░░░   22.69 % 
Wednesday                271 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.00 % 
Thursday                 320 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.35 % 
Friday                   159 commits         ██░░░░░░░░░░░░░░░░░░░░░░░   07.63 % 
Saturday                 357 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.12 % 
Sunday                   222 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.65 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     10 hrs 39 mins      █████████████████████░░░░   83.40 % 
YAML                     1 hr 33 mins        ███░░░░░░░░░░░░░░░░░░░░░░   12.19 % 
JavaScript               13 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   01.70 % 
Gradle                   7 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.97 % 
Markdown                 4 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.64 % 

🔥 Editors: 
IntelliJ                 12 hrs 46 mins      █████████████████████████   100.00 % 

🐱‍💻 Projects: 
halo-plugin-sitepush     9 hrs 9 mins        ██████████████████░░░░░░░   71.65 % 
DesignPattern            1 hr 35 mins        ███░░░░░░░░░░░░░░░░░░░░░░   12.50 % 
halo                     1 hr 2 mins         ██░░░░░░░░░░░░░░░░░░░░░░░   08.18 % 
untitled9                26 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   03.44 % 
Indexing-master          18 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.39 % 

💻 Operating System: 
Mac                      12 hrs 46 mins      █████████████████████████   100.00 % 
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


 Last Updated on 19/10/2023 18:38:21 UTC
<!--END_SECTION:waka-->

</details>

##

<p align="center"> 
  Visitor count<br>
  <img src="https://profile-counter.glitch.me/erzbir/count.svg" />
</p>
