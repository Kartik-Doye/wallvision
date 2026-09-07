# WallVision

WallVision is a camera-free WiFi sensing platform. It reads Channel State
Information (CSI) and related radio measurements, converts signal changes into
privacy-bounded spatial events, and exposes those events to edge applications,
automation systems, and research tools.

The short version is:

> WallVision turns changes in ordinary radio signals into local information
> about presence, motion, activity, and room state without using a camera.

The repository is a multi-runtime system. Rust is the maintained production
path. Python remains useful for reference proofs and selected research tools.
ESP32 firmware captures CSI at the edge, while the static UI and dashboard
visualize results.

## What this project does

Radio signals move through a room and reflect from walls, furniture, and people.
When a person moves or breathes, the received signal changes. CSI records the
complex response of a wireless link across subcarriers and antennas. WallVision
processes those changes through the following stages:

1. A sensor or replay source produces an RF frame.
2. A hardware adapter validates the frame and converts it to the internal
   representation.
3. Signal processing removes or models nuisance effects such as phase offsets,
   noise, packet loss, and drift.
4. Feature and neural-network components infer bounded properties such as
   presence, motion, activity, or vital-sign estimates.
5. Fusion combines compatible observations while preserving uncertainty,
   provenance, and policy information.
6. The server, Homecore integrations, UI, or downstream automation consumes
   the resulting event.

This is sensing, not optical vision. It does not produce a camera image. It
cannot see colors, faces, text, or arbitrary visual detail.

## Important evidence boundary

Not every capability in this repository has the same evidence level. Read the
labels before using results in a product or publication:

- **MEASURED** means the repository contains a reproducible measurement and
  identifies the benchmark, split, hardware, or command used.
- **CLAIMED** means a documented target or external claim that still needs the
  stated reproduction conditions.
- **SYNTHETIC** means generated data or a simulation. It is useful for testing
  algorithms but is not evidence of real-world sensing accuracy.
- **ARCHIVED** means retained for historical or research reference and not a
  supported production path.

WiFi sensing is not camera-grade perception. Vital-sign, pose, identity, and
through-wall results depend on hardware, placement, calibration, environment,
training data, and the evaluation split. Do not deploy a result as medical,
security, or safety-critical evidence without independent validation.

## Repository map

```text
.
|-- v2/                         Maintained Rust workspace and production crates
|   |-- crates/                 Signal, RF, server, Homecore, evidence, and UI-facing crates
|   |-- Cargo.toml              Workspace members and shared dependency policy
|   `-- Cargo.lock              Reproducible Rust dependency resolution
|-- firmware/esp32-csi-node/    ESP32-S3/C6 CSI capture firmware and provisioning
|-- archive/v1/                 ARCHIVED Python reference implementation and proof
|-- requirements.txt            Python runtime/research dependencies
|-- requirements-dev.txt        Python test, lint, and development dependencies
|-- pyproject.toml              Python package metadata and canonical dependency extras
|-- ui/                         Static browser visualizations and observatory pages
|-- dashboard/                  Vite/Lit TypeScript dashboard application
|-- scripts/                    Capture, replay, calibration, training, and validation tools
|-- examples/                   Small integrations and demonstrations
|-- docs/                       User guides, design notes, ADRs, and evidence reports
|-- tests/                      Repository-level tests and invariant checks
|-- verify                     Deterministic repository proof entry point
|-- Makefile                    Common install, build, test, and run commands
|-- docker/                     Container definitions and local service composition
`-- plans/                      Active implementation plans
```

### The maintained Rust workspace: `v2/`

The `v2/` workspace is the center of the project. Each crate owns one bounded
responsibility instead of putting capture, inference, storage, and policy in a
single module.

Important crate families include:

- `wifi-densepose-core`: shared frame, tensor, and domain primitives.
- `wifi-densepose-signal`: filtering, phase processing, FFTs, feature
  extraction, and signal-quality operations.
- `wifi-densepose-nn`: neural-network inference and model-facing logic.
- `wifi-densepose-hardware`: hardware adapters and device-facing boundaries.
- `wifi-densepose-wifiscan`: WiFi scan and CSI-oriented acquisition support.
- `wifi-densepose-vitals`: breathing and heart-rate signal features.
- `wifi-densepose-sensing-server`: the Axum HTTP/WebSocket sensing server and
  runtime composition layer.
- `wifi-densepose-cli`: command-line entry points for supported operations.
- `wifi-densepose-wasm` and related crates: browser and edge compilation paths.
- `wifi-densepose-calibration` and `wifi-densepose-train`: room calibration and
  model workflow support.
- `wallvision-ontology`, `wallvision-evidence`, `wallvision-policy`, and
  `wallvision-witness`: typed spatial concepts, evidence handling, authorization,
  and provenance.
- `wallvision-unified`: the unified RF world-model experiments and adapters.
- `homecore-*`: local state, automation, recording, migration, plugins, and
  smart-home APIs.
- `wifi-densepose-privshield`: compliant waveform countermeasure research.

The authoritative list is always `v2/Cargo.toml`. The list above is a guide,
not a promise that every crate has the same maturity or hardware support.

