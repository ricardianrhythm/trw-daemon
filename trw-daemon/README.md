# trw-daemon

A lightweight, open-source telemetry daemon for robotics fleets.

`trw-daemon` runs on every machine in your fleet, collecting system metrics and ROS2 introspection data. It exposes everything via a simple HTTP/JSON API that your control plane can poll.

## Why?

Robotics teams need fleet-wide observability but won't install closed-source binaries alongside safety-critical autonomy stacks. `trw-daemon` is fully auditable, minimal, and designed to stay out of the way of your real-time workloads.

## Quickstart

```bash
# Build from source
git clone https://github.com/therobotworks/trw-daemon.git
cd trw-daemon
cargo build --release

# Run with defaults (port 4040, 2s tick interval)
./target/release/trw-daemon

# Check it's working
curl http://localhost:4040/health
# → {"status":"ok"}

# Get the latest snapshot
curl http://localhost:4040/status
```

## Configuration

Copy and edit `config/default.toml`:

```toml
[daemon]
port = 4040
tick_interval_secs = 2
hostname = ""  # auto-detect

[collectors]
system = true
ros2 = true    # requires ROS2 sourced in environment
gpu = false

[ros2]
cli_path = "ros2"
timeout_secs = 5
```

Run with a custom config:

```bash
trw-daemon --config /etc/trw-daemon.toml
```

## API Reference

### `GET /status`

Returns the latest machine snapshot as JSON:

```json
{
  "hostname": "g1-orin-01",
  "timestamp": "2026-02-20T12:00:00Z",
  "system": {
    "cpu_percent": 42.3,
    "memory_percent": 67.1,
    "disk_percent": 55.0,
    "load_avg": [1.2, 0.9, 0.8],
    "uptime_secs": 86400
  },
  "ros2": {
    "nodes": ["/camera_driver", "/nav2_controller"],
    "topics": [
      {"name": "/cmd_vel", "type": "geometry_msgs/Twist", "hz": 10.2}
    ]
  },
  "processes": [
    {"pid": 1234, "name": "nav2_controller", "cpu_percent": 12.1, "mem_mb": 256}
  ]
}
```

### `GET /health`

Returns `{"status": "ok"}` — use for liveness checks.

### `GET /metrics`

Prometheus-compatible text format (coming soon).

## Feature Flags

| Flag  | Default | Description |
|-------|---------|-------------|
| `ros2` | on | ROS2 node/topic discovery via `ros2` CLI |
| `gpu` | off | GPU metrics via `nvidia-smi` / `rocm-smi` |

Build without ROS2 support:

```bash
cargo build --release --no-default-features
```

Build with GPU support:

```bash
cargo build --release --features gpu
```

## Architecture

```
/proc/* ──┐
           ├──> Collector Loop (2s) ──> MachineSnapshot ──> HTTP Server (:4040)
ros2 CLI ──┘
```

The daemon uses a `Collector` trait. Each collector module gathers one domain of data. The main loop runs all enabled collectors on a tick, merges results into a `MachineSnapshot`, and the HTTP server serves the latest snapshot.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

## License

Apache 2.0 — see [LICENSE](LICENSE) for details.
