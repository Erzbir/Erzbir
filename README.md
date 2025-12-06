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

[![Erzbir's WakaTime stats](https://github-readme-stats-fast.vercel.app/api/wakatime?username=Erzbir&layout=compact&theme=tokyonight)](https://github.com/Erzbir)

##

<!--START_SECTION:waka-->
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C611%20hrs%2024%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-5-blue)

**🐱 My GitHub Data** 

> 📦 351.9 kB Used in GitHub's Storage 
 > 
> 🏆 301 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 34 Public Repositories 
 > 
> 🔑 14 Private Repositories 
 > 
**I'm an Early 🐤** 

```text
🌞 Morning                204 commits         █████░░░░░░░░░░░░░░░░░░░░   20.90 % 
🌆 Daytime                306 commits         ████████░░░░░░░░░░░░░░░░░   31.35 % 
🌃 Evening                271 commits         ███████░░░░░░░░░░░░░░░░░░   27.77 % 
🌙 Night                  195 commits         █████░░░░░░░░░░░░░░░░░░░░   19.98 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   147 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.06 % 
Tuesday                  191 commits         █████░░░░░░░░░░░░░░░░░░░░   19.57 % 
Wednesday                109 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.17 % 
Thursday                 157 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.09 % 
Friday                   119 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.19 % 
Saturday                 116 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.89 % 
Sunday                   137 commits         ████░░░░░░░░░░░░░░░░░░░░░   14.04 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
SCSS                     2 hrs 26 mins       ██████░░░░░░░░░░░░░░░░░░░   25.97 % 
Java                     1 hr 39 mins        ████░░░░░░░░░░░░░░░░░░░░░   17.61 % 
HTML                     1 hr 23 mins        ████░░░░░░░░░░░░░░░░░░░░░   14.89 % 
YAML                     1 hr 3 mins         ███░░░░░░░░░░░░░░░░░░░░░░   11.21 % 
JavaScript               57 mins             ███░░░░░░░░░░░░░░░░░░░░░░   10.28 % 

🔥 Editors: 
IntelliJ IDEA            8 hrs 41 mins       ███████████████████████░░   92.67 % 
Rustrover                40 mins             ██░░░░░░░░░░░░░░░░░░░░░░░   07.28 % 
Neovim                   0 secs              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.05 % 

💻 Operating System: 
Mac                      9 hrs 22 mins       █████████████████████████   100.00 % 
```

**I Mostly Code in Java** 

```text
Java                     15 repos            ████████████░░░░░░░░░░░░░   50.00 % 
Rust                     3 repos             ██░░░░░░░░░░░░░░░░░░░░░░░   10.00 % 
Lua                      1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   03.33 % 
SCSS                     1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   03.33 % 
JavaScript               1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   03.33 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 05/12/2025 18:50:36 UTC
<!--END_SECTION:waka-->

</details>

##
