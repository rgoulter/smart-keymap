# Pre-baked development / CI images

Two ways to get a tool-complete environment (Rust → `riscv32imac-unknown-none-elf`,
xPack `riscv-none-elf-gcc`, Nickel, cbindgen, just, cmake):

1. **Dockerfile (Codespaces / Distrobox default)** — root `Dockerfile`, referenced by
   `.devcontainer.json`. Also includes Ceedling. Build with Docker or Podman; no Nix required.
2. **Nix OCI (preferred when publishing)** — `containers/flake.nix`:
   - `nix build ./containers#image-firmware-core` — firmware tools only (~2.4 GB measured)
   - `nix build ./containers#image-firmware-core-ceedling` — + Ceedling (~2.85 GB)

Once an image is on a registry (e.g. GHCR), point `.devcontainer.json` and GitHub Actions
`container:` at the same tag for local/CI parity. Until then, Codespaces builds the Dockerfile.

Do not use the old Cachix devenv base image + direnv as the Codespaces entrypoint (slow first open).
