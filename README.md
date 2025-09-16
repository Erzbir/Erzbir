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
![Code Time](http://img.shields.io/badge/Code%20Time-1%2C523%20hrs%2057%20mins-blue)

![Profile Views](http://img.shields.io/badge/Profile%20Views-0-blue)

**🐱 My GitHub Data** 

> 📦 389.2 kB Used in GitHub's Storage 
 > 
> 🚫 Not Opted to Hire
 > 
> 📜 32 Public Repositories 
 > 
> 🔑 14 Private Repositories 
 > 
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


 Last Updated on 16/09/2025 18:47:17 UTC
<!--END_SECTION:waka-->

</details>

##
