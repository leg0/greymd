# Research: Configuration & Customization

## Decision: No Config File

- **Decision**: Remove config file entirely. Use well-known file paths and auto port selection.
- **Rationale**: A config file that accepts arbitrary file paths is a security vulnerability — an attacker could configure `css: /etc/shadow` and exfiltrate via browser. Fixed well-known paths (`~/.config/greymd/css`, `~/.config/greymd/js`) eliminate this attack vector entirely while still allowing customization.
- **Alternatives considered**: Config file with path validation (complex, error-prone), config file restricted to served directory (still allows reading unintended files within that tree)

## Decision: Auto Port Selection

- **Decision**: Try default port 8080, fall back to OS-assigned random port if busy
- **Rationale**: Eliminates the most common reason users needed configuration. `TcpListener::bind("127.0.0.1:0")` is a zero-dependency solution.
- **Alternatives considered**: Config file with port setting (removed for simplicity), sequential port scanning (unnecessarily complex)

## Decision: XDG Config Path

- **Decision**: `~/.config/greymd/` for custom files
- **Rationale**: Follows XDG Base Directory Specification conventions. Standard location for user configuration on Linux. On Windows, `USERPROFILE/.config/greymd/` is reasonable.
- **Alternatives considered**: `~/.greymd/` (dot-directory, less standard), `~/.local/share/greymd/` (for data, not config)

## Decision: Custom CSS Endpoint

- **Decision**: Serve at `/?css2`, keep `/?css` as built-in gzipped
- **Rationale**: Avoids gzip complexity. Built-in assets stay pre-compressed, custom assets served uncompressed.
- **Alternatives considered**: Append to `/?css` (requires runtime decompression)

## Decision: Custom JS Replacement

- **Decision**: Custom JS replaces built-in at `/?js`
- **Rationale**: JS customization typically means replacing the highlighter entirely, not stacking two JS bundles.
- **Alternatives considered**: Separate `/?js2` endpoint (user rejected)
