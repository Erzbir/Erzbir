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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C609%20hrs%2035%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-2-blue)

**🐱 My GitHub Data** 

> 📦 351.9 kB Used in GitHub's Storage 
 > 
> 🏆 299 Contributions in the Year 2025
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
SCSS                     3 hrs 6 mins        ████████░░░░░░░░░░░░░░░░░   31.70 % 
CSS                      1 hr 33 mins        ████░░░░░░░░░░░░░░░░░░░░░   15.81 % 
HTML                     1 hr 23 mins        ████░░░░░░░░░░░░░░░░░░░░░   14.23 % 
YAML                     59 mins             ███░░░░░░░░░░░░░░░░░░░░░░   10.14 % 
Java                     50 mins             ██░░░░░░░░░░░░░░░░░░░░░░░   08.51 % 

🔥 Editors: 
IntelliJ IDEA            9 hrs               ███████████████████████░░   91.72 % 
Rustrover                40 mins             ██░░░░░░░░░░░░░░░░░░░░░░░   06.95 % 
Neovim                   7 mins              ░░░░░░░░░░░░░░░░░░░░░░░░░   01.33 % 

💻 Operating System: 
Mac                      9 hrs 49 mins       █████████████████████████   100.00 % 
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


 Last Updated on 04/12/2025 18:55:56 UTC
<!--END_SECTION:waka-->

</details>

##
