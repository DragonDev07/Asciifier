# Asciifier
[![wakatime](https://wakatime.com/badge/github/DragonDev07/Asciifier.svg)](https://wakatime.com/badge/github/DragonDev07/Asciifier)

## What & Why
- Similar to [Figlet](https://github.com/cmatsuoka/figlet) but written in rust & faster (see tests below)
### Speed
- Speed Comparison Using [`hyperfine`](https://github.com/sharkdp/hyperfine)
- 100,000 Runs
- Warmup of 5
![image](https://github.com/user-attachments/assets/52067491-a884-4d41-afb2-44d3bc564db7)
![image](https://github.com/user-attachments/assets/eb84ac2d-b3d1-4b00-95f8-1fc4e2817ac3)
![image](https://github.com/user-attachments/assets/95c1b31e-8b17-43e0-ad7e-b5c4eaf35260)

## Usage

## Building From Source
**1. Clone the Repository & Enter Directory**
  ```bash
    git clone https://github.com/DragonDev07/Asciifier
    cd Asciifier
  ```

**2. Build Using Cargo**
  ```bash
    cargo build --release
  ```

**3. Finished!**
- Binary is located at `target/release/asciifier`

## Roadmap
- [ ] Fix new lines (and string that are too long becoming spaghetti)
- [ ] Add more fonts
