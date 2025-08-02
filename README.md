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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C481%20hrs%205%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-36-blue)

**🐱 My GitHub Data** 

> 📦 298.5 kB Used in GitHub's Storage 
 > 
> 🏆 219 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                217 commits         █████░░░░░░░░░░░░░░░░░░░░   20.13 % 
🌆 Daytime                298 commits         ███████░░░░░░░░░░░░░░░░░░   27.64 % 
🌃 Evening                317 commits         ███████░░░░░░░░░░░░░░░░░░   29.41 % 
🌙 Night                  246 commits         ██████░░░░░░░░░░░░░░░░░░░   22.82 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   142 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.17 % 
Tuesday                  198 commits         █████░░░░░░░░░░░░░░░░░░░░   18.37 % 
Wednesday                138 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.80 % 
Thursday                 195 commits         █████░░░░░░░░░░░░░░░░░░░░   18.09 % 
Friday                   137 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.71 % 
Saturday                 137 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.71 % 
Sunday                   131 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.15 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     13 hrs 37 mins      █████░░░░░░░░░░░░░░░░░░░░   19.11 % 
SCSS                     11 hrs 19 mins      ████░░░░░░░░░░░░░░░░░░░░░   15.88 % 
JavaScript               11 hrs 17 mins      ████░░░░░░░░░░░░░░░░░░░░░   15.84 % 
HTML                     10 hrs 56 mins      ████░░░░░░░░░░░░░░░░░░░░░   15.35 % 
TypeScript               8 hrs 27 mins       ███░░░░░░░░░░░░░░░░░░░░░░   11.87 % 

🔥 Editors: 
IntelliJ IDEA            70 hrs 51 mins      █████████████████████████   99.40 % 
CLion                    25 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   00.60 % 

💻 Operating System: 
Mac                      71 hrs 17 mins      █████████████████████████   100.00 % 
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


 Last Updated on 02/08/2025 18:52:02 UTC
<!--END_SECTION:waka-->

</details>

##
