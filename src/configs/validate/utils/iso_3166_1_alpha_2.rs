// Check if a two-letter country code is valid according to ISO 3166-1 alpha-2 standard
pub fn is_valid_iso_3166_1_alpha_2(code: &str) -> bool {
    const ISO_3166_1_ALPHA_2_CODES: [&str; 249] = [
        "AF", // Afghanistan
        "AX", // Åland Islands
        "AL", // Albania
        "DZ", // Algeria
        "AS", // American Samoa
        "AD", // Andorra
        "AO", // Angola
        "AI", // Anguilla
        "AQ", // Antarctica
        "AG", // Antigua and Barbuda
        "AR", // Argentina
        "AM", // Armenia
        "AW", // Aruba
        "AU", // Australia
        "AT", // Austria
        "AZ", // Azerbaijan
        "BS", // Bahamas
        "BH", // Bahrain
        "BD", // Bangladesh
        "BB", // Barbados
        "BY", // Belarus
        "BE", // Belgium
        "BZ", // Belize
        "BJ", // Benin
        "BM", // Bermuda
        "BT", // Bhutan
        "BO", // Bolivia (Plurinational State of)
        "BQ", // Bonaire, Sint Eustatius and Saba
        "BA", // Bosnia and Herzegovina
        "BW", // Botswana
        "BV", // Bouvet Island
        "BR", // Brazil
        "IO", // British Indian Ocean Territory
        "BN", // Brunei Darussalam
        "BG", // Bulgaria
        "BF", // Burkina Faso
        "BI", // Burundi
        "CV", // Cabo Verde
        "KH", // Cambodia
        "CM", // Cameroon
        "CA", // Canada
        "KY", // Cayman Islands
        "CF", // Central African Republic
        "TD", // Chad
        "CL", // Chile
        "CN", // China
        "CX", // Christmas Island
        "CC", // Cocos (Keeling) Islands
        "CO", // Colombia
        "KM", // Comoros
        "CD", // Congo (the Democratic Republic of the)
        "CG", // Congo
        "CK", // Cook Islands
        "CR", // Costa Rica
        "HR", // Croatia
        "CU", // Cuba
        "CW", // Curaçao
        "CY", // Cyprus
        "CZ", // Czechia
        "CI", // Côte d'Ivoire
        "DK", // Denmark
        "DJ", // Djibouti
        "DM", // Dominica
        "DO", // Dominican Republic
        "EC", // Ecuador
        "EG", // Egypt
        "SV", // El Salvador
        "GQ", // Equatorial Guinea
        "ER", // Eritrea
        "EE", // Estonia
        "SZ", // Eswatini
        "ET", // Ethiopia
        "FK", // Falkland Islands (Malvinas)
        "FO", // Faroe Islands
        "FJ", // Fiji
        "FI", // Finland
        "FR", // France
        "GF", // French Guiana
        "PF", // French Polynesia
        "TF", // French Southern Territories
        "GA", // Gabon
        "GM", // Gambia
        "GE", // Georgia
        "DE", // Germany
        "GH", // Ghana
        "GI", // Gibraltar
        "GR", // Greece
        "GL", // Greenland
        "GD", // Grenada
        "GP", // Guadeloupe
        "GU", // Guam
        "GT", // Guatemala
        "GG", // Guernsey
        "GN", // Guinea
        "GW", // Guinea-Bissau
        "GY", // Guyana
        "HT", // Haiti
        "HM", // Heard Island and McDonald Islands
        "VA", // Holy See
        "HN", // Honduras
        "HK", // Hong Kong
        "HU", // Hungary
        "IS", // Iceland
        "IN", // India
        "ID", // Indonesia
        "IR", // Iran (Islamic Republic of)
        "IQ", // Iraq
        "IE", // Ireland
        "IM", // Isle of Man
        "IL", // Israel
        "IT", // Italy
        "JM", // Jamaica
        "JP", // Japan
        "JE", // Jersey
        "JO", // Jordan
        "KZ", // Kazakhstan
        "KE", // Kenya
        "KI", // Kiribati
        "KP", // Korea (the Democratic People's Republic of)
        "KR", // Korea (the Republic of)
        "KW", // Kuwait
        "KG", // Kyrgyzstan
        "LA", // Lao People's Democratic Republic
        "LV", // Latvia
        "LB", // Lebanon
        "LS", // Lesotho
        "LR", // Liberia
        "LY", // Libya
        "LI", // Liechtenstein
        "LT", // Lithuania
        "LU", // Luxembourg
        "MO", // Macao
        "MG", // Madagascar
        "MW", // Malawi
        "MY", // Malaysia
        "MV", // Maldives
        "ML", // Mali
        "MT", // Malta
        "MH", // Marshall Islands
        "MQ", // Martinique
        "MR", // Mauritania
        "MU", // Mauritius
        "YT", // Mayotte
        "MX", // Mexico
        "FM", // Micronesia (Federated States of)
        "MD", // Moldova (the Republic of)
        "MC", // Monaco
        "MN", // Mongolia
        "ME", // Montenegro
        "MS", // Montserrat
        "MA", // Morocco
        "MZ", // Mozambique
        "MM", // Myanmar
        "NA", // Namibia
        "NR", // Nauru
        "NP", // Nepal
        "NL", // Netherlands
        "NC", // New Caledonia
        "NZ", // New Zealand
        "NI", // Nicaragua
        "NE", // Niger
        "NG", // Nigeria
        "NU", // Niue
        "NF", // Norfolk Island
        "MK", // North Macedonia
        "MP", // Northern Mariana Islands
        "NO", // Norway
        "OM", // Oman
        "PK", // Pakistan
        "PW", // Palau
        "PS", // Palestine, State of
        "PA", // Panama
        "PG", // Papua New Guinea
        "PY", // Paraguay
        "PE", // Peru
        "PH", // Philippines
        "PN", // Pitcairn
        "PL", // Poland
        "PT", // Portugal
        "PR", // Puerto Rico
        "QA", // Qatar
        "RE", // Réunion
        "RO", // Romania
        "RU", // Russian Federation
        "RW", // Rwanda
        "BL", // Saint Barthélemy
        "SH", // Saint Helena, Ascension and Tristan da Cunha
        "KN", // Saint Kitts and Nevis
        "LC", // Saint Lucia
        "MF", // Saint Martin (French part)
        "PM", // Saint Pierre and Miquelon
        "VC", // Saint Vincent and the Grenadines
        "WS", // Samoa
        "SM", // San Marino
        "ST", // Sao Tome and Principe
        "SA", // Saudi Arabia
        "SN", // Senegal
        "RS", // Serbia
        "SC", // Seychelles
        "SL", // Sierra Leone
        "SG", // Singapore
        "SX", // Sint Maarten (Dutch part)
        "SK", // Slovakia
        "SI", // Slovenia
        "SB", // Solomon Islands
        "SO", // Somalia
        "ZA", // South Africa
        "GS", // South Georgia and the South Sandwich Islands
        "SS", // South Sudan
        "ES", // Spain
        "LK", // Sri Lanka
        "SD", // Sudan
        "SR", // Suriname
        "SJ", // Svalbard and Jan Mayen
        "SE", // Sweden
        "CH", // Switzerland
        "SY", // Syrian Arab Republic
        "TW", // Taiwan (Province of China)
        "TJ", // Tajikistan
        "TZ", // Tanzania, United Republic of
        "TH", // Thailand
        "TL", // Timor-Leste
        "TG", // Togo
        "TK", // Tokelau
        "TO", // Tonga
        "TT", // Trinidad and Tobago
        "TN", // Tunisia
        "TR", // Turkey
        "TM", // Turkmenistan
        "TC", // Turks and Caicos Islands
        "TV", // Tuvalu
        "UG", // Uganda
        "UA", // Ukraine
        "AE", // United Arab Emirates
        "GB", // United Kingdom of Great Britain and Northern Ireland
        "UM", // United States Minor Outlying Islands
        "US", // United States of America
        "UY", // Uruguay
        "UZ", // Uzbekistan
        "VU", // Vanuatu
        "VE", // Venezuela (Bolivarian Republic of)
        "VN", // Viet Nam
        "VG", // Virgin Islands (British)
        "VI", // Virgin Islands (U.S.)
        "WF", // Wallis and Futuna
        "EH", // Western Sahara
        "YE", // Yemen
        "ZM", // Zambia
        "ZW", // Zimbabwe
    ];
    ISO_3166_1_ALPHA_2_CODES.contains(&code.to_uppercase().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_codes_uppercase() {
        assert!(is_valid_iso_3166_1_alpha_2("AU"));
        assert!(is_valid_iso_3166_1_alpha_2("US"));
        assert!(is_valid_iso_3166_1_alpha_2("GB"));
        assert!(is_valid_iso_3166_1_alpha_2("DE"));
    }

    #[test]
    fn test_valid_codes_lowercase() {
        assert!(is_valid_iso_3166_1_alpha_2("au"));
        assert!(is_valid_iso_3166_1_alpha_2("us"));
        assert!(is_valid_iso_3166_1_alpha_2("gb"));
        assert!(is_valid_iso_3166_1_alpha_2("de"));
        assert!(is_valid_iso_3166_1_alpha_2("fr"));
    }

    #[test]
    fn test_valid_codes_mixed_case() {
        assert!(is_valid_iso_3166_1_alpha_2("Au"));
        assert!(is_valid_iso_3166_1_alpha_2("uS"));
        assert!(is_valid_iso_3166_1_alpha_2("Gb"));
        assert!(is_valid_iso_3166_1_alpha_2("dE"));
    }

    #[test]
    fn test_invalid_codes() {
        assert!(!is_valid_iso_3166_1_alpha_2("XX"));
        assert!(!is_valid_iso_3166_1_alpha_2("ZZ"));
        assert!(!is_valid_iso_3166_1_alpha_2("AA"));
        assert!(!is_valid_iso_3166_1_alpha_2("99"));
        assert!(!is_valid_iso_3166_1_alpha_2("A1"));
    }

    #[test]
    fn test_special_characters() {
        assert!(!is_valid_iso_3166_1_alpha_2("A-"));
        assert!(!is_valid_iso_3166_1_alpha_2("A "));
        assert!(!is_valid_iso_3166_1_alpha_2(" A"));
        assert!(!is_valid_iso_3166_1_alpha_2("A@"));
    }

    #[test]
    fn test_all_249_codes_are_valid() {
        let codes = [
            "AF", "AX", "AL", "DZ", "AS", "AD", "AO", "AI", "AQ", "AG",
            "AR", "AM", "AW", "AU", "AT", "AZ", "BS", "BH", "BD", "BB",
            "BY", "BE", "BZ", "BJ", "BM", "BT", "BO", "BQ", "BA", "BW",
            "BV", "BR", "IO", "BN", "BG", "BF", "BI", "CV", "KH", "CM",
            "CA", "KY", "CF", "TD", "CL", "CN", "CX", "CC", "CO", "KM",
            "CD", "CG", "CK", "CR", "HR", "CU", "CW", "CY", "CZ", "CI",
            "DK", "DJ", "DM", "DO", "EC", "EG", "SV", "GQ", "ER", "EE",
            "SZ", "ET", "FK", "FO", "FJ", "FI", "FR", "GF", "PF", "TF",
            "GA", "GM", "GE", "DE", "GH", "GI", "GR", "GL", "GD", "GP",
            "GU", "GT", "GG", "GN", "GW", "GY", "HT", "HM", "VA", "HN",
            "HK", "HU", "IS", "IN", "ID", "IR", "IQ", "IE", "IM", "IL",
            "IT", "JM", "JP", "JE", "JO", "KZ", "KE", "KI", "KP", "KR",
            "KW", "KG", "LA", "LV", "LB", "LS", "LR", "LY", "LI", "LT",
            "LU", "MO", "MG", "MW", "MY", "MV", "ML", "MT", "MH", "MQ",
            "MR", "MU", "YT", "MX", "FM", "MD", "MC", "MN", "ME", "MS",
            "MA", "MZ", "MM", "NA", "NR", "NP", "NL", "NC", "NZ", "NI",
            "NE", "NG", "NU", "NF", "MK", "MP", "NO", "OM", "PK", "PW",
            "PS", "PA", "PG", "PY", "PE", "PH", "PN", "PL", "PT", "PR",
            "QA", "RE", "RO", "RU", "RW", "BL", "SH", "KN", "LC", "MF",
            "PM", "VC", "WS", "SM", "ST", "SA", "SN", "RS", "SC", "SL",
            "SG", "SX", "SK", "SI", "SB", "SO", "ZA", "GS", "SS", "ES",
            "LK", "SD", "SR", "SJ", "SE", "CH", "SY", "TW", "TJ", "TZ",
            "TH", "TL", "TG", "TK", "TO", "TT", "TN", "TR", "TM", "TC",
            "TV", "UG", "UA", "AE", "GB", "UM", "US", "UY", "UZ", "VU",
            "VE", "VN", "VG", "VI", "WF", "EH", "YE", "ZM", "ZW",
        ];
        
        for code in &codes {
            assert!(
                is_valid_iso_3166_1_alpha_2(code),
                "Code {} should be valid",
                code
            );
        }
        
        // Verify we tested all 249 codes
        assert_eq!(codes.len(), 249);
    }
}

