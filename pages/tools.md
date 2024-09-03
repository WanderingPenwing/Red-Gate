# Yuya - Tools

###### nixos

## A beautiful declarative OS

I am using [NixOS](https://nixos.org/) on all 3 of my machines (my laptop and both of my servers).
That allows me multiple things :

- A reproducible architecture : 
  If I wipe my system I can recreate it perfectly using my config,
  and no package will stop working because of undeclared dependency
  
- My environnement everywhere : 
  My config is set up so that all my cli tools are installed on the 
  3 machines (with the same config), so I find myself at home 
  wherever I am. However while the 3 have a common config, they also
  each have their separate config, for example only my laptop has a 
  graphic environment, and only the bathhouse server has the [proxy](/tools#nginx) 
  config.

- A declarative system :
  I make a lot of mess when I debug an issue and I tend to touch 
  a lot of stuff to ty to fix my problem, so some obscure config
  could turn against me month later.
  But when your system is completely declared in the same place, 
  and that config represent exaclty the state of my computer, 
  everything is easier to clean.

###### docker

## A container System

> > to document

###### nginx
  
## A reverse Proxy

> > to document

###### cloudflare

## My domain (penwing.org) and Some Tunnels

> > to document

*on the bathhouse server*
```

services:
  cloudflared:
    image: cloudflare/cloudflared
    container_name: cloudflare-tunnel
    restart: unless-stopped
    command: tunnel run
    environment:
      - TUNNEL_TOKEN=your-token

```

###### portainer

## A dashboard ⌨ 

*local network only x)*

Being as new to docker container as I was (and still am), I wanted a pretty 
dashboard to motinor and tinker with the containers I run on the server. 
I chose [portainer](https://www.portainer.io/") for its beginner-friendliness, 
and it never failed me (yet).

This is the only thing installed on the 2 servers (so I can monitor them both). 
And they both call to the same ui (hosted on the bathhouse server)

**bathouse** config

```

services:
  portainer:
    image: portainer/portainer-ce:latest
    ports:
      - 9000:9000
    volumes:
      - ./data:/data
      - /var/run/docker.sock:/var/run/docker.sock
    restart: unless-stopped

```

**boiler** config (yeah it is a run instead of a compose, but I am lazy)

```

docker run -d   -p 9001:9001 \
				--name portainer_agent \
				--restart=always \
  				-v /var/run/docker.sock:/var/run/docker.sock \
  				-v /var/lib/docker/volumes:/var/lib/docker/volumes \  
  				portainer/agent:2.19.5

```

###### yuya

## This website

[www.penwing.org](https://www.penwing.org)

> > to document

###### pihole

## An adblocker / DNS Record 🛑

*local network only x)*

> > to document

###### searxng

## A search engine 🔍

[search.penwing.org](https://search.penwing.org)

I am not a big corporate fan *(as a linux user, surprising)*, so I was 
unhappy about relying on google for my searchs. Not because of its hunger for data 
but merely because I want to search for informations and not accept whatever google 
says the best result is. [SearXNG](https://github.com/searxng/searxng) 
is a **self-hostable meta search engine** (a bit of a mouthful). What it means 
in practice is that it will sort results according to *multiple sources* 
instead of just one (and you can choose the sources !)

on the **bathhouse** server
```

services:
  searxng:
    image: searxng/searxng
    container_name: searxng
    restart: unless-stopped
    ports:
      - "32768:8080"
    volumes:
      - ./settings:/etc/searxng:rw
    environment:
      - BASE_URL=https://search.penwing.org/
      - INSTANCE_NAME=penwing
      
```

###### forgejo

## Some git versioning 🗃 

[git.penwing.org](https://git.penwing.org)

> > to document

on the **bathhouse** server
```

networks:
  forgejo:
    external: false

services:
  server:
    image: codeberg.org/forgejo/forgejo:7
    container_name: forgejo
    environment:
      - USER_UID=1000
      - USER_GID=1000
      - FORGEJO__database__DB_TYPE=mysql
      - FORGEJO__database__HOST=db:3306
      - FORGEJO__database__NAME=forgejo
      - FORGEJO__database__USER=forgejo
      - FORGEJO__database__PASSWD=forgejo
    restart: always
    networks:
      - forgejo
    volumes:
      - ./forgejo:/data
      - /etc/timezone:/etc/timezone:ro
      - /etc/localtime:/etc/localtime:ro
    ports:
      - "3000:3000"
      - "4023:22"
    depends_on:
      - db

  db:
    image: mysql:8
    restart: always
    environment:
      - MYSQL_ROOT_PASSWORD=forgejo
      - MYSQL_USER=forgejo
      - MYSQL_PASSWORD=forgejo
      - MYSQL_DATABASE=forgejo
    networks:
      - forgejo
    volumes:
      - ./mysql:/var/lib/mysql

```

###### jellyfin

## Some movies 🎬

[movie.penwing.org](https://movie.penwing.org)

As a huge movie watcher, which I collect very legally, I had to make myself a 
collection. But why not share it with my friends ? So I use my server to host a 
[Jellyfin](https://jellyfin.org/) instance

on the **boiler** server

```

services:
  jellyfin:
    image: jellyfin/jellyfin
    container_name: jelly_compose
    environment:
      - VIRTUAL_HOST=movie.penwing.org
    ports:
      - "8096:8096"
    volumes:
      - jellyfin-config:/config
      - jellyfin-cache:/cache
      - /media/storage:/media
    restart: unless-stopped

volumes:
  jellyfin-config:
  jellyfin-cache:

```

###### stirling

## A pdf "edition" tool

[pdf.penwing.org](https://pdf.penwing.org)

Disclaimer : a pdf is compiled so it cannot be "edited" per say, only
scanned, and recompiled

You may know the pdf editor [I love PDF](https://www.ilovepdf.com/), 
the service I host ([Stirling](https://github.com/Stirling-Tools/Stirling-PDF)) is roughly the same, but with a bit more capabilities.
For example you can chain together different modifications like :

scan to pdf - merge pdf - page number - compress - lock 

Also I did not like to upload scan of *sensitive documents* to a random website.

on the **boiler** server

```

services:
  stirling-pdf:
    image: frooodle/s-pdf:latest
    restart: unless-stopped
    ports:
      - '1280:8080'
    volumes:
      - ./trainingData:/usr/share/tessdata #Required for extra OCR languages
      - ./extraConfigs:/configs
      - ./logs:/logs/
    environment:
      - DOCKER_ENABLE_SECURITY=false
      - INSTALL_BOOK_AND_ADVANCED_HTML_OPS=false
      - LANGS=en_GB

```

###### seafile

## A file manager 📁

[file.penwing.org](https://file.penwing.org)

While I use a lot scp (not the foundation, [the command](https://linux.die.net/man/1/scp)),
I like to have my own remote file drive. Following the move away from google as a 
search engine, I want to be free of google drive as well.

I chose [seafile](https://www.seafile.com/en/home/) and I am pretty happy with it. 
It is very lightweight with lot of optimizations, but to achieve this
seafile does not store the files (on the server) in a directory structure.

(not sure if I will keep it long since I do not use it that much since I become used to scp)

on the **boiler** server

```

services:
  db:
    image: mariadb:10.11
    container_name: seafile-mysql
    environment:
      - MYSQL_ROOT_PASSWORD=password
      - MYSQL_LOG_CONSOLE=true
      - MARIADB_AUTO_UPGRADE=1
    volumes:
      - ./mysql-data/db:/var/lib/mysql
    networks:
      - seafile-net
    restart: unless-stopped

  memcached:
    image: memcached:1.6.18
    container_name: seafile-memcached
    entrypoint: ["memcached", "-m", "256"]
    networks:
      - seafile-net
    restart: unless-stopped

  seafile:
    image: seafileltd/seafile-mc:10.0-latest
    container_name: seafile
    ports:
      - "7780:80"
#     - "443:443" #Uncomment if you are using HTTPS
    volumes:
      - ./seafile-data:/shared
    environment:
      - DB_HOST=db
      - DB_ROOT_PASSWD=password
      - TIME_ZONE=Europe/Paris
      - SEAFILE_SERVER_LETSENCRYPT=false
      - SEAFILE_SERVER_HOSTNAME=file.penwing.org
    depends_on:
      - db
      - memcached
    networks:
      - seafile-net
    restart: unless-stopped

networks:
  seafile-net:
    driver: bridge


```
