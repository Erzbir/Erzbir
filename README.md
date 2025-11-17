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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C581%20hrs%2030%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-6-blue)

**🐱 My GitHub Data** 

> 📦 350.2 kB Used in GitHub's Storage 
 > 
> 🏆 270 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 14 Private Repositories 
 > 
**I'm an Early 🐤** 

```text
🌞 Morning                222 commits         ██████░░░░░░░░░░░░░░░░░░░   22.49 % 
🌆 Daytime                302 commits         ████████░░░░░░░░░░░░░░░░░   30.60 % 
🌃 Evening                268 commits         ███████░░░░░░░░░░░░░░░░░░   27.15 % 
🌙 Night                  195 commits         █████░░░░░░░░░░░░░░░░░░░░   19.76 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   166 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.82 % 
Tuesday                  178 commits         █████░░░░░░░░░░░░░░░░░░░░   18.03 % 
Wednesday                101 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.23 % 
Thursday                 153 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.50 % 
Friday                   117 commits         ███░░░░░░░░░░░░░░░░░░░░░░   11.85 % 
Saturday                 135 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.68 % 
Sunday                   137 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.88 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     24 hrs 3 mins       ███████████████░░░░░░░░░░   61.15 % 
YAML                     4 hrs 35 mins       ███░░░░░░░░░░░░░░░░░░░░░░   11.68 % 
HTML                     4 hrs 10 mins       ███░░░░░░░░░░░░░░░░░░░░░░   10.60 % 
JavaScript               2 hrs 30 mins       ██░░░░░░░░░░░░░░░░░░░░░░░   06.39 % 
SCSS                     53 mins             █░░░░░░░░░░░░░░░░░░░░░░░░   02.29 % 

🔥 Editors: 
IntelliJ IDEA            39 hrs 20 mins      █████████████████████████   100.00 % 

💻 Operating System: 
Mac                      39 hrs 20 mins      █████████████████████████   100.00 % 
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


 Last Updated on 17/11/2025 18:51:15 UTC
<!--END_SECTION:waka-->

</details>

##
