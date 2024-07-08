import { DOMParser } from "https://deno.land/x/deno_dom@v0.1.46/deno-dom-wasm.ts";
import * as MusicKit from "https://js-cdn.music.apple.com/musickit/v3/musickit.js";
Deno.core.print("Hello runjs!\n");

const document = new DOMParser().parseFromString(
  `<!DOCTYPE html>
  </html>`,
  "text/html",
);
document.addEventListener("musickitloaded", async function () {
  // Call configure() to configure an instance of MusicKit on the Web
  try {
    MusicKit.configure({
      developerToken: "DEV-TOKEN",
      app: {
        name: "My Cool App",
        build: "1",
      },
    });
  } catch (err) {
    Deno.core.print("Error running the MusicKit configure func: " + err + "\n");
  }
});
