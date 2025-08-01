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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C467%20hrs%2022%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-36-blue)

**🐱 My GitHub Data** 

> 📦 298.4 kB Used in GitHub's Storage 
 > 
> 🏆 204 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                217 commits         █████░░░░░░░░░░░░░░░░░░░░   20.32 % 
🌆 Daytime                298 commits         ███████░░░░░░░░░░░░░░░░░░   27.90 % 
🌃 Evening                313 commits         ███████░░░░░░░░░░░░░░░░░░   29.31 % 
🌙 Night                  240 commits         ██████░░░░░░░░░░░░░░░░░░░   22.47 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   142 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.30 % 
Tuesday                  198 commits         █████░░░░░░░░░░░░░░░░░░░░   18.54 % 
Wednesday                138 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.92 % 
Thursday                 195 commits         █████░░░░░░░░░░░░░░░░░░░░   18.26 % 
Friday                   137 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.83 % 
Saturday                 133 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.45 % 
Sunday                   125 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.70 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
SCSS                     15 hrs 23 mins      █████░░░░░░░░░░░░░░░░░░░░   21.54 % 
HTML                     11 hrs 34 mins      ████░░░░░░░░░░░░░░░░░░░░░   16.22 % 
JavaScript               11 hrs 16 mins      ████░░░░░░░░░░░░░░░░░░░░░   15.80 % 
Java                     9 hrs               ███░░░░░░░░░░░░░░░░░░░░░░   12.63 % 
TypeScript               8 hrs 28 mins       ███░░░░░░░░░░░░░░░░░░░░░░   11.88 % 

🔥 Editors: 
IntelliJ IDEA            70 hrs 58 mins      █████████████████████████   99.38 % 
CLion                    26 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   00.62 % 

💻 Operating System: 
Mac                      71 hrs 24 mins      █████████████████████████   100.00 % 
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


 Last Updated on 01/08/2025 18:54:35 UTC
<!--END_SECTION:waka-->

</details>

##
