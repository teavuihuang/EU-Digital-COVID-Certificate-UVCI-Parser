# EU-Digital-COVID-Certificate-UVCI-Parser
Tool to parse &amp; verify EU Digital COVID Certificate UVCI (Unique Vaccination Certificate/Assertion Identifier)

See [Rust community’s crate registry](https://crates.io/crates/covid_cert_uvci)

Following the conclusions of the European Council of 10-11 December 2020 and of 21 January 2021 that called for “a coordinated approach to vaccination certificates”, these guidelines establish a unique identifier for vaccination certificates. This software library parses and verifies the EU Digital COVID Certificate UVCI according to eHealth Network Guidelines on ‘verifiable vaccination certificates - basic interoperability elements’ - Release 2. 

The inclusion of the checksum is optional. The prefix “URN:UVCI:” may be added. Verification is performed by this crate.


## Sample output

```
URN:UVCI:01:SE:EHM/C878/123456789ABC#B

version               : 1
country               : SE
schema_option_number  : 1
schema_option_desc    : identifier with semantics
issuing_entity        : EHM
vaccine_id            : C878
opaque_unique_string  : 123456789ABC
checksum              : B
checksum_verification : true
```
