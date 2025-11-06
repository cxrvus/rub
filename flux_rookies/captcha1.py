import requests
import re

s = requests.Session()

r = s.get('https://rkchals.fluxfingers.net/code/captcha.php')
match = re.search(r"\d+ \+ \d+", r.text)

print(match.group(0)[0])
print(r.text)

