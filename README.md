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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C489%20hrs%2042%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-36-blue)

**🐱 My GitHub Data** 

> 📦 299.2 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                219 commits         █████░░░░░░░░░░░░░░░░░░░░   19.80 % 
🌆 Daytime                304 commits         ███████░░░░░░░░░░░░░░░░░░   27.49 % 
🌃 Evening                337 commits         ████████░░░░░░░░░░░░░░░░░   30.47 % 
🌙 Night                  246 commits         ██████░░░░░░░░░░░░░░░░░░░   22.24 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   146 commits         ███░░░░░░░░░░░░░░░░░░░░░░   13.20 % 
Tuesday                  198 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.90 % 
Wednesday                138 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.48 % 
Thursday                 195 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.63 % 
Friday                   137 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.39 % 
Saturday                 137 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.39 % 
Sunday                   155 commits         ████░░░░░░░░░░░░░░░░░░░░░   14.01 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
Java                     18 hrs 58 mins      ███████░░░░░░░░░░░░░░░░░░   28.86 % 
JavaScript               14 hrs 10 mins      █████░░░░░░░░░░░░░░░░░░░░   21.55 % 
YAML                     8 hrs 3 mins        ███░░░░░░░░░░░░░░░░░░░░░░   12.26 % 
HTML                     6 hrs 46 mins       ███░░░░░░░░░░░░░░░░░░░░░░   10.30 % 
SCSS                     5 hrs 56 mins       ██░░░░░░░░░░░░░░░░░░░░░░░   09.03 % 

🔥 Editors: 
IntelliJ IDEA            65 hrs 45 mins      █████████████████████████   100.00 % 

💻 Operating System: 
Mac                      65 hrs 45 mins      █████████████████████████   100.00 % 
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


 Last Updated on 04/08/2025 18:56:34 UTC
<!--END_SECTION:waka-->

</details>

##
