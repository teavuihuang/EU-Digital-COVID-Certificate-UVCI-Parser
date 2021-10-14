use itertools::Itertools;
use luhn::Luhn;
use std::fmt;

/// EU Digital COVID Certificate UVCI (Unique Vaccination Certificate/Assertion Identifier) data.
#[derive(Clone)]
pub struct Uvci {
    /// Version of the UVCI schema, the version is composed of two digits, 0 for unknown
    pub version: u8,
    /// Country code is specified by ISO 3166-1
    pub country: String,
    /// EU member states can deploy different option in different version of the UVCI schema
    pub schema_option_number: u8,
    /// EU member states can deploy different option in different version of the UVCI schema, 0 for unknown
    pub schema_option_desc: String,
    /// The authority issuing the COVID certificate
    pub issuing_entity: String,
    /// Vaccine product identifier, vaccine/lot identifier(s) etc
    pub vaccine_id: String,
    /// The unique identifier of the vaccination in the national vaccination registry of the corresponding country
    pub opaque_unique_string: String,
    /// The unique opaque identifier of the vaccination in the national vaccination registry of the corresponding country
    pub opaque_id: String,
    /// The unique opaque issuance of the vaccination in the national vaccination registry of the corresponding country
    pub opaque_issuance: String,
    /// The opaque vaccination month of the vaccination in the national vaccination registry of the corresponding country
    pub opaque_vaccination_month: u8,
    /// The opaque vaccination year of the vaccination in the national vaccination registry of the corresponding country
    pub opaque_vaccination_year: u16,
    /// The ISO-7812-1 (LUHN-10) checksum used to verify the integrity of the UVCI
    pub checksum: String,
    /// Checksum verification. For successful verification the value is 'true', else 'false'
    pub checksum_verification: bool,
}

/// Display the parsed EU Digital COVID Certificate UVCI (Unique Vaccination Certificate/Assertion Identifier) data
impl fmt::Display for Uvci {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "version                  : {}\n\
            country                  : {}\n\
            schema_option_number     : {}\n\
            schema_option_desc       : {}\n\
            issuing_entity           : {}\n\
            vaccine_id               : {}\n\
            opaque_unique_string     : {}\n\
            opaque_id                : {}\n\
            opaque_issuance          : {}\n\
            opaque_vaccination_month : {}\n\
            opaque_vaccination_year  : {}\n\
            checksum                 : {}\n\
            checksum_verification    : {}\n",
            &self.version.to_string(),
            &self.country,
            &self.schema_option_number.to_string(),
            &self.schema_option_desc,
            &self.issuing_entity,
            &self.vaccine_id,
            &self.opaque_unique_string,
            &self.opaque_id,
            &self.opaque_issuance,
            &self.opaque_vaccination_month,
            &self.opaque_vaccination_year,
            &self.checksum,
            &self.checksum_verification
        )
    }
}

/// Export a EU Digital COVID Certificate UVCI to CSV
/// # Arguments
///
/// * `cert_id` - the UVCI (Unique Vaccination Certificate/Assertion Identifier), e.g. "URN:UVCI:01:SE:EHM/V12907267LAJW#E"
pub fn uvci_to_csv(cert_id: &str) -> String {
    return to_csv(parse(cert_id));
}

/// Export the parsed EU Digital COVID Certificate UVCI data to CSV
fn to_csv(uvci: Uvci) -> String {
    let mut output = "".to_string();
    output.push_str(&uvci.version.to_string());
    output.push_str(",");
    output.push_str(&uvci.country);
    output.push_str(",");
    output.push_str(&uvci.schema_option_number.to_string());
    output.push_str(",");
    output.push_str(&uvci.schema_option_desc);
    output.push_str(",");
    output.push_str(&uvci.issuing_entity);
    output.push_str(",");
    output.push_str(&uvci.vaccine_id);
    output.push_str(",");
    output.push_str(&uvci.opaque_unique_string);
    output.push_str(",");
    output.push_str(&uvci.opaque_id);
    output.push_str(",");
    output.push_str(&uvci.opaque_issuance);
    output.push_str(",");
    output.push_str(&uvci.opaque_vaccination_month.to_string());
    output.push_str(",");
    output.push_str(&uvci.opaque_vaccination_year.to_string());
    output.push_str(",");
    output.push_str(&uvci.checksum);
    output.push_str(",");
    output.push_str(&uvci.checksum_verification.to_string());
    return output.to_string();
}

