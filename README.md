# Steam Workshop Downloader

This app is a simple application that allows you to download Steam workshop files from their mod urls. It was initially developed for use with Rimworld, but has been modified to allow for any app id. By default this uses the anonymous login with steamcmd to download, but you can add your account info as a parameter if you want to download using your own account.

## Features

- Download a workshop mod using steamcmd using the mods url
- Download multiple at once
- Mod folders are automatically renamed after downloading for better readability

## Usage

_**Note**: Downloading from Steam Workshop should work with most games that support anonymous login. The renaming functionality will not work with workshop items that are not for RimWorld._

1. Take all the urls for the mods you want to download and put them in the folder `data\urls.txt`
2. Run the program with `download` as a command line argument
3. Go to the Steamcmd content directory `SteamCMD\steamapps\workshop\content\<app-id>`
4. Move all the folders to `data\mods\`
5. Now run the program with the argument `rename`
6. Move the renamed folders into your `Rimworld\mods` directory

After step 3, you can just move the folders into your `Rimworld\mods` directory without renaming them. I just added that to make things a bit easier if you want to change your modlist later.

## Issues I ran into

1. Sometimes the download might fail, just go into the steamcmd dir and delete your steamapps folder _(make sure you move any mods out first)_. After you delete the folder try again.
2. If any folder fails to rename just turn off the read only flag on the data folder, include all subdirectories.

### TODO

- Implement using a different account other than the default "login anonymous"
- Rename files directly in the steamcmd directory, instead of having to move them over
