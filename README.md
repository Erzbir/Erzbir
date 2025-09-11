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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C523%20hrs%2057%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-0-blue)

**🐱 My GitHub Data** 

> 📦 389.2 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 14 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                219 commits         █████░░░░░░░░░░░░░░░░░░░░   19.36 % 
🌆 Daytime                313 commits         ███████░░░░░░░░░░░░░░░░░░   27.67 % 
🌃 Evening                348 commits         ████████░░░░░░░░░░░░░░░░░   30.77 % 
🌙 Night                  251 commits         ██████░░░░░░░░░░░░░░░░░░░   22.19 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   146 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.91 % 
Tuesday                  211 commits         █████░░░░░░░░░░░░░░░░░░░░   18.66 % 
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
Java                     21 mins             █████████████████████████   99.85 % 
Kotlin                   0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.15 % 
SPI                      0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.00 % 
Java module              0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.00 % 
Markdown                 0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.00 % 

🔥 Editors: 
IntelliJ IDEA            21 mins             █████████████████████████   100.00 % 

💻 Operating System: 
Mac                      21 mins             █████████████████████████   100.00 % 
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


 Last Updated on 11/09/2025 18:44:43 UTC
<!--END_SECTION:waka-->

</details>

##
