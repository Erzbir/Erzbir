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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C551%20hrs%2054%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-2-blue)

**🐱 My GitHub Data** 

> 📦 349.5 kB Used in GitHub's Storage 
 > 
> 🏆 219 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 14 Private Repositories 
 > 
**I'm an Early 🐤** 

```text
🌞 Morning                220 commits         ██████░░░░░░░░░░░░░░░░░░░   23.06 % 
🌆 Daytime                301 commits         ████████░░░░░░░░░░░░░░░░░   31.55 % 
🌃 Evening                250 commits         ███████░░░░░░░░░░░░░░░░░░   26.21 % 
🌙 Night                  183 commits         █████░░░░░░░░░░░░░░░░░░░░   19.18 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   158 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.56 % 
Tuesday                  171 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.92 % 
Wednesday                101 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.59 % 
Thursday                 153 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.04 % 
Friday                   115 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.05 % 
Saturday                 131 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.73 % 
Sunday                   125 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.10 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     4 hrs 55 mins       ██████████████████░░░░░░░   70.95 % 
SCSS                     46 mins             ███░░░░░░░░░░░░░░░░░░░░░░   11.06 % 
JavaScript               36 mins             ██░░░░░░░░░░░░░░░░░░░░░░░   08.73 % 
YAML                     32 mins             ██░░░░░░░░░░░░░░░░░░░░░░░   07.73 % 
Properties               3 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   00.94 % 

🔥 Editors: 
IntelliJ IDEA            6 hrs 57 mins       █████████████████████████   100.00 % 

💻 Operating System: 
Mac                      6 hrs 57 mins       █████████████████████████   100.00 % 
```

**I Mostly Code in Java** 

```text
Java                     15 repos            █████████████░░░░░░░░░░░░   53.57 % 
Rust                     3 repos             ███░░░░░░░░░░░░░░░░░░░░░░   10.71 % 
Lua                      1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   03.57 % 
SCSS                     1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   03.57 % 
JavaScript               1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   03.57 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 13/11/2025 18:51:21 UTC
<!--END_SECTION:waka-->

</details>

##
