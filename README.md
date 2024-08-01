# Metroid Prime Museum

This is a romhack for the Gamecube version(s) of Metroid Prime which purges the game world of *all* combat. This trivializes almost the entire game with the exception of the game's platforming and puzzle sections. There are a few reasons why such a hack may be of interest to the community:

- **Speedruning**: (i.e. "museum%") where the skill on display is pure undiluted movement
- **Content Creation**: Generate video of the game world without interruption from enemies
- **Practice/Labbing**: Explore the game world without having to clear enemies before each attempt
- **Romhacking**: The open source/license layout file provided as part of this repository makes for a good starting point for any romhack which necessitates starting from a "blank slate"

## "Features"

- Removed most enemies from the game. Exceptions were made for some ambient "enemies" such as Blast Caps and Tangle Weed
- Modified scripting to better match a combat-free game world
- Removed all instances of combat music
- Removed energy/missile count from the HUD. If the visor/helmet opacity are set to 0, there is effectively no HUD visible
- Removed many cutscenes, all others are skippable
- Player has infinite health
- Missile Stations refill Power Bomb ammunition to compensate for the lack of enemies
- Fast item acquisition text
- Added Spring Ball (C-Stick up with Morph Ball Bombs)

## How to Play (Windows)

1. Download the release .zip attached to the [latest release](https://github.com/toasterparty/metroid-prime-museum/releases)

2. Extract the contents into a folder

3. Double click `patch.bat` or `patch_all_items.bat` and select your Gamecube Metroid Prime ISO* when prompted. If successful, a file called `Metroid Prime Museum vX.Y.iso` will be created in the same folder. 

**Version NTSC 0-00 is recommended - but all GC versions would work.*

**As an alternative to a "vanilla" ISO, you may choose [Prime Practice Mod (Legacy)](https://practice.metroidprime.run/) as your input ISO.*

4. Drag and drop that ISO into Dolphin's main window, or load onto storage media and open with Nintendont

## Credits

|  |  |
-------|------------------
| **Romhack Author** | *toasterparty* |
| **Skippable Cutscenes** | *MrMiguel211* |
| **Spring Ball** | *UltiNaruto* |
