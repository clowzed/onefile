# Onefile server
Server for uploading and opening one file
### Technologies
- rust
- actix-web
- docker

# Installation
With docker compose
```bash
git clone https://github.com/clowzed/onefile
cd onefile
docker compose up -d
```

Or provide `.env` file with the following options:
```bash
PORT
UPLOAD_FOLDER
```

Usage:
- Make post request with curl

```bash
curl -F 'file=@/pat/to/my/report.html' http(s)://(<server domain> or <localhost> or <ip>)/upload
```
This will return a key. Open it in your browser with following link
```
http(s)://(<server domain> or <localhost> or <ip>)/get/<key>
```

I have own server running on http://onefile.clowzed.ru

So you can use my cli tool to open and upload files from the server. 
Install it from [this repository](https://github.com/clowzed/ofcli)

```
ofcli upload ./myreport.html
ofcli open 2332-2323-232323-32232
```

I do not take any responsibility about file storing time!


