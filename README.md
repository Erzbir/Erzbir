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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C515%20hrs%2021%20mins-blue)

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
🌞 Morning                219 commits         █████░░░░░░░░░░░░░░░░░░░░   19.41 % 
🌆 Daytime                311 commits         ███████░░░░░░░░░░░░░░░░░░   27.57 % 
🌃 Evening                348 commits         ████████░░░░░░░░░░░░░░░░░   30.85 % 
🌙 Night                  250 commits         ██████░░░░░░░░░░░░░░░░░░░   22.16 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   144 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.77 % 
Tuesday                  210 commits         █████░░░░░░░░░░░░░░░░░░░░   18.62 % 
Wednesday                138 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.23 % 
Thursday                 197 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.46 % 
Friday                   141 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.50 % 
Saturday                 138 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.23 % 
Sunday                   160 commits         ████░░░░░░░░░░░░░░░░░░░░░   14.18 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
SCSS                     13 hrs 44 mins      █████████░░░░░░░░░░░░░░░░   37.53 % 
JavaScript               8 hrs 6 mins        ██████░░░░░░░░░░░░░░░░░░░   22.16 % 
HTML                     5 hrs 11 mins       ████░░░░░░░░░░░░░░░░░░░░░   14.18 % 
YAML                     3 hrs 34 mins       ██░░░░░░░░░░░░░░░░░░░░░░░   09.74 % 
Java                     1 hr 58 mins        █░░░░░░░░░░░░░░░░░░░░░░░░   05.40 % 

🔥 Editors: 
IntelliJ IDEA            35 hrs 55 mins      █████████████████████████   98.16 % 
PyCharm                  35 mins             ░░░░░░░░░░░░░░░░░░░░░░░░░   01.60 % 
RustRover                5 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.24 % 

💻 Operating System: 
Mac                      36 hrs 36 mins      █████████████████████████   100.00 % 
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


 Last Updated on 10/08/2025 18:49:38 UTC
<!--END_SECTION:waka-->

</details>

##
