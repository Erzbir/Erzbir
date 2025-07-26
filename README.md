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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C402%20hrs%2013%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-36-blue)

**🐱 My GitHub Data** 

> 📦 297.6 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 30 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm an Early 🐤** 

```text
🌞 Morning                265 commits         █████░░░░░░░░░░░░░░░░░░░░   20.15 % 
🌆 Daytime                408 commits         ████████░░░░░░░░░░░░░░░░░   31.03 % 
🌃 Evening                405 commits         ████████░░░░░░░░░░░░░░░░░   30.80 % 
🌙 Night                  237 commits         █████░░░░░░░░░░░░░░░░░░░░   18.02 % 
```
📅 **I'm Most Productive on Thursday** 

```text
Monday                   131 commits         ██░░░░░░░░░░░░░░░░░░░░░░░   09.96 % 
Tuesday                  237 commits         █████░░░░░░░░░░░░░░░░░░░░   18.02 % 
Wednesday                206 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.67 % 
Thursday                 254 commits         █████░░░░░░░░░░░░░░░░░░░░   19.32 % 
Friday                   208 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.82 % 
Saturday                 154 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.71 % 
Sunday                   125 commits         ██░░░░░░░░░░░░░░░░░░░░░░░   09.51 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
SCSS                     15 hrs 54 mins      █████████░░░░░░░░░░░░░░░░   37.80 % 
HTML                     13 hrs 32 mins      ████████░░░░░░░░░░░░░░░░░   32.16 % 
YAML                     2 hrs 48 mins       ██░░░░░░░░░░░░░░░░░░░░░░░   06.69 % 
Markdown                 2 hrs 6 mins        █░░░░░░░░░░░░░░░░░░░░░░░░   05.01 % 
JavaScript               2 hrs 3 mins        █░░░░░░░░░░░░░░░░░░░░░░░░   04.90 % 

🔥 Editors: 
IntelliJ IDEA            41 hrs 9 mins       ████████████████████████░   97.78 % 
CLion                    52 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.07 % 
Rustrover                3 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.16 % 

💻 Operating System: 
Mac                      42 hrs 5 mins       █████████████████████████   100.00 % 
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


 Last Updated on 26/07/2025 18:51:52 UTC
<!--END_SECTION:waka-->

</details>

##
