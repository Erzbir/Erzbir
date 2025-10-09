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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C526%20hrs%2043%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-7-blue)

**🐱 My GitHub Data** 

> 📦 349.4 kB Used in GitHub's Storage 
 > 
> 🏆 309 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 33 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                223 commits         █████░░░░░░░░░░░░░░░░░░░░   19.63 % 
🌆 Daytime                314 commits         ███████░░░░░░░░░░░░░░░░░░   27.64 % 
🌃 Evening                349 commits         ████████░░░░░░░░░░░░░░░░░   30.72 % 
🌙 Night                  250 commits         ██████░░░░░░░░░░░░░░░░░░░   22.01 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   147 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.94 % 
Tuesday                  210 commits         █████░░░░░░░░░░░░░░░░░░░░   18.49 % 
Wednesday                138 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.15 % 
Thursday                 202 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.78 % 
Friday                   141 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.41 % 
Saturday                 138 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.15 % 
Sunday                   160 commits         ████░░░░░░░░░░░░░░░░░░░░░   14.08 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
No Activity Tracked This Week

🔥 Editors: 
No Activity Tracked This Week

💻 Operating System: 
No Activity Tracked This Week
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


 Last Updated on 09/10/2025 18:48:32 UTC
<!--END_SECTION:waka-->

</details>

##
