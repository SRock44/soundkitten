import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { ask } from "@tauri-apps/plugin-dialog";

export async function checkForUpdates() {
  let update;
  try {
    update = await check();
  } catch {
    return; // offline, endpoint unreachable, etc. -- fail silently
  }
  if (!update) return;

  const install = await ask(
    `SoundKitten ${update.version} is available. Install and restart now?`,
    { title: "Update available", kind: "info" },
  );
  if (!install) return;

  await update.downloadAndInstall();
  await relaunch();
}