/// Export a vector of EU Digital COVID Certificate UVCI to Neo4j Cypher Graph
///
/// Only for Sweden EHM-issued COVID certificates
/// # Arguments
///
/// * `cert_ids` - String vector of UVCI (Unique Vaccination Certificate/Assertion Identifier)
pub fn uvcis_to_graph(cert_ids: &Vec<String>) -> String {
    let mut cypher_cmd = "".to_string();
    for cert_id in cert_ids {
        cypher_cmd.push_str(&uvci_to_graph(cert_id));
    }
    // Remove duplicates
    let values: Vec<_> = cypher_cmd.split('\n').collect();
    let values: Vec<_> = values.into_iter().unique().collect();
    let cypher_output: String = values.into_iter().collect();
    let cypher_output = cypher_output.replace("CREATE", "\nCREATE");
    return cypher_output;
}

/// Export a EU Digital COVID Certificate UVCI to Neo4j Cypher Graph
///
/// Only for Sweden EHM-issued COVID certificates
/// # Arguments
///
/// * `cert_id` - the UVCI (Unique Vaccination Certificate/Assertion Identifier), e.g. "URN:UVCI:01:SE:EHM/V12907267LAJW#E"
pub fn uvci_to_graph(cert_id: &str) -> String {
    return to_graph(parse(cert_id));
}

/// Export the parsed EU Digital COVID Certificate UVCI data to Neo4j Cypher Graph
/// Only for Sweden EHM-issued COVID certificates
/// # Arguments
///
/// * `cert_id` - the UVCI (Unique Vaccination Certificate/Assertion Identifier), e.g. "URN:UVCI:01:SE:EHM/V12907267LAJW#E"
fn to_graph(uvci_data: Uvci) -> String {
    // Only for Sweden EHM-issued COVID certificates
    if !((uvci_data.version == 1)
        && (uvci_data.country == "SE")
        && (uvci_data.issuing_entity == "EHM")
        && (uvci_data.schema_option_number == 3))
    {
        return "".to_string();
    }

    // Init
    let mut cypher_cmd = "".to_string();
    let var_country = "Sweden";
    let var_issuer = "E-Hälso Myndigheten";

    // CREATE (SE:country {name:'Sweden'})-[:COUNTRY_OF {}]->(EHM:issuing_entity {name:'E-Hälso Myndigheten'})
    cypher_cmd.push_str("CREATE (");
    cypher_cmd.push_str(&uvci_data.country);
    cypher_cmd.push_str(":country {name:'");
    cypher_cmd.push_str(var_country);
    cypher_cmd.push_str("'})-[:COUNTRY_OF {}]->(");
    cypher_cmd.push_str(&uvci_data.issuing_entity);
    cypher_cmd.push_str(":issuing_entity {name:'");
    cypher_cmd.push_str(var_issuer);
    cypher_cmd.push_str("'})\n");

    // CREATE (EHM)-[:ISSUER_OF {}]->(V11916227:opaque_id {name:'V11916227'})
    cypher_cmd.push_str("CREATE (");
    cypher_cmd.push_str(&uvci_data.issuing_entity);
    cypher_cmd.push_str(")-[:ISSUER_OF {}]->(");
    cypher_cmd.push_str(&uvci_data.opaque_id);
    cypher_cmd.push_str(":opaque_id {name:'");
    cypher_cmd.push_str(&uvci_data.opaque_id);
    cypher_cmd.push_str("'})\n");

    // CREATE (d20218:vac_date {name:'Aug 2021'})
    let mut var_date_name = "d".to_string();
    var_date_name.push_str(&uvci_data.opaque_vaccination_year.to_string());
    var_date_name.push_str(&uvci_data.opaque_vaccination_month.to_string());

    let var_month_name;
    match uvci_data.opaque_vaccination_month {
        1 => var_month_name = "Jan".to_string(),
        2 => var_month_name = "Feb".to_string(),
        3 => var_month_name = "Mar".to_string(),
        4 => var_month_name = "Apr".to_string(),
        5 => var_month_name = "May".to_string(),
        6 => var_month_name = "Jun".to_string(),
        7 => var_month_name = "Jul".to_string(),
        8 => var_month_name = "Aug".to_string(),
        9 => var_month_name = "Sep".to_string(),
        10 => var_month_name = "Oct".to_string(),
        11 => var_month_name = "Nov".to_string(),
        12 => var_month_name = "Dec".to_string(),
        _ => var_month_name = "Unknown".to_string(),
    }
    let mut var_date_data = "".to_string();
    var_date_data.push_str(&var_month_name);
    var_date_data.push_str(" ");
    var_date_data.push_str(&uvci_data.opaque_vaccination_year.to_string());

    // CREATE (d20218:vac_date {name:'Aug 2021'})
    cypher_cmd.push_str("CREATE (");
    cypher_cmd.push_str(&var_date_name);
    cypher_cmd.push_str(":vac_date {name:'");
    cypher_cmd.push_str(&var_date_data);
    cypher_cmd.push_str("'})\n");

    // CREATE (d20218)-[:VAC_DATE_OF {}]->(V12916227)
    cypher_cmd.push_str("CREATE (");
    cypher_cmd.push_str(&var_date_name);
    cypher_cmd.push_str(")-[:VAC_DATE_OF {}]->(");
    cypher_cmd.push_str(&uvci_data.opaque_id);
    cypher_cmd.push_str(")\n");

    // CREATE (V11916227TFJJ:reissue_id {name:'TFJJ'})-[:REISSUE_OF {}]->(V11916227)
    cypher_cmd.push_str("CREATE (");
    cypher_cmd.push_str(&uvci_data.opaque_unique_string);
    cypher_cmd.push_str(":reissue_id {name:'");
    cypher_cmd.push_str(&uvci_data.opaque_issuance);
    cypher_cmd.push_str("'})-[:REISSUE_OF {}]->(");
    cypher_cmd.push_str(&uvci_data.opaque_id);
    cypher_cmd.push_str(")\n");

    // cypher_cmd.push_str("return *");
    return cypher_cmd;
}

