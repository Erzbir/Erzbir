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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C513%20hrs%2055%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-0-blue)

**🐱 My GitHub Data** 

> 📦 299.3 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                219 commits         █████░░░░░░░░░░░░░░░░░░░░   19.52 % 
🌆 Daytime                305 commits         ███████░░░░░░░░░░░░░░░░░░   27.18 % 
🌃 Evening                348 commits         ████████░░░░░░░░░░░░░░░░░   31.02 % 
🌙 Night                  250 commits         ██████░░░░░░░░░░░░░░░░░░░   22.28 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   144 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.83 % 
Tuesday                  210 commits         █████░░░░░░░░░░░░░░░░░░░░   18.72 % 
Wednesday                138 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.30 % 
Thursday                 197 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.56 % 
Friday                   141 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.57 % 
Saturday                 137 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.21 % 
Sunday                   155 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.81 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
SCSS                     13 hrs 25 mins      ███████░░░░░░░░░░░░░░░░░░   28.84 % 
Java                     10 hrs 43 mins      ██████░░░░░░░░░░░░░░░░░░░   23.03 % 
JavaScript               7 hrs 57 mins       ████░░░░░░░░░░░░░░░░░░░░░   17.11 % 
HTML                     5 hrs 30 mins       ███░░░░░░░░░░░░░░░░░░░░░░   11.84 % 
YAML                     3 hrs 57 mins       ██░░░░░░░░░░░░░░░░░░░░░░░   08.52 % 

🔥 Editors: 
IntelliJ IDEA            45 hrs 52 mins      █████████████████████████   98.55 % 
PyCharm                  35 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   01.26 % 
RustRover                5 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.19 % 

💻 Operating System: 
Mac                      46 hrs 32 mins      █████████████████████████   100.00 % 
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


 Last Updated on 08/08/2025 18:52:26 UTC
<!--END_SECTION:waka-->

</details>

##
