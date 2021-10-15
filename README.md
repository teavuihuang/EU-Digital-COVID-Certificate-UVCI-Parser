# EU-Digital-COVID-Certificate-UVCI-Parser
Tool to parse &amp; verify EU Digital COVID Certificate UVCI (Unique Vaccination Certificate/Assertion Identifier)

See [Rust community’s crate registry](https://crates.io/crates/covid_cert_uvci) for documentation

Following the conclusions of the European Council of 10-11 December 2020 and of 21 January 2021 that called for “a coordinated approach to vaccination certificates”, these guidelines establish a unique identifier for vaccination certificates. This software library parses and verifies the EU Digital COVID Certificate UVCI according to eHealth Network Guidelines on ‘verifiable vaccination certificates - basic interoperability elements’ - Release 2. The inclusion of the checksum is optional. The prefix “URN:UVCI:” may be added. Verification is performed by this crate.

Only for Sweden EHM-issued COVID certificates: Export a vector of EU Digital COVID Certificate UVCIs to **Neo4j Cypher Graph**. Parsing of Swedish UVCI ‘Opaque Unique String’ is experimental. The Swedish vaccination dates are derived from the UVCI aganist national statistics for vaccination against COVID-19. The statistics is from the Public Health Agency of Sweden (Folkhalsomyndigheten) based on cumulatively number of vaccinations per week. The Swedish vaccination dates are predicted with an accuracy of approximately +/- 1 month. Test UVCI is generated using software from Sweden’s Agency for Digital Government (Myndigheten för digital förvaltning).


## Sample output

```
URN:UVCI:01:SE:EHM/V12916227TFJJ#Q
version                  : 1
country                  : SE
schema_option_number     : 3
schema_option_desc       : some semantics
issuing_entity           : EHM
vaccine_id               :
opaque_unique_string     : V12916227TFJJ
opaque_id                : V12916227
opaque_issuance          : TFJJ
opaque_vaccination_month : 8
opaque_vaccination_year  : 2021
checksum                 : Q
checksum_verification    : true

URN:UVCI:01:SE:EHM/C878/123456789ABC#B
version                  : 1
country                  : SE
schema_option_number     : 1
schema_option_desc       : identifier with semantics
issuing_entity           : EHM
vaccine_id               : C878
opaque_unique_string     : 123456789ABC
opaque_id                : 
opaque_issuance          : 
opaque_vaccination_month : 0
opaque_vaccination_year  : 0
checksum                 : B
checksum_verification    : true
```


## Sample graph

[![](https://raw.githubusercontent.com/teavuihuang/EU-Digital-COVID-Certificate-UVCI-Parser/main/examples/graph_33.png)](https://raw.githubusercontent.com/teavuihuang/EU-Digital-COVID-Certificate-UVCI-Parser/main/examples/graph_33.png)


## Usage (executable)
covid_cert_uvci [Name of Covid UVCI input file] [Name of Graph Cypher output file]


