# Generic Rust `tonic` + `buf` Template

**WARNING:** This template/repository is not ready for consumption.

As is, this just generates a bare bones client + server + service impl +
bindings for a known `proto` file.

There are faults with the overall template, but it should be functional.

This is my first template, bear with me, and see the Notes section.

## Usage

### Generate from Github

```shell
cargo generate gh:canardleteer/tonic-buf-template --name awesome tonic-buf-template
```

### Generate from cloned repo

```shell
cargo generate --path tonic-buf-template --name awesome
```

### Validation

```shell
cd awesome

# There are useful things to read in here.
cat README.md
```

## CI Testing

We generally just test that the template can build, and the generated template
itself, is buildable. We don't test variation from the default values.

I'd be okay with adding a matrix, of some slight variations beyond the
defaults.

## Notes / Faults

- I've done my best to annotate the template with `TEMPLATE_` in comments
  that are template related, and I'm still figuring out.

I'd like to do a couple of things here, none of which I'm doing at the
moment:

- Pull in some newer changes from [grpc-service-rs](https://github.com/canardleteer/grpc-service-rs).
- Default Github Actions
  - Had a hard time getting this to work with the templating engine, seemed to
    ignore the directory.
- Figure out how to support n-services, not just one.
  - We're close internally.
- Use `buf` with the `prost` plugin, for better discovery of generic
  protos, instead of a static one.
  - Optionally.
  - And remove `protoc` from the Dockerfile
- Add stock features to the prost output, potentially as features.
- Add some stock useful Layers (w3c tracing)
- `tracing` / logging could be done much better, and in a common pattern shared
  across services out of the box.
- Some prompts for:
  - TLS features
  - License
  - proto sourcing from a submodule/schema registry instead of a static one.
- `docker-compose` + static `envoy` config
- Some folks will want a `project-service` out of the box, others will want
  it to be in `project-server`.
- Templating of `proto/buf.yaml`
- A templated `buf.gen.yaml`