### Firmware: `firmware/esp32-csi-node/`

The firmware runs on supported ESP32 targets and performs the first edge step:
capturing and forwarding CSI. `main/` contains application code, `components/`
contains reusable firmware components, `CMakeLists.txt` defines the ESP-IDF
build, and `provision.py` handles device provisioning. Target-specific
configuration lives in the `sdkconfig*` files.

Firmware build success is not hardware validation. A hardware claim requires a
real device, the exact target and firmware configuration, and a captured boot
or runtime log.

### Python: `archive/v1/`, `requirements.txt`, and `pyproject.toml`

`archive/v1/` is the original pure-Python implementation. It is deprecated and
does not receive normal production fixes. Its deterministic proof remains
valuable because it gives a small, inspectable reference for selected signal
invariants. Its historical pose head has no committed trained checkpoint and
must not be presented as a production model.

The root Python metadata describes the current package and development extras.
`requirements.txt` is the convenient runtime/research installation file.
`requirements-dev.txt` adds tests and tooling. Keep these files aligned when
Python dependencies change.

### Browser applications: `ui/` and `dashboard/`

`ui/` is a static visualization surface that can be served directly. It contains
the observatory, pose-fusion, mobile, service, and visualization assets.

`dashboard/` is a separate Vite application written in TypeScript and Lit. Its
`package.json` owns its JavaScript dependencies and scripts. Python or Cargo
requirements do not install the dashboard.

## System architecture

```text
ESP32-S3/C6, research NIC, or replay file
                    |
                    v
          hardware adapter boundary
                    |
                    v
       validated native RF frame / tensor
                    |
                    v
      calibration + signal processing
                    |
                    v
       features, models, and estimators
                    |
                    v
      uncertainty-aware fusion and policy
                    |
                    v
 bounded event, evidence, and provenance record
                    |
          +---------+----------+
          |                    |
          v                    v
   HTTP/WebSocket server   UI/dashboard/automation
```

### Why the boundaries matter

RF input is untrusted and varies by chipset, firmware, antenna geometry, room,
and deployment. Adapters validate shape, metadata, provenance, and quality
before downstream code can interpret a frame. Signal processing should not
quietly turn missing or invalid samples into confident measurements. Models
should return uncertainty and evidence metadata rather than an unexplained
number. Policy and privacy checks belong at the event boundary, before data is
exported to an integration.

The same design applies to simulated data. A simulator can exercise the code
path, but it must remain marked synthetic and cannot silently become a measured
claim.

## Requirements

Choose only the toolchain you need.

### Minimum common tools

- Git
- Python 3.9 or newer for the Python package and scripts
- Rust toolchain defined by `v2/rust-toolchain.toml` for the production path
- Node.js and npm for `dashboard/`
- Docker Desktop if using the container workflow

### Python requirements

Create an isolated environment and install the runtime set:

```bash
python -m venv .venv

# macOS/Linux
source .venv/bin/activate

# Windows PowerShell
.venv\\Scripts\\Activate.ps1

python -m pip install --upgrade pip
python -m pip install -r requirements.txt
```

For tests and formatting tools:

```bash
python -m pip install -r requirements-dev.txt
```

The Python list includes API, database, numerical, ML, networking, hardware,
validation, and monitoring packages. PostgreSQL, Redis, and Celery are optional
services; installing their Python clients does not start those services.

### Rust requirements

Install Rust with `rustup`, then let the repository toolchain select the
expected compiler:

```bash
cd v2
cargo test --workspace --no-default-features
cargo build --release
```

Some optional crates need additional native libraries or targets. Build the
focused crate first when working on a feature instead of assuming the entire
workspace has identical platform requirements.

### Dashboard requirements

```bash
cd dashboard
npm ci
npm run typecheck
npm test
npm run build
```

### Firmware requirements

Use the ESP-IDF version documented by
`firmware/esp32-csi-node/README.md`. Confirm the exact target before building or
flashing:

```bash
cd firmware/esp32-csi-node
idf.py set-target esp32s3
idf.py build
```

The C6 target is a separate hardware path. Do not flash a binary built for the
wrong target. Never put WiFi credentials in commands, logs, issues, or commits.

## Quick start

### 1. Run the deterministic proof

This is the fastest no-hardware check:

```bash
python archive/v1/data/proof/verify.py
```

The expected result is `VERDICT: PASS`. This validates the governed reference
proof; it does not validate real-room accuracy.

The repository wrapper also supports:

```bash
./verify
```

On Windows, use Git Bash or run the underlying Python command directly.

### 2. Test the Rust production workspace

```bash
cd v2
cargo test --workspace --no-default-features
```

For a narrow iteration, use the owning crate:

```bash
cargo test -p wifi-densepose-signal
cargo test -p wifi-densepose-sensing-server
```

### 3. Start the legacy/reference API only when needed

The Makefile retains the historical command:

```bash
make run-api-dev
```

This launches the archived Python API with reload enabled. It is not the
recommended production server. Prefer the Rust sensing server for maintained
runtime work.

### 4. View the static UI

