### Simple script to remove subdomains from urls written in Rust.

1. Processes all `txt` files in directory of execution.
2. Script creates folder `result` in current folder.
3. Each changed file has postfix new in its name

Sample data:
```
...
sample.url.com
url.net
...
```

Result:
```
...
url.com
url.net
...
```
