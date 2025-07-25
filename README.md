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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C383%20hrs%203%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-12-blue)

**🐱 My GitHub Data** 

> 📦 295.6 kB Used in GitHub's Storage 
 > 
> 🏆 202 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 30 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                223 commits         █████░░░░░░░░░░░░░░░░░░░░   20.82 % 
🌆 Daytime                304 commits         ███████░░░░░░░░░░░░░░░░░░   28.38 % 
🌃 Evening                307 commits         ███████░░░░░░░░░░░░░░░░░░   28.66 % 
🌙 Night                  237 commits         ██████░░░░░░░░░░░░░░░░░░░   22.13 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   131 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.23 % 
Tuesday                  204 commits         █████░░░░░░░░░░░░░░░░░░░░   19.05 % 
Wednesday                155 commits         ████░░░░░░░░░░░░░░░░░░░░░   14.47 % 
Thursday                 200 commits         █████░░░░░░░░░░░░░░░░░░░░   18.67 % 
Friday                   123 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.48 % 
Saturday                 133 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.42 % 
Sunday                   125 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.67 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
HTML                     8 hrs 23 mins       █████████░░░░░░░░░░░░░░░░   34.38 % 
SCSS                     6 hrs 15 mins       ██████░░░░░░░░░░░░░░░░░░░   25.66 % 
YAML                     1 hr 43 mins        ██░░░░░░░░░░░░░░░░░░░░░░░   07.09 % 
JavaScript               1 hr 33 mins        ██░░░░░░░░░░░░░░░░░░░░░░░   06.36 % 
TypeScript               1 hr 16 mins        █░░░░░░░░░░░░░░░░░░░░░░░░   05.21 % 

🔥 Editors: 
IntelliJ IDEA            22 hrs 17 mins      ███████████████████████░░   91.33 % 
CLion                    2 hrs 2 mins        ██░░░░░░░░░░░░░░░░░░░░░░░   08.40 % 
Rustrover                3 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.27 % 

💻 Operating System: 
Mac                      24 hrs 24 mins      █████████████████████████   100.00 % 
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


 Last Updated on 24/07/2025 18:54:27 UTC
<!--END_SECTION:waka-->

</details>

##
