# Email Reaper CLI tool
CLI tool for grabbing websites from google search results like https://bit.ly/3QetS9S and visiting those links and getting email addresses from there.

### How to use
- Install Rust on your machine
- Clone the repo
- Install chromedriver on your machine
- Run the application like `reaper --url https://bit.ly/3QetS9S --start 1 --end 2`

### Additional args
- `csv` - specify a path to .csv file you want to save found emails. Data will be saved as website emails where found at and up to 3 emails. Exmaple: `website address,email1,email2,email3`
- `json` - specify a path to .json file you want to save found emails. Data wil be saved as an array of json objects that contain website emails where found at and an array of emails.

### Example
emails.csv 
```csv
http://www.archibald-co.be/,info@archibald-co.be,,
http://conixrdbm.com/,info@conixrdbm.com,,
http://www.dds.plus/,mail@dds.plus,,
https://polo-architects.be/,info@polo-platform.eu,hr@polo-platform.eu,press@polo-platform.eu
http://d44.be/,,,
http://www.epikur-architecture.com/,605a7baede844d278b89dc95ae0a9123@sentry-next.wixpress.com,8eb368c655b84e029ed79ad7a5c1718e@sentry.wixpress.com,
https://twi.archi/,info@twi.archi,info@architecture.com,
http://www.vincentvanduysen.com/,,,
https://www.abscis-architecten.be/nl,logo-abscis@2x.png,icon-fb@2x.png,icon-twitter@2x.png
http://www.lucasfreire.be/,info@lucasfreire.be,,
http://www.noaarchitecten.net/,studio@noAarchitecten.net,,
https://frame-architects.com/,info@frame-architects.com,,
http://www.assar.com/,luxembourg@assar.com,info@email.com,assar.logo_.full@2x.png
http://architects.bc-as.org/,,,
http://www.jaspers-eyers.be/,architects@jaspers-eyers.be,,
http://www.modulo-architects.be/,,,
http://www.oyo.eu/,jobs@oyo.eu,press@oyo.eu,business@oyo.eu
http://www.xdga.be/,info@xdga.be,jobs@xdga.be,
http://www.b2ai.com/,info@B2Ai.com,,
```

emails.json
```json
[{"link":"http://www.archibald-co.be/","emails":["info@archibald-co.be"]},{"link":"http://conixrdbm.com/","emails":["info@conixrdbm.com"]},{"link":"http://www.dds.plus/","emails":["mail@dds.plus"]},{"link":"https://polo-architects.be/","emails":["info@polo-platform.eu","hr@polo-platform.eu","press@polo-platform.eu"]},{"link":"http://d44.be/","emails":[]},{"link":"http://www.epikur-architecture.com/","emails":["605a7baede844d278b89dc95ae0a9123@sentry-next.wixpress.com","8eb368c655b84e029ed79ad7a5c1718e@sentry.wixpress.com"]},{"link":"https://twi.archi/","emails":["info@twi.archi","info@architecture.com"]},{"link":"http://www.vincentvanduysen.com/","emails":[]},{"link":"https://www.abscis-architecten.be/nl","emails":["logo-abscis@2x.png","icon-fb@2x.png","icon-twitter@2x.png","icon-linkedin@2x.png","info@abscis.be","icon-instagram@2x.png"]},{"link":"http://www.lucasfreire.be/","emails":["info@lucasfreire.be"]},{"link":"http://www.noaarchitecten.net/","emails":["studio@noAarchitecten.net"]},{"link":"https://frame-architects.com/","emails":["info@frame-architects.com"]},{"link":"http://www.assar.com/","emails":["luxembourg@assar.com","info@email.com","assar.logo_.full@2x.png","paris@assar.com","liege@assar.com","brussels@assar.com","antwerp@assar.com"]},{"link":"http://architects.bc-as.org/","emails":[]},{"link":"http://www.jaspers-eyers.be/","emails":["architects@jaspers-eyers.be"]},{"link":"http://www.modulo-architects.be/","emails":[]},{"link":"http://www.oyo.eu/","emails":["jobs@oyo.eu","press@oyo.eu","business@oyo.eu","info@oyo.eu","gandert@oyo.eu","oyobcn@oyo.eu"]},{"link":"http://www.xdga.be/","emails":["info@xdga.be","jobs@xdga.be"]},{"link":"http://www.b2ai.com/","emails":["info@B2Ai.com"]}]
```
