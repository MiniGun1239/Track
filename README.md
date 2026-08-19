# Track
Track planes progress

[![Time Spent](https://hackatime.hackclub.com/api/v1/badge/U0AJWQ44PGQ/MiniGun1239/Track)](https://hackatime.hackclub.com/my/projects/Track)
[![Track](https://img.shields.io/badge/GitHub-Track-green?style=plastic)](https://www.github.com/MiniGun1239/Track)

> Fun with airplanes

Track any plane you desire

---

## Features

*   **Track Flight Progress:** 
    * Enter callsign of a flying aircraft and its progress will be displayed in the terminal

## Stack

*   **Frontend:**   Terminal
*   **Backend:**    Python
*   **Database:**   json
*   **Styling:**    Vibez

---

## Getting Started

Follow these simple steps to set up your local copy of track.

### Prerequisites

- For main executable (Linux):
  - glib version >= 2.44
- For the other executable (Linux):
  - glib version >= 2.31

Check glib version by running:
```shell
$ ldd --version
```

>The main executable should work on any rolling release distro  
>The other one should work on any distro newer than ubuntu 20.04

### How to get this for yourselves:

**Download it**

Get the latest release from [GitHub](https://github.com/MiniGun1239/Track/releases)

>**Note:** The name of the executable will be "track-*", where * is the version number,
> remember to type the full name when executing like ```./track-* -V``` , or rename it from "track-*" to "track"

> If you downloaded, most likely it is in the downloads directory,
> so either move it to the home directory (/home/user/) or run ```cd ~/Downloads``` before
> doing ```./track```

Or download from command line, like this:

#### Linux

Regular version:
```shell
curl -L https://github.com/MiniGun1239/Track/releases/download/Release-1.2.0/track-1.2.0-x86_64-Linux -o track
chmod +x track
```

Legacy version:
```shell
curl -L https://github.com/MiniGun1239/Track/releases/download/Release-1.2.0/track-1.2.0-Legacy-x86_64-Linux -o track
chmod +x track
```

> Always check what you are running, don't run random commands you find on the internet.

Done!, add to path to run anywhere or run from home like:
```shell
./track
```

#### Windows

```shell
curl -L https://github.com/MiniGun1239/Track/releases/download/Release-1.2.0/track-1.2.0-x86_64-Windows.exe -o track.exe
```

> Always check what you are running, don't run random commands you find on the internet.

Done!, add to path to run anywhere or run from home like:
```shell
./track.exe
```

### Video demonstration (TBA):  

* [Demonstration](https://youtu.be/m_t-4qjGvSE)
* [How to Download](https://youtu.be/d5zkCtXqlpM)

## Building from Source

1. **Pre-requisites:**  
   - Need to install Python, no extra dependencies needed except for pyinstaller to compile
   - And uv, or not, pip would also work, but I used uv
   
   If installing uv:

   **Windows:**
```shell
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
```

   **Arch Linux:** 
```shell
sudo pacman -Syu python python-uv 
```
   
   **Other distros:**  
   - Option 1:
```shell  
pip install uv 
```
   
   - Option 2:
```shell 
curl -LsSf https://astral.sh/uv/install.sh | sh
```


2. **Clone the repository:**  
```shell
git clone https://github.com/MiniGun1239/Track.git
cd Track
```
   
3. **Build**:  
```shell
uv run pyinstaller --onefile src/main.py
```
   
   In docker of ubuntu 20.04 for older glib versions:
```shell
docker run --rm -v "$(pwd)":/app -w /app ubuntu:20.04 bash -c "
  apt-get update && apt-get install -y curl ca-certificates binutils && \
  curl -LsSf https://astral.sh/uv/install.sh | sh && \
  export PATH=\"/root/.local/bin:\$PATH\" && \
  rm -rf .venv && \
  uv sync --python 3.14 && \
  uv pip install --python 3.14 --force-reinstall pyinstaller && \
  uv run --python 3.14 pyinstaller --onefile src/main.py
"
mv dist/main track
chmod +x track
```

4. **Done!:**  
   Now test the binary with: 
```shell
.track
```

## Contributors
*   **[![Me✨✨](https://img.shields.io/badge/GitHub-MiniGun1239-orange?style=plastic)](https://www.github.com/MiniGun1239)**
*   **TBA (no one else yet 🥹)**

> Coded and tested in Arch Linux, should work in any linux distro.


## Examples (tested on Aug 4th, at 12:30 GMT+4)

```shell
./track
  Enter callsign: igo725
  Callsign: IGO725   | Altitude: 36000 | Speed: 465.8
  Type: A21N | Tail: VT-IMV | Progress: 33.98%
    [█████████████████                                 ]
  Coimbatore - VOCB (IN)                 Chennai - VOMM (IN)
```

```shell
./track
  Enter callsign: KQA886
  Callsign: KQA886   | Altitude: 36975 | Speed: 444.9
  Type: B788 | Tail: 5Y-KZD | Progress: 72.97%
    [████████████████████████████████████              ]
  Nairobi - HKJK (KE)                    Bangkok - VTBS (TH)
```

```shell
./track
Enter callsign: ups9
  Callsign: UPS9     | Altitude: 16875 | Speed: 467.4
  Type: B748 | Tail: N630UP | Progress: 97.55%
    [█████████████████████████████████████████████████ ]
  Shenzhen - ZGSZ (CN)                 Bangalore - VOBL (IN)
```

### Grievance

This project was meant to be in rust, but I couldn't get rust to work in time, so I switched to python, 
keeping the archived Rust files in rust_src and the [rust branch of this repo](https://github.com/MiniGun1239/Track/tree/Rust-Version) if anyone wants to take a look, and/or help.


### AI Usage

AI was used for debugging issues with glib version and to get the docker code to use ubuntu