/// ## EU Digital COVID Certificate UVCI (Unique Vaccination Certificate/Assertion Identifier) Parser
/// Tool to parse and verify the EU Digital COVID Certificate UVCI (Unique Vaccination Certificate/Assertion Identifier).
/// Following the conclusions of the European Council of 10-11 December 2020 and of 21 January 2021 that called for
/// “a coordinated approach to vaccination certificates”, these guidelines establish a unique identifier for vaccination certificates.
/// This software library parses the EU Digital COVID Certificate UVCI according to eHealth Network Guidelines on
/// 'verifiable vaccination certificates - basic interoperability elements' - Release 2.
/// The inclusion of the checksum is optional. The prefix "URN:UVCI:" may be added.
/// Verification is performed by this crate.
///
///
/// ```no_run
/// // URN:UVCI:01:SE:EHM/V12916227TFJJ#Q
/// // version                  : 1
/// // country                  : SE
/// // schema_option_number     : 3
/// // schema_option_desc       : some semantics
/// // issuing_entity           : EHM
/// // vaccine_id               :
/// // opaque_unique_string     : V12916227TFJJ
/// // opaque_id                : V12916227
/// // opaque_issuance          : TFJJ
/// // opaque_vaccination_month : 8
/// // opaque_vaccination_year  : 2021
/// // checksum                 : Q
/// // checksum_verification    : true
/// //
/// // URN:UVCI:01:SE:EHM/C878/123456789ABC#B
/// // version                  : 1
/// // country                  : SE
/// // schema_option_number     : 1
/// // schema_option_desc       : identifier with semantics
/// // issuing_entity           : EHM
/// // vaccine_id               : C878
/// // opaque_unique_string     : 123456789ABC
/// // opaque_id                :
/// // opaque_issuance          :
/// // opaque_vaccination_month : 0
/// // opaque_vaccination_year  : 0
/// // checksum                 : B
/// // checksum_verification    : true
/// ```
///
/// # Arguments
///
/// * `cert_id` - the UVCI (Unique Vaccination Certificate/Assertion Identifier), e.g. "URN:UVCI:01:SE:EHM/V12907267LAJW#E"
pub fn parse(cert_id: &str) -> Uvci {
    let mut uvci_data = Uvci {
        version: 0,
        country: "".to_string(),
        schema_option_number: 0,
        schema_option_desc: "".to_string(),
        issuing_entity: "".to_string(),
        vaccine_id: "".to_string(),
        opaque_unique_string: "".to_string(),
        opaque_id: "".to_string(),
        opaque_issuance: "".to_string(),
        opaque_vaccination_month: 0,
        opaque_vaccination_year: 0,
        checksum: "".to_string(),
        checksum_verification: false,
    };

    // Reject if empty
    if cert_id.is_empty() {
        return uvci_data;
    }

    // Up to a total length of 72 characters
    if cert_id.len() > 72 {
        return uvci_data;
    }

    // Only uppercase characters are allowed
    let cert_id = cert_id.to_uppercase();

    // Headers
    let mut cert_id2 = cert_id.clone();
    if !cert_id.starts_with("URN:UVCI:") {
        cert_id2 = "URN:UVCI:".to_owned() + &cert_id2;
    }
    let cert_id = cert_id2;

    // Verify integrity of the UVCI
    let l = Luhn::new("/0123456789:ABCDEFGHIJKLMNOPQRSTUVWXYZ").expect("invalid alphabet given");
    uvci_data.checksum_verification = l.validate(rearrange(cert_id.to_string())).unwrap();

    // Start parsing
    let split_checksum = cert_id.split("#");
    let vec: Vec<&str> = split_checksum.collect();
    if vec.len() > 1 {
        uvci_data.checksum = vec[1].to_string();
    }

    // Verify that the prefix "URN:UVCI:" is added
    let split_blocks = vec[0].split(":");
    let vec: Vec<&str> = split_blocks.collect();
    if vec[0] != "URN" && vec[1] != "UVCI" {
        return uvci_data;
    }

    // Detect schema
    if vec.len() < 4 {
        return uvci_data;
    }

    // UVCI schema version
    let temp = vec[2].to_string();
    if temp.parse::<u8>().is_ok() {
        uvci_data.version = temp.parse::<u8>().unwrap();
    }

    // ISO 3166-1 country code
    uvci_data.country = vec[3].to_string();

    // Detect schema
    if vec.len() < 5 {
        return uvci_data;
    }
    let split_options = vec[4].split("/");
    let vec: Vec<&str> = split_options.collect();
    match vec.len() {
        3 => {
            uvci_data.schema_option_number = 1;
            uvci_data.schema_option_desc = "identifier with semantics".to_string();
            uvci_data.issuing_entity = vec[0].to_string();
            uvci_data.vaccine_id = vec[1].to_string();
            uvci_data.opaque_unique_string = vec[2].to_string();
        }
        1 => {
            uvci_data.schema_option_number = 2;
            uvci_data.schema_option_desc = "opaque identifier - no structure".to_string();
            uvci_data.opaque_unique_string = vec[0].to_string();
        }
        2 => {
            uvci_data.schema_option_number = 3;
            uvci_data.schema_option_desc = "some semantics".to_string();
            uvci_data.issuing_entity = vec[0].to_string();
            uvci_data.opaque_unique_string = vec[1].to_string();
        }
        _ => (),
    }

    // Only for Sweden EHM-issued COVID certificates
    if (uvci_data.version == 1)
        && (uvci_data.country == "SE")
        && (uvci_data.issuing_entity == "EHM")
        && (uvci_data.schema_option_number == 3)
    {
        if uvci_data.opaque_unique_string.len() == 13 {
            uvci_data.opaque_id = (&uvci_data.opaque_unique_string[0..9]).to_string();
            uvci_data.opaque_issuance = (&uvci_data.opaque_unique_string[9..13]).to_string();

            let vaccination_date = get_vaccination_date_tan(uvci_data.opaque_id.clone());
            uvci_data.opaque_vaccination_month = vaccination_date.0;
            uvci_data.opaque_vaccination_year = vaccination_date.1;
        }
    }

    return uvci_data;
}

