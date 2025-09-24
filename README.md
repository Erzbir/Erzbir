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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C524%20hrs%2027%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-2-blue)

**🐱 My GitHub Data** 

> 📦 328.7 kB Used in GitHub's Storage 
 > 
> 🏆 300 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 33 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                219 commits         █████░░░░░░░░░░░░░░░░░░░░   19.36 % 
🌆 Daytime                314 commits         ███████░░░░░░░░░░░░░░░░░░   27.76 % 
🌃 Evening                348 commits         ████████░░░░░░░░░░░░░░░░░   30.77 % 
🌙 Night                  250 commits         ██████░░░░░░░░░░░░░░░░░░░   22.10 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   147 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.00 % 
Tuesday                  210 commits         █████░░░░░░░░░░░░░░░░░░░░   18.57 % 
Wednesday                138 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.20 % 
Thursday                 197 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.42 % 
Friday                   141 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.47 % 
Saturday                 138 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.20 % 
Sunday                   160 commits         ████░░░░░░░░░░░░░░░░░░░░░   14.15 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     29 mins             █████████████████████████   99.16 % 
Python                   0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.32 % 
SQL                      0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.29 % 
XML                      0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.21 % 
Text                     0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.02 % 

🔥 Editors: 
IntelliJ IDEA            29 mins             █████████████████████████   99.37 % 
PyCharm                  0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.63 % 

💻 Operating System: 
Mac                      29 mins             █████████████████████████   100.00 % 
```

**I Mostly Code in Java** 

```text
Java                     14 repos            █████████████░░░░░░░░░░░░   53.85 % 
Rust                     3 repos             ███░░░░░░░░░░░░░░░░░░░░░░   11.54 % 
Lua                      1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   03.85 % 
SCSS                     1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   03.85 % 
JavaScript               1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   03.85 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 24/09/2025 18:46:06 UTC
<!--END_SECTION:waka-->

</details>

##
