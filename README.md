# Generic Rust `tonic` + `buf` Template

As is, this just generates a bare bones client + server + service impl +
bindings for a known `proto` file.

There are faults with the overall template, but it should be functional.

This is my first template, bear with me.

## Notes / Faults

I'd like to do a couple of things here, none of which I'm doing at the
moment:

- Figure out how to support n-services, not just one.
  - We're close.
- Use `buf` with the `prost` plugin, for better discovery of generic
  protos, instead of a static one.
  - And remove `protoc` from the Dockerfile
- Add stock features to the prost output.
- Add some stock useful Layers (w3c tracing)
- `tracing` / logging could be done much better, and in a common pattern shared
  across services out of the box.
- Some prompts for:
  - TLS features
  - License
  - proto sourcing from a submodule/schema registry instead of a static one.
- Default Github Actions
- `docker-compose` + static `envoy` config
- Some folks will want a `project-service` out of the box, others will want
  it to be in `project-server`
- Templating of `proto/buf.yaml`
- A templated `buf.gen.yaml`

## Usage

```shell
# replace this with gh usage
cargo generate --path tonic-buf-template --name awesome
cd awesome

# There are useful things to check in here.
cat README.md
```