/// Rearrange the UVCI characters to enable validation of the checksum
///
/// EU Digital COVID Certificate UVCI uses "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/:",
/// whereas 'luhn-rs' crate uses "/0123456789:ABCDEFGHIJKLMNOPQRSTUVWXYZ"
/// # Arguments
///
/// * `cert_id` - the UVCI (Unique Vaccination Certificate/Assertion Identifier), e.g. "URN:UVCI:01:SE:EHM/V12907267LAJW#E"
fn rearrange(cert_id: String) -> String {
    let cert_id = cert_id.to_uppercase();
    let cert_id = cert_id.replace("#", "");
    let cert_id = cert_id.replace("M", "a");
    let cert_id = cert_id.replace("N", "b");
    let cert_id = cert_id.replace("O", "c");
    let cert_id = cert_id.replace("P", "d");
    let cert_id = cert_id.replace("Q", "e");
    let cert_id = cert_id.replace("R", "f");
    let cert_id = cert_id.replace("S", "g");
    let cert_id = cert_id.replace("T", "h");
    let cert_id = cert_id.replace("U", "i");
    let cert_id = cert_id.replace("V", "j");
    let cert_id = cert_id.replace("W", "k");
    let cert_id = cert_id.replace("X", "l");
    let cert_id = cert_id.replace("Y", "m");
    let cert_id = cert_id.replace("Z", "m");
    let cert_id = cert_id.replace("0", "o");
    let cert_id = cert_id.replace("1", "p");
    let cert_id = cert_id.replace("2", "q");
    let cert_id = cert_id.replace("3", "r");
    let cert_id = cert_id.replace("4", "s");
    let cert_id = cert_id.replace("5", "t");
    let cert_id = cert_id.replace("6", "u");
    let cert_id = cert_id.replace("7", "v");
    let cert_id = cert_id.replace("8", "w");
    let cert_id = cert_id.replace("9", "x");
    let cert_id = cert_id.replace("/", "y");
    let cert_id = cert_id.replace(":", "z");
    let cert_id = cert_id.replace("A", "/");
    let cert_id = cert_id.replace("B", "0");
    let cert_id = cert_id.replace("C", "1");
    let cert_id = cert_id.replace("D", "2");
    let cert_id = cert_id.replace("E", "3");
    let cert_id = cert_id.replace("F", "4");
    let cert_id = cert_id.replace("G", "5");
    let cert_id = cert_id.replace("H", "6");
    let cert_id = cert_id.replace("I", "7");
    let cert_id = cert_id.replace("J", "8");
    let cert_id = cert_id.replace("K", "9");
    let cert_id = cert_id.replace("L", ":");
    return cert_id.to_uppercase();
}

