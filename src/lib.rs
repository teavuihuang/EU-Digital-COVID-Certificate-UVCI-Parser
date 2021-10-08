use luhn::Luhn;
use std::fmt;

/// EU Digital COVID Certificate UVCI (Unique Vaccination Certificate/Assertion Identifier) data.
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
    /// The ISO-7812-1 (LUHN-10) checksum used to verify the integrity of the UVCI
    pub checksum: String,
    /// Checksum verification. For successful verification the value is 'true', else 'false'
    pub checksum_verification: bool,
}

/// Display the parsed EU Digital COVID Certificate (Unique Vaccination Certificate/Assertion Identifier) data
impl fmt::Display for Uvci {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "version               : {}\n\
            country               : {}\n\
            schema_option_number  : {}\n\
            schema_option_desc    : {}\n\
            issuing_entity        : {}\n\
            vaccine_id            : {}\n\
            opaque_unique_string  : {}\n\
            checksum              : {}\n\
            checksum_verification : {}\n",
            &self.version.to_string(),
            &self.country,
            &self.schema_option_number.to_string(),
            &self.schema_option_desc,
            &self.issuing_entity,
            &self.vaccine_id,
            &self.opaque_unique_string,
            &self.checksum,
            &self.checksum_verification
        )
    }
}

/// ## EU Digital COVID Certificate UVCI (Unique Vaccination Certificate/Assertion Identifier) Parser
/// Tool to parse EU Digital COVID Certificate UVCI (Unique Vaccination Certificate/Assertion Identifier).
/// Following the conclusions of the European Council of 10-11 December 2020 and of 21 January 2021 that called for
/// “a coordinated approach to vaccination certificates”, these guidelines establish a unique identifier for vaccination certificates.
/// This software library parses the EU Digital COVID Certificate UVCI according to eHealth Network Guidelines on
/// 'verifiable vaccination certificates - basic interoperability elements' - Release 2.
/// The inclusion of the checksum is optional. The prefix "URN:UVCI:" may be added.
/// Verification is performed by this crate.
///
///
/// ```no_run
/// // URN:UVCI:01:SE:EHM/C878/123456789ABC#B
///
/// // version               : 1
/// // country               : SE
/// // schema_option_number  : 1
/// // schema_option_desc    : identifier with semantics
/// // issuing_entity        : EHM
/// // vaccine_id            : C878
/// // opaque_unique_string  : 123456789ABC
/// // checksum              : B
/// // checksum_verification : true
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

#[cfg(test)]
mod tests {
    use super::parse;

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
