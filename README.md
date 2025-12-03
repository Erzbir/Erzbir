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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C605%20hrs%2031%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-3-blue)

**🐱 My GitHub Data** 

> 📦 351.9 kB Used in GitHub's Storage 
 > 
> 🏆 298 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 34 Public Repositories 
 > 
> 🔑 14 Private Repositories 
 > 
**I'm an Early 🐤** 

```text
🌞 Morning                204 commits         █████░░░░░░░░░░░░░░░░░░░░   20.94 % 
🌆 Daytime                306 commits         ████████░░░░░░░░░░░░░░░░░   31.42 % 
🌃 Evening                269 commits         ███████░░░░░░░░░░░░░░░░░░   27.62 % 
🌙 Night                  195 commits         █████░░░░░░░░░░░░░░░░░░░░   20.02 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   147 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.09 % 
Tuesday                  191 commits         █████░░░░░░░░░░░░░░░░░░░░   19.61 % 
Wednesday                109 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.19 % 
Thursday                 157 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.12 % 
Friday                   117 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.01 % 
Saturday                 116 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.91 % 
Sunday                   137 commits         ████░░░░░░░░░░░░░░░░░░░░░   14.07 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
CSS                      2 hrs 14 mins       █████████░░░░░░░░░░░░░░░░   34.36 % 
SCSS                     2 hrs 3 mins        ████████░░░░░░░░░░░░░░░░░   31.58 % 
HTML                     52 mins             ███░░░░░░░░░░░░░░░░░░░░░░   13.35 % 
Rust                     22 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   05.67 % 
Text                     18 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   04.75 % 

🔥 Editors: 
IntelliJ IDEA            5 hrs 42 mins       ██████████████████████░░░   87.54 % 
Rustrover                40 mins             ███░░░░░░░░░░░░░░░░░░░░░░   10.46 % 
Neovim                   7 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   02.00 % 

💻 Operating System: 
Mac                      6 hrs 31 mins       █████████████████████████   100.00 % 
```

**I Mostly Code in Java** 

```text
Java                     15 repos            █████████████░░░░░░░░░░░░   51.72 % 
Rust                     3 repos             ███░░░░░░░░░░░░░░░░░░░░░░   10.34 % 
Lua                      1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   03.45 % 
SCSS                     1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   03.45 % 
JavaScript               1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   03.45 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 03/12/2025 18:54:46 UTC
<!--END_SECTION:waka-->

</details>

##