/// Estimate vaccination month & year from opaque_issuance_id in UVCI opaque_unique_string
///
/// # Arguments
///
/// * `opaque_id` - e.g. "V12907267"
fn get_vaccination_date_tan(opaque_id: String) -> (u8, u16) {
    // vaccination_month from 0-xxxx
    let opaque_id = opaque_id.replace("V", "");
    if !opaque_id.parse::<f32>().is_ok() {
        return (0, 0);
    }
    let mut vaccination_doses = opaque_id.parse::<f32>().unwrap();

    // Reject negative numbers
    if vaccination_doses < 0.0 {
        return (0, 0);
    }

    let mut vaccination_month;
    if vaccination_doses <= 13983264.0 {
        // Use tangent cruve
        vaccination_doses = (6991632.0 - vaccination_doses) / 5536858.0;
        let mth_f = 5.03 + ((-vaccination_doses.tan()) * 1.6);
        let mth_u8 = mth_f.round() as u16;
        vaccination_month = mth_u8;
    } else {
        // Assuming 1552008 doses a month
        vaccination_month = (vaccination_doses / 1552008.0) as u16;
    }

    // vaccination_year from 2020-xxxx
    let vaccination_year;
    if vaccination_month == 0 {
        vaccination_year = 2020;
    } else {
        vaccination_year = ((vaccination_month - 1) / 12) + 2021;
    }

    // Reformat vaccination_month from 0-11 to 1-12
    if vaccination_month == 0 {
        vaccination_month = 12;
    }
    while vaccination_month > 12 {
        vaccination_month = vaccination_month - 12;
    }

    // Return data
    return (vaccination_month as u8, vaccination_year as u16);
}

