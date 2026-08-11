# Changelog

## 0.2.2

### Discovery
- Reaching the end of a playlist, album, or artist queue no longer just stops playback -- it keeps going with related tracks, the same "keep listening" behavior SoundCloud's own app has. Your original queue is left intact with more tacked onto the end, and it keeps extending for as long as there's more to find. Loop still takes priority when it's on, and a queue that turns out to be entirely unplayable still tells you so instead of silently giving up.

### Track detail page
- Redesigned to match the rest of the app: bigger artwork with a play/pause overlay, an artist avatar and follow button next to the title, an Add to playlist action, and a pill-style comment composer with your avatar.
- Adds "Related tracks" and "More by {artist}" shelves so there's always something to jump to instead of a mostly-empty page.

### Likes
- New tile view alongside the existing list, matching the tiles/rows toggle Search and Playlists already have.

### Fixes
- Adjusting the volume slider (main window or mini player) could silently pin the displayed playback position at whatever was last saved, freezing the progress bar until the next track change, even though playback itself kept going normally underneath.
- Pressing Space to pause right after adjusting the volume slider stopped working, since focus was still on the slider and it was being treated as a text-entry field.
- Tracks shown as tiles (Likes' new tile view, and the detail page's new shelves) weren't right-clickable -- they were missing the same context menu (Like, Follow, Add to playlist, Play next, Copy link, etc.) the row view already has.

## 0.2.1

### Navigation & search
- Navbar collapses to icons (Home/Likes/Playlists) with tooltips; the "SoundKitten" wordmark is gone.
- Search is now a click-to-open, Spotlight-style overlay: centered floating panel over a blurred/dimmed backdrop, with live debounced results across tracks, artists, and playlists. Performance Mode skips the blur/scale-in animation.

### Playlists
- Full create, rename, delete, add-track, and remove-track support, all via official OAuth (playlists were read-only before this).
- New "+ New playlist" entry point on the Playlists screen, a real playlist detail view with owner-only controls, and an Add-to-playlist picker (with inline "create new") reachable from any track.

### Track context menu
- Right-click (or the new "..." button on every row, which works even when right-click doesn't cooperate with a particular setup) now has Like/Unlike, Follow/Unfollow artist, and Add to playlist alongside the existing Play now/Play next/Add to queue/View track/Go to artist/Copy link.
- Only one menu can be open at a time, and it opens with a short entrance animation matching the rest of the app.

### Feed
- Feed items now show who reposted them and when.
- A "..." button on Home's Feed card opens a full, focused Feed view: original SoundKitten post cards with a large artwork, in-place play/pause, and inline Like, Comment, Repost, Add-to-playlist, and Share, all without leaving the feed. Comments load lazily per card, and there's an always-visible "Write a comment..." field.
- Cards lay out in a responsive grid that fills the available width, sized uniformly.

### Fixes
- WebView2 was showing its own native right-click menu (Back/Forward/Reload/Print) instead of the app's -- fixed by hooking the correct native API instead of the DOM `contextmenu` event, which WebView2 doesn't reliably honor here.
- Launching SoundKitten while it's already running now just focuses the existing window instead of opening a second (or third) instance.
