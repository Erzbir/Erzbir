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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C521%20hrs%2037%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-0-blue)

**🐱 My GitHub Data** 

> 📦 299.4 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 13 Private Repositories 
 > 
**I'm a Night 🦉** 

```text
🌞 Morning                219 commits         █████░░░░░░░░░░░░░░░░░░░░   19.38 % 
🌆 Daytime                313 commits         ███████░░░░░░░░░░░░░░░░░░   27.70 % 
🌃 Evening                348 commits         ████████░░░░░░░░░░░░░░░░░   30.80 % 
🌙 Night                  250 commits         ██████░░░░░░░░░░░░░░░░░░░   22.12 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   146 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.92 % 
Tuesday                  210 commits         █████░░░░░░░░░░░░░░░░░░░░   18.58 % 
Wednesday                138 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.21 % 
Thursday                 197 commits         ████░░░░░░░░░░░░░░░░░░░░░   17.43 % 
Friday                   141 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.48 % 
Saturday                 138 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.21 % 
Sunday                   160 commits         ████░░░░░░░░░░░░░░░░░░░░░   14.16 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
CLASS                    1 hr 1 min          ████████░░░░░░░░░░░░░░░░░   33.30 % 
Java                     42 mins             ██████░░░░░░░░░░░░░░░░░░░   23.09 % 
Java Properties          26 mins             ████░░░░░░░░░░░░░░░░░░░░░   14.52 % 
Python                   22 mins             ███░░░░░░░░░░░░░░░░░░░░░░   12.36 % 
HTML                     9 mins              █░░░░░░░░░░░░░░░░░░░░░░░░   04.99 % 

🔥 Editors: 
IntelliJ IDEA            2 hrs 40 mins       ██████████████████████░░░   87.58 % 
PyCharm                  22 mins             ███░░░░░░░░░░░░░░░░░░░░░░   12.42 % 

💻 Operating System: 
Mac                      3 hrs 3 mins        █████████████████████████   100.00 % 
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


 Last Updated on 19/08/2025 18:48:52 UTC
<!--END_SECTION:waka-->

</details>

##
