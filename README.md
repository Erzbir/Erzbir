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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C395%20hrs%2057%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-36-blue)

**🐱 My GitHub Data** 

> 📦 296.4 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 30 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                225 commits         █████░░░░░░░░░░░░░░░░░░░░   20.47 % 
🌆 Daytime                316 commits         ███████░░░░░░░░░░░░░░░░░░   28.75 % 
🌃 Evening                321 commits         ███████░░░░░░░░░░░░░░░░░░   29.21 % 
🌙 Night                  237 commits         █████░░░░░░░░░░░░░░░░░░░░   21.57 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   131 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.92 % 
Tuesday                  204 commits         █████░░░░░░░░░░░░░░░░░░░░   18.56 % 
Wednesday                155 commits         ████░░░░░░░░░░░░░░░░░░░░░   14.10 % 
Thursday                 200 commits         █████░░░░░░░░░░░░░░░░░░░░   18.20 % 
Friday                   151 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.74 % 
Saturday                 133 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.10 % 
Sunday                   125 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.37 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
HTML                     12 hrs 38 mins      █████████░░░░░░░░░░░░░░░░   35.21 % 
SCSS                     11 hrs 49 mins      ████████░░░░░░░░░░░░░░░░░   32.93 % 
YAML                     2 hrs 42 mins       ██░░░░░░░░░░░░░░░░░░░░░░░   07.54 % 
JavaScript               2 hrs 3 mins        █░░░░░░░░░░░░░░░░░░░░░░░░   05.75 % 
TypeScript               1 hr 55 mins        █░░░░░░░░░░░░░░░░░░░░░░░░   05.34 % 

🔥 Editors: 
IntelliJ IDEA            34 hrs 55 mins      ████████████████████████░   97.28 % 
CLion                    54 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.53 % 
Rustrover                3 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.18 % 

💻 Operating System: 
Mac                      35 hrs 54 mins      █████████████████████████   100.00 % 
```

**I Mostly Code in Java** 

```text
Java                     13 repos            ██████████████░░░░░░░░░░░   56.52 % 
HTML                     2 repos             ██░░░░░░░░░░░░░░░░░░░░░░░   08.70 % 
SCSS                     1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.35 % 
JavaScript               1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.35 % 
C                        1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   04.35 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 25/07/2025 18:53:57 UTC
<!--END_SECTION:waka-->

</details>

##
