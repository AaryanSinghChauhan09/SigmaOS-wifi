# SigmaOS-wifi

Component boundary for Wi-Fi device discovery, firmware lifecycle, interface
control, and link events. The component manifest assigns canonical ownership
to `SigmaOS-services`.

## Current state

`COMPONENT.toml` marks this repository as a scaffold. The current checkout has
the component manifest, license, README, and a metadata validation workflow;
it does not yet contain a device backend or a stable service API.

## Intended boundary

Keep hardware discovery and firmware/device operations separate from network
connection policy and desktop presentation. Expose link and device state
through a documented contract so the owning services layer can coordinate
networking without depending on a particular UI. Hardware access, firmware
availability, permissions, and failure reporting need explicit handling in
the eventual implementation.

## Readiness evidence

Before claiming device support, identify the supported interfaces and hardware
scope, implement and document the service contract, and provide reproducible
checks for device discovery and failure handling. The current metadata
workflow validates declarations only; it does not test hardware behavior.
