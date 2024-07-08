Goals:

- Use zed-industries gpui framework to create a wrapper around Apple's MusicKit.js library
- Use deno_core to roll a barebones javascript runtime and use as little javascript as possible
- Make the the best desktop client for Apple Music 

Blockers:

- MusicKit uses DOM apis provided by browsers like Document event listeners, Window, etc.
- I have not been able to get linkeDOM, jsdom, or deno_dom to adequately stand-in in lieu of a browser

