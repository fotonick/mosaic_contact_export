# Mosaic Contact Export

This tool can export the contacts page from a [Mosaic](https://mosaicsoftware.org/) community site as a vCard (.vcf), which enables you to import all the contacts into your phone.

## How to run

1. Install a Rust toolchain, generally through [rustup](https://rustup.rs/), which can be installed by most package managers.
2. `MOSAIC_USERNAME=<username> MOSAIC_PASSWORD=<password> cargo run <your base URL> <your address without the leading number>`

As an example of step 2, here it is for [Cully Grove](https://cullygrove.org): `MOSAIC_USERNAME=<username> MOSAIC_PASSWORD=<password> cargo run https://cullygrove.zenlunatics.org "NE Going St, Portland, OR 97218"`

It will print a vCard 3.0 file to stdout, which you can redirect into a .vcf file. You can email this to yourself and open the attachment from your phone.

## License

This software is provided under the GPLv3 license. All contributions will be incorporated under the same terms unless explicitly excluded.
