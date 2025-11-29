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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C602%20hrs%207%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-5-blue)

**🐱 My GitHub Data** 

> 📦 350.3 kB Used in GitHub's Storage 
 > 
> 🏆 287 Contributions in the Year 2025
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 33 Public Repositories 
 > 
> 🔑 14 Private Repositories 
 > 
**I'm an Early 🐤** 

```text
🌞 Morning                204 commits         █████░░░░░░░░░░░░░░░░░░░░   21.14 % 
🌆 Daytime                298 commits         ████████░░░░░░░░░░░░░░░░░   30.88 % 
🌃 Evening                268 commits         ███████░░░░░░░░░░░░░░░░░░   27.77 % 
🌙 Night                  195 commits         █████░░░░░░░░░░░░░░░░░░░░   20.21 % 
```
📅 **I'm Most Productive on Tuesday** 

```text
Monday                   147 commits         ████░░░░░░░░░░░░░░░░░░░░░   15.23 % 
Tuesday                  190 commits         █████░░░░░░░░░░░░░░░░░░░░   19.69 % 
Wednesday                101 commits         ███░░░░░░░░░░░░░░░░░░░░░░   10.47 % 
Thursday                 157 commits         ████░░░░░░░░░░░░░░░░░░░░░   16.27 % 
Friday                   117 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.12 % 
Saturday                 116 commits         ███░░░░░░░░░░░░░░░░░░░░░░   12.02 % 
Sunday                   137 commits         ████░░░░░░░░░░░░░░░░░░░░░   14.20 % 
```


📊 **This Week I Spent My Time On** 

```text
🕑︎ Time Zone: Asia/Shanghai

💬 Programming Languages: 
CSS                      2 hrs 12 mins       ██████████░░░░░░░░░░░░░░░   40.57 % 
SCSS                     51 mins             ████░░░░░░░░░░░░░░░░░░░░░   15.63 % 
CLASS                    36 mins             ███░░░░░░░░░░░░░░░░░░░░░░   11.23 % 
JavaScript               35 mins             ███░░░░░░░░░░░░░░░░░░░░░░   10.93 % 
TypeScript               34 mins             ███░░░░░░░░░░░░░░░░░░░░░░   10.49 % 

🔥 Editors: 
IntelliJ IDEA            5 hrs 1 min         ███████████████████████░░   92.01 % 
Neovim                   26 mins             ██░░░░░░░░░░░░░░░░░░░░░░░   07.99 % 

💻 Operating System: 
Mac                      5 hrs 27 mins       █████████████████████████   100.00 % 
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


 Last Updated on 29/11/2025 18:48:24 UTC
<!--END_SECTION:waka-->

</details>

##
