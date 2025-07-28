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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C423%20hrs%2056%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-36-blue)

**🐱 My GitHub Data** 

> 📦 298.0 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm an Early 🐤** 

```text
🌞 Morning                280 commits         █████░░░░░░░░░░░░░░░░░░░░   19.87 % 
🌆 Daytime                441 commits         ████████░░░░░░░░░░░░░░░░░   31.30 % 
🌃 Evening                451 commits         ████████░░░░░░░░░░░░░░░░░   32.01 % 
🌙 Night                  237 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.82 % 
```
📅 **I'm Most Productive on Thursday** 

```text
Monday                   145 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.29 % 
Tuesday                  248 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.60 % 
Wednesday                223 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.83 % 
Thursday                 272 commits         █████░░░░░░░░░░░░░░░░░░░░   19.30 % 
Friday                   227 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.11 % 
Saturday                 164 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.64 % 
Sunday                   130 commits         ██░░░░░░░░░░░░░░░░░░░░░░░   09.23 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
SCSS                     24 hrs 4 mins       █████████░░░░░░░░░░░░░░░░   37.99 % 
HTML                     19 hrs 23 mins      ████████░░░░░░░░░░░░░░░░░   30.59 % 
TypeScript               6 hrs 56 mins       ███░░░░░░░░░░░░░░░░░░░░░░   10.95 % 
YAML                     3 hrs 34 mins       █░░░░░░░░░░░░░░░░░░░░░░░░   05.65 % 
JavaScript               2 hrs 37 mins       █░░░░░░░░░░░░░░░░░░░░░░░░   04.15 % 

🔥 Editors: 
IntelliJ IDEA            62 hrs 27 mins      █████████████████████████   98.57 % 
CLion                    50 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   01.33 % 
Rustrover                3 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.10 % 

💻 Operating System: 
Mac                      63 hrs 21 mins      █████████████████████████   100.00 % 
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


 Last Updated on 28/07/2025 18:55:59 UTC
<!--END_SECTION:waka-->

</details>

##
