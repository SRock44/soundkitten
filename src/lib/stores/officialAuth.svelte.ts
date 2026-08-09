import { api } from "../api";

/** Tracks whether the second, optional official-OAuth login (used only to
 * enable real in-app like/unlike and follow/unfollow) is connected. Lazy:
 * nothing here runs until the user actually tries to like or follow
 * something, see LikeButton/FollowButton usage of ensureConnected(). */
class OfficialAuthStore {
  connected = $state(false);
  connecting = $state(false);

  async refresh() {
    this.connected = await api.isOfficialConnected();
  }

  /** Resolves true if already connected, or if this call successfully
   * completes the SoundCloud login. Resolves false if the user closes the
   * browser tab without finishing, or the login otherwise fails. */
  async ensureConnected(): Promise<boolean> {
    if (this.connected) return true;
    if (this.connecting) return false;
    this.connecting = true;
    try {
      await api.startOfficialLogin();
      this.connected = true;
      return true;
    } catch {
      this.connected = false;
      return false;
    } finally {
      this.connecting = false;
    }
  }

  async disconnect() {
    await api.disconnectOfficialLogin();
    this.connected = false;
  }
}

export const officialAuth = new OfficialAuthStore();
