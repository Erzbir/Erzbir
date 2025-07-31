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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C458%20hrs%2011%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-36-blue)

**🐱 My GitHub Data** 

> 📦 298.2 kB Used in GitHub's Storage 
 > 
> 🏆 202 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                217 commits         █████░░░░░░░░░░░░░░░░░░░░   20.36 % 
🌆 Daytime                298 commits         ███████░░░░░░░░░░░░░░░░░░   27.95 % 
🌃 Evening                311 commits         ███████░░░░░░░░░░░░░░░░░░   29.17 % 
🌙 Night                  240 commits         ██████░░░░░░░░░░░░░░░░░░░   22.51 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   142 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.32 % 
Tuesday                  198 commits         █████░░░░░░░░░░░░░░░░░░░░   18.57 % 
Wednesday                138 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.95 % 
Thursday                 195 commits         █████░░░░░░░░░░░░░░░░░░░░   18.29 % 
Friday                   135 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.66 % 
Saturday                 133 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.48 % 
Sunday                   125 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.73 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
SCSS                     19 hrs 2 mins       ██████░░░░░░░░░░░░░░░░░░░   25.34 % 
HTML                     14 hrs 24 mins      █████░░░░░░░░░░░░░░░░░░░░   19.18 % 
JavaScript               9 hrs 45 mins       ███░░░░░░░░░░░░░░░░░░░░░░   13.00 % 
TypeScript               9 hrs 7 mins        ███░░░░░░░░░░░░░░░░░░░░░░   12.14 % 
Java                     8 hrs 24 mins       ███░░░░░░░░░░░░░░░░░░░░░░   11.18 % 

🔥 Editors: 
IntelliJ IDEA            74 hrs 32 mins      █████████████████████████   99.20 % 
CLion                    36 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   00.80 % 

💻 Operating System: 
Mac                      75 hrs 8 mins       █████████████████████████   100.00 % 
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


 Last Updated on 31/07/2025 18:55:36 UTC
<!--END_SECTION:waka-->

</details>

##