#[cfg(test)]
mod tests {
    use super::get_vaccination_date_tan;
    use super::parse;
    use super::uvci_to_csv;

    #[test]
    fn uvci_csv() {
        assert!(
            uvci_to_csv("URN:UVCI:01:SE:EHM/V00016227TFJJ#Q")
                == "1,SE,3,some semantics,EHM,,V00016227TFJJ,V00016227,TFJJ,12,2020,Q,false"
        );
    }

    #[test]
    fn swedish_uvci_opaque_date() {
        assert!(
            get_vaccination_date_tan("0".to_string()) == (12, 2020),
            "Dec, Wrong date"
        );
        assert!(
            get_vaccination_date_tan("2014920".to_string()) == (3, 2021),
            "March, Wrong date"
        );
        assert!(
            get_vaccination_date_tan("6991632".to_string()) == (5, 2021),
            "May, Wrong date"
        );
        assert!(
            get_vaccination_date_tan("12916227".to_string()) == (8, 2021),
            "Aug, Wrong date"
        );
        assert!(
            get_vaccination_date_tan("13592955".to_string()) == (9, 2021),
            "Sep, Wrong date"
        );
        assert!(
            get_vaccination_date_tan("13983264".to_string()) == (10, 2021),
            "Oct, Wrong date"
        );
        assert!(
            get_vaccination_date_tan("99999999".to_string()) == (4, 2026),
            "Max, wrong date"
        );
        // Sweden Population = 10427296, Reference period: August 2021
        assert!(
            get_vaccination_date_tan("10427296".to_string()) == (6, 2021),
            "Single dose, wrong date"
        );
        assert!(
            get_vaccination_date_tan("20854592".to_string()) == (1, 2022),
            "Double dose, wrong date"
        );
        assert!(
            get_vaccination_date_tan("31281888".to_string()) == (8, 2022),
            "Double dose + booster, wrong date"
        );
    }

    #[test]
    fn swedish_uvci_opaque_data() {
        assert!(
            parse("URN:UVCI:01:SE:EHM/V12907267LAJW#E").opaque_unique_string == "V12907267LAJW",
            "wrong opaque_unique_string"
        );
        assert!(
            parse("URN:UVCI:01:SE:EHM/V12907267LAJW#E").opaque_id == "V12907267",
            "wrong opaque_id"
        );
        assert!(
            parse("URN:UVCI:01:SE:EHM/V12907267LAJW#E").opaque_issuance == "LAJW",
            "wrong opaque_issuance"
        );
        assert!(
            parse("URN:UVCI:01:SE:EHM/V12907267LAJW#E").opaque_vaccination_month == 8,
            "wrong opaque_vaccination_month"
        );
        assert!(
            parse("URN:UVCI:01:SE:EHM/V12907267LAJW#E").opaque_vaccination_year == 2021,
            "wrong opaque_vaccination_month"
        );
    }

