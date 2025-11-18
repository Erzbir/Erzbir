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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C578%20hrs%2051%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-6-blue)

**🐱 My GitHub Data** 

> 📦 350.3 kB Used in GitHub's Storage 
 > 
> 🏆 282 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 14 Private Repositories 
 > 
**I'm an Early 🐤** 

```text
🌞 Morning                223 commits         ██████░░░░░░░░░░░░░░░░░░░   22.32 % 
🌆 Daytime                313 commits         ████████░░░░░░░░░░░░░░░░░   31.33 % 
🌃 Evening                268 commits         ███████░░░░░░░░░░░░░░░░░░   26.83 % 
🌙 Night                  195 commits         █████░░░░░░░░░░░░░░░░░░░░   19.52 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   166 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.62 % 
Tuesday                  190 commits         █████░░░░░░░░░░░░░░░░░░░░   19.02 % 
Wednesday                101 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.11 % 
Thursday                 153 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.32 % 
Friday                   117 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.71 % 
Saturday                 135 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.51 % 
Sunday                   137 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.71 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     24 hrs 4 mins       █████████████░░░░░░░░░░░░   52.94 % 
HTML                     6 hrs 13 mins       ███░░░░░░░░░░░░░░░░░░░░░░   13.68 % 
YAML                     5 hrs 40 mins       ███░░░░░░░░░░░░░░░░░░░░░░   12.47 % 
SCSS                     3 hrs 32 mins       ██░░░░░░░░░░░░░░░░░░░░░░░   07.78 % 
JavaScript               2 hrs 33 mins       █░░░░░░░░░░░░░░░░░░░░░░░░   05.63 % 

🔥 Editors: 
IntelliJ IDEA            45 hrs 28 mins      █████████████████████████   100.00 % 

💻 Operating System: 
Mac                      45 hrs 28 mins      █████████████████████████   100.00 % 
```

**I Mostly Code in Java** 

```text
Java                     15 repos            █████████████░░░░░░░░░░░░   53.57 % 
Rust                     3 repos             ███░░░░░░░░░░░░░░░░░░░░░░   10.71 % 
HTML                     3 repos             ███░░░░░░░░░░░░░░░░░░░░░░   10.71 % 
Lua                      1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   03.57 % 
JavaScript               1 repo              █░░░░░░░░░░░░░░░░░░░░░░░░   03.57 % 
```



**Timeline**

![Lines of Code chart](https://raw.githubusercontent.com/Erzbir/Erzbir/main/assets/bar_graph.png)


 Last Updated on 18/11/2025 18:52:58 UTC
<!--END_SECTION:waka-->

</details>

##
