# Experimental Fcitx5 installation

Build prerequisites are listed in the [development guide](development.md#build-and-check-the-fcitx5-experiment).
Run commands from the repository root. Staged installation has been checked;
actual installed-route desktop typing is still awaiting user confirmation.

## Install the experimental addon

This procedure targets the recorded CachyOS x86_64 environment with Fcitx5 under
`/usr`. The name is provisional; this is a local source installation, with the
[existing desktop limitations](v1.md#remaining-desktop-smoke-checklist).
Keep your already configured Fcitx5/frontend setup. Installation adds the addon
files; activation remains a separate manual desktop action.

From the repository root, prepare and check a Release build:

```sh
cmake -S adapters/fcitx5 -B target/fcitx5-install -G Ninja \
  -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/usr \
  -DFCITX_INSTALL_USE_FCITX_SYS_PATHS=ON
cmake --build target/fcitx5-install
ctest --test-dir target/fcitx5-install --output-on-failure
python3 tools/check-install.py target/fcitx5-install
```

The Python check requires Python 3. It stages installation in a fresh temporary
directory, checks the exact three-file layout, runs all ten dispatcher suites
against the staged copy, checks that a missing staged library fails, and removes
only Jong's files while retaining an unrelated sentinel. It does not contact a
desktop frontend or change your input-method settings.

The real installation writes exactly these files:

```text
/usr/lib/fcitx5/libjong-bangla.so
/usr/share/fcitx5/addon/jong-bangla.conf
/usr/share/fcitx5/inputmethod/jong-bangla.conf
```

Before installing, check for existing files at those paths, including symlinks.
If present, inspect ownership with `pacman -Qo /absolute/path`; use the owning
package's update/removal procedure or retain a backup of an earlier manual
installation. Do not overwrite an unrelated or package-managed file.

**Run this yourself when ready to install:**

```sh
sudo cmake --install target/fcitx5-install
```

Use the prefix configured above: the Fcitx5 destination variables are absolute,
so `cmake --install --prefix` does not relocate them. To change the layout,
reconfigure in another build directory and repeat the staging check. A staged
custom-prefix pass does not establish discovery by your normal desktop daemon.
Only the shared addon and its two registrations are installed; Rust/Cargo,
private headers and test executables are not runtime installation artifacts.

After installing, restart Fcitx5 through your existing desktop/session setup;
logging out and back in is an alternative. An already running daemon keeps the
old addon until restarted. Open Fcitx5's configuration tool, add **Jong Bangla**
(disable the current-language filter if needed), and select it. In a disposable
text field, type `ki` then Space: expect `কি `. Keep the current desktop route;
this installation does not establish support for additional Wayland routes.

For a later source update, rebuild, rerun CTest and the staging check, then repeat
the installation and restart. CMake does not supply a general uninstall target.
For removal of this manual `/usr` installation, first switch away from Jong
Bangla and remove its entry in Fcitx5's configuration tool. Then run:

```sh
sudo rm -- /usr/lib/fcitx5/libjong-bangla.so \
  /usr/share/fcitx5/addon/jong-bangla.conf \
  /usr/share/fcitx5/inputmethod/jong-bangla.conf
```

Restart Fcitx5 through your existing session setup afterward. Restore any prior
manual-install backups as appropriate. Keep shared directories, other engines,
toolkit integrations and the rest of your Fcitx5 configuration.
