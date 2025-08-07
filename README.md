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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C511%20hrs%202%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-24-blue)

**🐱 My GitHub Data** 

> 📦 299.3 kB Used in GitHub's Storage 
 > 
> 🏆 287 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                219 commits         █████░░░░░░░░░░░░░░░░░░░░   19.16 % 
🌆 Daytime                311 commits         ███████░░░░░░░░░░░░░░░░░░   27.21 % 
🌃 Evening                357 commits         ████████░░░░░░░░░░░░░░░░░   31.23 % 
🌙 Night                  256 commits         ██████░░░░░░░░░░░░░░░░░░░   22.40 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   144 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.60 % 
Tuesday                  214 commits         █████░░░░░░░░░░░░░░░░░░░░   18.72 % 
Wednesday                138 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.07 % 
Thursday                 204 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.85 % 
Friday                   146 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.77 % 
Saturday                 137 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.99 % 
Sunday                   160 commits         ████░░░░░░░░░░░░░░░░░░░░░   14.00 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
SCSS                     15 hrs 14 mins      ███████░░░░░░░░░░░░░░░░░░   28.86 % 
Java                     10 hrs 45 mins      █████░░░░░░░░░░░░░░░░░░░░   20.35 % 
JavaScript               8 hrs 56 mins       ████░░░░░░░░░░░░░░░░░░░░░   16.93 % 
HTML                     6 hrs 47 mins       ███░░░░░░░░░░░░░░░░░░░░░░   12.86 % 
YAML                     5 hrs 32 mins       ███░░░░░░░░░░░░░░░░░░░░░░   10.48 % 

🔥 Editors: 
IntelliJ IDEA            52 hrs 50 mins      █████████████████████████   99.99 % 
PyCharm                  0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.01 % 

💻 Operating System: 
Mac                      52 hrs 50 mins      █████████████████████████   100.00 % 
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


 Last Updated on 07/08/2025 18:57:28 UTC
<!--END_SECTION:waka-->

</details>

##
