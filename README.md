buqi -- Clipboard Synchronization Tools

**Under contraction**
The `buqi` is designed to share clipboard between linux hosts.


## Design
 * The `buqi` has server and client mode.
 * The `buqi` does not touch Xorg or Wayland clipboard but only bind to
   a socket which accept read(get synced clipboard) and write(overwrite synced
   clipboard).
 * The `buqi-wayland` could hook to wayland composer and sync Wayland clipboard
   to buqi synced data.
 * The `buqi-xorg` could hook to xorg and sync xorg clipboard to buqi synced
   data.
