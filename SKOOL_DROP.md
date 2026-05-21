# Video Lab — Skool Drop Kit

Everything you need to ship `Video Lab` to your Skool community in one paste.

---

## 1. Announcement post (paste in Skool)

> **NEW DROP: Video Lab v0.1.0**
>
> Paste any video link. Get the file. No browser ads, no sketchy sites, no "convert to MP3" pop-ups.
>
> **What it does:**
> • Downloads from YouTube, Vimeo, TikTok, Twitter/X, Instagram, Twitch, and 1000+ other sites (anywhere yt-dlp works)
> • MP4 video up to 4K, or rip the audio as MP3
> • Playlist mode → pick which videos you want with checkboxes
> • Subtitles + thumbnails embedded automatically (optional toggles)
> • Sequential queue so you can fire off 20 downloads and walk away
>
> **Install:** download the `Video Lab_0.1.0_x64-setup.exe` below → double-click → next next finish → done. Nothing else needed (yt-dlp + ffmpeg are bundled inside).
>
> **First time you run it:** Windows SmartScreen might warn "Unrecognized app" because I haven't paid Microsoft $300/yr for a code-signing cert yet. Click **More info → Run anyway**. It's safe — I built this myself, in front of y'all.
>
> Made with vibe coding (Tauri + React + Rust). Ask me anything in the comments — I'll show how it's built in the next live.
>
> **Use it for personal stuff only. Respect creators. Don't redistribute downloads.**
>
> — Desi

---

## 2. Install guide (paste under the post or as a comment)

### Quick install

1. Click the `Video Lab_0.1.0_x64-setup.exe` file attached to this post
2. Save it somewhere (Downloads is fine)
3. Double-click the `.msi`
4. If Windows says **"Microsoft Defender SmartScreen prevented an unrecognized app from starting"** → click **More info** → **Run anyway**
5. The installer runs. Click through (default options are fine).
6. Find **DA Video Tool** in your Start menu or Desktop, open it.

### First download

1. Copy any YouTube / TikTok / Vimeo / X link to your clipboard
2. Click **paste** in the app (or Ctrl+V into the URL field)
3. Pick where you want the file saved (the app remembers next time)
4. Hit **Download**
5. When it's done, click **open file** to play it instantly

### Pro moves

- **Playlist mode:** paste a YouTube playlist URL → check off which videos you want → bulk download
- **Audio only:** switch format to `Audio MP3` → great for ripping music, podcasts, voiceovers
- **Subtitles toggle:** purple pill button — gets subs as a separate `.srt` file AND embedded in the MP4
- **Thumbnail toggle:** gold pill — embeds the poster image into the MP4

### Troubleshooting

| Problem | Fix |
|---|---|
| "SmartScreen prevented..." | Click **More info → Run anyway**. The app's not malware, just unsigned. |
| Download fails on a YouTube video | The bundled yt-dlp gets stale every few months. v0.4 will auto-update; for now, message me and I'll drop a new build. |
| Audio missing on playback | Make sure you're on v0.1.0+ — the codec fix is baked in. |
| Antivirus flags `yt-dlp.exe` | False positive (super common). Whitelist the app folder or check VirusTotal yourself. |

---

## 3. Where to find the installer

Hosted on **GitHub Releases** — Skool's free-tier file cap is ~30MB so we host externally and link from the post.

**Direct download (paste this into the Skool post as the install button link):**

```
https://github.com/jackdog668/da-video-tool/releases/download/v0.1.0/Video%20Lab_0.1.0_x64-setup.exe
```

**Release page (paste in the body so members can see what's new):**

```
https://github.com/jackdog668/da-video-tool/releases/tag/v0.1.0
```

### Verify your download (optional, for the paranoid)

After downloading, open PowerShell where the file is and run:

```powershell
Get-FileHash -Algorithm SHA256 .\"Video Lab_0.1.0_x64-setup.exe"
```

You should see:

```
224CDF7B141F45A67B80E612F235B51A255D259399AD8AF6C219AA22313A3153
```

If the hash matches, the file you downloaded is byte-for-byte the one I built. If it doesn't — don't run it, message me.

### Local build paths (for me, not members)

```
src-tauri/target/release/bundle/nsis/Video Lab_0.1.0_x64-setup.exe   ← shipped to Release (62MB)
src-tauri/target/release/bundle/msi/Video Lab_0.1.0_x64_en-US.msi    ← MSI alternative (86MB, optional)
```

---

## 4. v0.4 wishlist (drop later)

- [ ] Batch URLs (paste 10 different sites at once)
- [ ] yt-dlp self-update button
- [ ] Cookie support for sites needing login (Patreon, X private, etc.)
- [ ] Custom output filename templates
- [ ] Cancel/retry buttons per job
- [ ] Built-in trim / clip tool
- [ ] Code signing cert so SmartScreen shuts up

---

**For personal/educational use only.** Respect creators. Don't redistribute.
*beacons.ai/dbcreations · Digital Alchemy Academy*
