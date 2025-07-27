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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C414%20hrs%2011%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-36-blue)

**🐱 My GitHub Data** 

> 📦 297.6 kB Used in GitHub's Storage 
 > 
> 🏆 254 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 30 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm an Early 🐤** 

```text
🌞 Morning                278 commits         █████░░░░░░░░░░░░░░░░░░░░   19.93 % 
🌆 Daytime                439 commits         ████████░░░░░░░░░░░░░░░░░   31.47 % 
🌃 Evening                441 commits         ████████░░░░░░░░░░░░░░░░░   31.61 % 
🌙 Night                  237 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.99 % 
```
📅 **I'm Most Productive on Thursday** 

```text
Monday                   131 commits         ██░░░░░░░░░░░░░░░░░░░░░░░   09.39 % 
Tuesday                  248 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.78 % 
Wednesday                223 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.99 % 
Thursday                 272 commits         █████░░░░░░░░░░░░░░░░░░░░   19.50 % 
Friday                   227 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.27 % 
Saturday                 164 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.76 % 
Sunday                   130 commits         ██░░░░░░░░░░░░░░░░░░░░░░░   09.32 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
SCSS                     24 hrs 3 mins       ███████████░░░░░░░░░░░░░░   44.50 % 
HTML                     15 hrs 37 mins      ███████░░░░░░░░░░░░░░░░░░   28.92 % 
YAML                     3 hrs 23 mins       ██░░░░░░░░░░░░░░░░░░░░░░░   06.26 % 
Markdown                 2 hrs 28 mins       █░░░░░░░░░░░░░░░░░░░░░░░░   04.58 % 
TypeScript               2 hrs 5 mins        █░░░░░░░░░░░░░░░░░░░░░░░░   03.86 % 

🔥 Editors: 
IntelliJ IDEA            52 hrs 41 mins      ████████████████████████░   97.49 % 
CLion                    1 hr 17 mins        █░░░░░░░░░░░░░░░░░░░░░░░░   02.39 % 
Rustrover                3 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.12 % 

💻 Operating System: 
Mac                      54 hrs 3 mins       █████████████████████████   100.00 % 
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


 Last Updated on 27/07/2025 18:51:36 UTC
<!--END_SECTION:waka-->

</details>

##
