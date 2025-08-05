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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C497%20hrs%2048%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-28-blue)

**🐱 My GitHub Data** 

> 📦 299.2 kB Used in GitHub's Storage 
 > 
> 🏆 274 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                219 commits         █████░░░░░░░░░░░░░░░░░░░░   19.59 % 
🌆 Daytime                307 commits         ███████░░░░░░░░░░░░░░░░░░   27.46 % 
🌃 Evening                346 commits         ████████░░░░░░░░░░░░░░░░░   30.95 % 
🌙 Night                  246 commits         ██████░░░░░░░░░░░░░░░░░░░   22.00 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   144 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.88 % 
Tuesday                  212 commits         █████░░░░░░░░░░░░░░░░░░░░   18.96 % 
Wednesday                138 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.34 % 
Thursday                 195 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.44 % 
Friday                   137 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.25 % 
Saturday                 137 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.25 % 
Sunday                   155 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.86 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
JavaScript               13 hrs 33 mins      █████░░░░░░░░░░░░░░░░░░░░   21.75 % 
Java                     12 hrs 50 mins      █████░░░░░░░░░░░░░░░░░░░░   20.59 % 
SCSS                     12 hrs 36 mins      █████░░░░░░░░░░░░░░░░░░░░   20.22 % 
HTML                     7 hrs 42 mins       ███░░░░░░░░░░░░░░░░░░░░░░   12.36 % 
YAML                     5 hrs 58 mins       ██░░░░░░░░░░░░░░░░░░░░░░░   09.59 % 

🔥 Editors: 
IntelliJ IDEA            62 hrs 20 mins      █████████████████████████   100.00 % 

💻 Operating System: 
Mac                      62 hrs 20 mins      █████████████████████████   100.00 % 
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


 Last Updated on 05/08/2025 18:58:03 UTC
<!--END_SECTION:waka-->

</details>

##