```bash
python -m http.server 3000 --directory ui
```

Open `http://localhost:3000`. The page is a visualization client; it needs a
compatible data source for live sensing.

### 5. Run the dashboard

```bash
cd dashboard
npm ci
npm run dev
```

Vite prints the local URL. Use `npm run test:e2e` when browser automation is
available.

## Common workflows

### Capture and replay

Capture tools live in `scripts/` and include CSI UDP recording, relay, ESP32
conversion, calibration, and replay helpers. Before using one:

1. Read its usage and input assumptions.
2. Confirm the output path is not tracked or sensitive.
3. Apply the repository data policy in `scripts/csi-data-policy-check.sh`.
4. Record whether the result is real capture, replay, or synthetic data.

Raw CSI and person data are sensitive. Do not commit captures, room profiles,
private indexes, transcripts, credentials, or generated artifacts.

### Model and training work

Training and evaluation helpers are distributed between `v2/crates/`,
`scripts/`, `examples/`, and `docs/`. Start with the nearest ADR and the model
card or evaluation report. A training script completing successfully proves
that the script ran; it does not prove model quality. Use a leakage-free
held-out split and preserve the baseline, dataset identity, and hardware
conditions.

### Home automation

Home Assistant, Matter, Apple Home, and Homecore integration documentation is
under `docs/` and the corresponding Rust crates are under `v2/crates/`. Keep
identity, raw RF, and inferred event permissions separate. Integration support
must not be interpreted as a claim that the underlying inference is clinically
or security validated.

## Command reference

The Makefile is a convenient index:

| Command | Purpose |
| --- | --- |
| `make check` | Check local prerequisites without installing everything |
| `make install-python` | Install the Python profile through `install.sh` |
| `make install-rust` | Install the Rust profile through `install.sh` |
| `make install-full` | Install all supported local profiles |
| `make verify` | Run the deterministic proof wrapper |
| `make verify-audit` | Run proof plus repository audit checks |
| `make build-rust` | Build the Rust workspace in release mode |
| `make test-rust` | Run Rust workspace tests without default features |
| `make build-wasm` | Build the browser WASM package |
| `make run-api-dev` | Run the archived Python API with reload |
| `make run-viz` | Serve the static `ui/` directory |
| `make run-docker` | Start the Docker Compose development stack |
| `make help` | Print the current target list |

The Makefile is authoritative for target names; command availability depends on
the shell and operating system.

## Testing and verification strategy

Use the smallest deterministic check that exercises the change, then expand:

1. Python change: run the affected Python test or the deterministic proof.
2. Rust change: run `cargo test -p <crate>` and then the workspace check when
   the contract is shared.
3. Dashboard change: run typecheck, unit tests, build, and relevant Playwright
   tests.
4. Firmware change: build for the exact target, run firmware tests, then obtain
   a real boot/runtime log for hardware claims.
5. Security or boundary change: run the relevant security tests and review the
   diff for new permissions, exports, paths, or secret handling.

Do not use a passing build as a substitute for real hardware validation. Do not
use a simulator result as a real-world accuracy number.

## Privacy and security

WallVision can process data about people and occupied spaces. Deploy it with
consent, clear retention rules, access control, and local disclosure. Treat RF
captures, room profiles, identity-related features, logs, and calibration data
as potentially sensitive.

Contributors must:

- validate paths, process arguments, network inputs, hardware inputs, and FFI
  boundaries;
- use least authority and default-deny behavior for integrations;
- avoid committing secrets, `.env` files, raw captures, private indexes, or
  agent transcripts;
- preserve provenance and uncertainty when adding an inference path; and
- never claim camera-grade, medical, or safety-critical accuracy without the
  required evidence.

## Documentation guide

Use these locations to continue learning:

- `docs/user-guide.md`: user-facing operations and deployment concepts.
- `docs/TROUBLESHOOTING.md`: common setup and runtime problems.
- `docs/adr/`: accepted architecture decisions and their evidence boundaries.
- `docs/security/`: security design and audit material.
- `firmware/esp32-csi-node/README.md`: firmware-specific build and hardware
  instructions.
- `ui/README.md`: static UI details.
- `CHANGELOG.md`: chronological project changes.
- `PROOF.md`: repository proof and verification context.

When documentation conflicts with source, tests, workflows, or an accepted ADR,
prefer the source-backed behavior and update the documentation as part of the
same change.

## Contributing

1. Read `AGENTS.md` and `CLAUDE.md` before making changes.
2. Inspect the nearest source, tests, workflow, and ADR.
3. Keep the change small and preserve unrelated work in a dirty tree.
4. Add or update focused tests and evidence labels.
5. Run the applicable validation commands.
6. Review the final diff for secrets, generated files, permission expansion,
   unsupported claims, and unrelated edits.

Pull requests should explain what changed, how it was tested, what evidence
level applies, and what remains unvalidated.

## License

The repository contains components with the licenses declared in their local
metadata. The primary project metadata declares MIT for the Python package and
MIT OR Apache-2.0 for the Rust workspace. Check each vendored or external
component before redistribution.
