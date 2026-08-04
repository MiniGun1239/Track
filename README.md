# Track
Track planes progress

[![Time Spent](https://hackatime.hackclub.com/api/v1/badge/U0AJWQ44PGQ/MiniGun1239/Track)](https://hackatime.hackclub.com/my/projects/Track)
[![Track](https://img.shields.io/badge/GitHub-Track-green?style=plastic)](https://www.github.com/MiniGun1239/Track)

> Fun with chemistry and statistics

Fun chem app ("Fun" in very, very deep quotes, and very subjective)

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

Follow these simple steps to setup your local copy of track .

### Prerequisites

None (Unless you want to build it from source)

### How to get this for yourselves:

**Download it**

#### Linux
Get the latest release from [GitHub](https://github.com/MiniGun1239/Track/releases)

>**Note:** The name of the executable will be "track-*", where * is the version number, 
> remember to type the full name when executing like ```./track-* -V``` , or rename it from "track-*" to "track"

> If you downloaded, most likely it is in the downloads directory, 
> so either move it to the home directory (/home/user/) or run ```cd ~/Downloads``` before
> doing ```./track```

Or download from command line, like this:

```shell
curl -L https://github.com/MiniGun1239/Track/releases/download/Release/track-1.0.0-x86_64-Linux -o track
chmod +x track
```

> Always check what you are running, don't run random commands you find on the internet.

Done!, add to path to run anywhere or run from home like:
```shell
./track
```

#### Windows

Not Supported

### Video demonstration (TBA):  

* [Demonstration](https://youtu.be/m_t-4qjGvSE)
* [How to Download](https://youtu.be/d5zkCtXqlpM)

## Building from Source

1. **Pre-requisites:**  
   Need to install Python, and uv, or not, pip would also work

    ```shell
   sudo pacman -Syu python python-uv
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

4. **Done!:**  
   Now test the binary with: 
   ```shell
   ./track
   ```

## Contributors
*   **[![Me✨✨](https://img.shields.io/badge/GitHub-MiniGun1239-orange?style=plastic)](https://www.github.com/MiniGun1239)**
*   **TBA (no one else yet 🥹)**

> Coded and tested in Arch Linux, should work in any linux distro.


### Examples (tested on Aug 4th, at 12:30 GMT+4)

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