    #[test]
    fn swedish_uvci_with_checksum_valid() {
        let cert_ids_sweden: [&str; 15] = [
            "URN:UVCI:01:SE:EHM/V12907267LAJW#E",
            "URN:UVCI:01:SE:EHM/V12916227TFJJ#Q",
            "URN:UVCI:01:SE:EHM/V12920064NYOH#4",
            "URN:UVCI:01:SE:EHM/V12923931NNBY#T",
            "URN:UVCI:01:SE:EHM/V12939008LSVR#F",
            "URN:UVCI:01:SE:EHM/V12939037PXFJ#V",
            "URN:UVCI:01:SE:EHM/V12940126MRXQ#N",
            "URN:UVCI:01:SE:EHM/V12956472WRGE#7",
            "URN:UVCI:01:SE:EHM/V12965046ALNM#I",
            "URN:UVCI:01:SE:EHM/V12982924YQMV#T",
            "URN:UVCI:01:SE:EHM/V12991074UCIC#4",
            "URN:UVCI:01:SE:EHM/V12993686OVCX#R",
            "URN:UVCI:01:SE:EHM/V12996544DVKM#M",
            "URN:UVCI:01:SE:EHM/V12997980ASMG#1",
            "URN:UVCI:01:SE:EHM/V12998404MNQF#6",
        ];
        for cert_id in &cert_ids_sweden {
            println!("{}\n{}\n", cert_id, parse(cert_id));
            assert!(
                parse(cert_id).checksum_verification,
                "checksum verification failed"
            );
        }
    }

    #[test]
    fn swedish_uvci_with_checksum_invalid() {
        let cert_ids_sweden: [&str; 15] = [
            "URN:UVCI:01:SE:EHM/V12907267LAJW#A",
            "URN:UVCI:01:SE:EHM/V12916227TFJJ#B",
            "URN:UVCI:01:SE:EHM/V12920064NYOH#C",
            "URN:UVCI:01:SE:EHM/V12923931NNBY#D",
            "URN:UVCI:01:SE:EHM/V12939008LSVR#E",
            "URN:UVCI:01:SE:EHM/V12939037PXFJ#F",
            "URN:UVCI:01:SE:EHM/V12940126MRXQ#G",
            "URN:UVCI:01:SE:EHM/V12956472WRGE#H",
            "URN:UVCI:01:SE:EHM/V12965046ALNM#0",
            "URN:UVCI:01:SE:EHM/V12982924YQMV#1",
            "URN:UVCI:01:SE:EHM/V12991074UCIC#2",
            "URN:UVCI:01:SE:EHM/V12993686OVCX#3",
            "URN:UVCI:01:SE:EHM/V12996544DVKM#4",
            "URN:UVCI:01:SE:EHM/V12997980ASMG#5",
            "URN:UVCI:01:SE:EHM/V12998404MNQF#9",
        ];
        for cert_id in &cert_ids_sweden {
            println!("{}\n{}\n", cert_id, parse(cert_id));
            assert!(
                !parse(cert_id).checksum_verification,
                "checksum verification failed"
            );
        }
    }

    #[test]
    fn assorted_uvci() {
        let cert_ids_assorted: [&str; 18] = [
            "",
            "a",
            "::::::::::",
            "//////////",
            "a:a:a:a:a:a:a:a:a:a:a",
            "URN:UVCI:01:SE://////////",
            "URN:UVCI:01:AT:10807843F94AEE0EE5093FBC254BD8131080784F94AEE0E43C25D813#B",
            "URN:UVCI:01:SE:EHM/C878/123456789ABC",
            "URN:UVCI:01:SE:EHM/C878/123456789ABC#B",
            "01:SE:EHM/C878/123456789ABC#B",
            "URN:UVCI:01:SE:123456789ABC",
            "URN:UVCI:01:AT:10807843F94AEE0EE5093FBC254BD813#B",
            "URN:UVCI:01:SE:EHM/V12916227TFJJ#Q",
            "URN:UVCI:01:NL:187/37512422923",
            "urn:uvci:01:se:ehm/v12982924yqmv#t",
            "urn:uvci:98:se:ehm/v12982924yqmv#t",
            "URN:UVCI:01:IT:84A0F1A35F1D454C96939812CA55D571#F",
            "01:IT:84A0F1A35F1D454C96939812CA55D571#F",
        ];

        for cert_id in &cert_ids_assorted {
            println!("{}\n{}\n", cert_id, parse(cert_id));
            assert!(
                parse(cert_id).schema_option_number <= 3,
                "schema_option_number larger than 3"
            );
        }
    }
}
