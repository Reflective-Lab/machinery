# Build Machine Stack — Home Lab Infrastructure

**Decision (2026-07-10):** No paid registry/hosting services. A dedicated build
machine (arriving ~August 2026, setup post-vacation) runs the self-hosted
factory infrastructure. This doc is the canonical plan; the Bedrock-side
registry migration specifics live in
`framework/bedrock/kb/07-decisions/consolidation/private-registry-self-hosted.md`.

## Trigger

The Bedrock v4.0.1 release needed 47 crates published; Shipyard's Free plan
caps at 5 crates (all consumed at 4.0.0). Rather than upgrade a subscription,
the registry moves in-house.

## The stack, bottom to top

**Technitium DNS → Caddy → Kellnr + Forgejo**

### 1. Technitium DNS Server — local DNS + DHCP

- Open source, single container, web UI.
- Owns BOTH local DNS zones and DHCP: fixed addresses are DHCP
  **reservations** (MAC → IP) managed in one place — never static configs
  sprinkled per device.
- Fallback considered: dnsmasq (minimalist, what OpenWrt runs).
  Pi-hole/AdGuard only if network-wide ad-blocking is wanted; local records
  are a side feature there.

### 2. Domain — split-horizon subdomain of an owned zone

- Use a subdomain of an already-owned zone (e.g. `lab.axioms.zone`),
  resolved only on the LAN.
- NOT `.local`/`.lan` (mDNS conflicts, no TLS story) and not `home.arpa`
  (self-signed certs would force CA plumbing into every cargo/git client).

### 3. Caddy — reverse proxy + TLS (Karl has run Caddy before)

- Wildcard Let's Encrypt cert via **DNS-01** challenge: valid TLS on
  `kellnr.lab.<zone>` / `forgejo.lab.<zone>` with nothing exposed to the
  internet, auto-renewed.
- Cargo sparse-registry HTTPS, Forgejo git-over-HTTPS, and browsers all work
  with zero client-side cert setup.

### 4. Kellnr — private Cargo registry (replaces Shipyard)

- Open source (AGPL), Rust, purpose-built; sparse index over plain HTTPS
  (no SSH-key index plumbing), token auth with stock `cargo publish`,
  no crate limits, web UI + rustdoc hosting.
- The `reflective-labs` registry name in Bedrock manifests is symbolic —
  migration is config-only:
  `index = "sparse+https://kellnr.lab.<zone>/api/v1/crates/"`.

### 5. Forgejo — self-hosted forge

- First tool alongside Kellnr. Its built-in Cargo registry was considered
  and passed over — Kellnr's purpose-built registry UX wins for a 47-crate
  publish set.

## First task once the stack is up

Re-run the parked Bedrock v4.0.1 publish (47 crates, topo-ordered):
`gh workflow run publish.yml` in `Reflective-Lab/bedrock-platform`, after
swapping the workflow's `SHIPYARD_TOKEN`/`SHIPYARD_SSH_KEY` secrets for a
single Kellnr token. Shipyard idles with 5 crates at 4.0.0; leave them.
