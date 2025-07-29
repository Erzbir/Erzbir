<div dir="auto">
  <p dir="auto" align="center">
  </p>
  <p dir="auto" align="center">
    <a href="https://erzbir.com" rel="nofollow">Blog</a> |
    <a href="https://erzbir.com/about/" rel="nofollow">About</a> |
    <a href="mailto:contact@erzbir.com">Email</a>
  </p>
</div>

```bash
blog="https://erzbir.com"

name=$(curl -s $blog \
	| sed -n '/<head>/,/<\/head>/p' \
	| tr '\n' ' ' \
	| sed -n 's:.*<title>\([^<]*\)</title>.*:\1:p')

about=$(curl -s $blog/about \
	| sed -n '/<about>/,/<\/about>/p' \
	| perl -pe 's/<[^>]+>/\n/g; s/^\s+|\s+$/\n/g;')
```

##

<a href="https://github.com/Erzbir">
<img src="https://github-profile-summary-cards.vercel.app/api/cards/profile-details?username=Erzbir&theme=tokyonight" alt="summary">
</a>

##

<details>
<summary>📈 Wakatime Stats</summary>
<br>

![Erzbir's wakatime stats](https://github-readme-stats.vercel.app/api/wakatime?username=Erzbir\&layout=compact)

##

<!--START_SECTION:waka-->
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C435%20hrs%2027%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-36-blue)

**🐱 My GitHub Data** 

> 📦 299.2 kB Used in GitHub's Storage 
 > 
> 🏆 292 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm an Early 🐤** 

```text
🌞 Morning                293 commits         █████░░░░░░░░░░░░░░░░░░░░   19.62 % 
🌆 Daytime                476 commits         ████████░░░░░░░░░░░░░░░░░   31.88 % 
🌃 Evening                487 commits         ████████░░░░░░░░░░░░░░░░░   32.62 % 
🌙 Night                  237 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.87 % 
```
📅 **I'm Most Productive on Thursday** 

```text
Monday                   142 commits         ██░░░░░░░░░░░░░░░░░░░░░░░   09.51 % 
Tuesday                  268 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.95 % 
Wednesday                240 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.08 % 
Thursday                 290 commits         █████░░░░░░░░░░░░░░░░░░░░   19.42 % 
Friday                   246 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.48 % 
Saturday                 174 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.65 % 
Sunday                   133 commits         ██░░░░░░░░░░░░░░░░░░░░░░░   08.91 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
SCSS                     22 hrs 19 mins      ████████░░░░░░░░░░░░░░░░░   32.44 % 
HTML                     15 hrs 54 mins      ██████░░░░░░░░░░░░░░░░░░░   23.11 % 
TypeScript               6 hrs 57 mins       ███░░░░░░░░░░░░░░░░░░░░░░   10.11 % 
Java                     6 hrs 8 mins        ██░░░░░░░░░░░░░░░░░░░░░░░   08.92 % 
YAML                     5 hrs 44 mins       ██░░░░░░░░░░░░░░░░░░░░░░░   08.35 % 

🔥 Editors: 
IntelliJ IDEA            67 hrs 55 mins      █████████████████████████   98.68 % 
CLion                    50 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   01.22 % 
Rustrover                3 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.10 % 

💻 Operating System: 
Mac                      68 hrs 49 mins      █████████████████████████   100.00 % 
```

**I Mostly Code in Java** 

```text
Java                     14 repos            ███████████████░░░░░░░░░░   58.33 % 
HTML                     2 repos             ██░░░░░░░░░░░░░░░░░░░░░░░   08.33 % 
SCSS                     1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.17 % 
JavaScript               1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.17 % 
C                        1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.17 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 29/07/2025 18:57:16 UTC
<!--END_SECTION:waka-->

</details>

##
