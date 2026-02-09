use quickjs_regex::Regex;

/// All 96 noseyparker patterns from rebar's wild/noseyparker.txt benchmark.
const PATTERNS: &[&str] = &[
    r#"(?i)\b(p8e-[a-z0-9-]{32})(?:[^a-z0-9-]|$)"#,
    r#"\bage1[0-9a-z]{58}\b"#,
    r#"\bAGE-SECRET-KEY-1[0-9A-Z]{58}\b"#,
    r#"(?i)artifactory.{0,50}\b([a-z0-9]{73})\b"#,
    r#"\b((?:A3T[A-Z0-9]|AKIA|AGPA|AIDA|AROA|AIPA|ANPA|ANVA|ASIA)[A-Z0-9]{16})\b"#,
    r#"(?i)\baws_?(?:secret)?_?(?:access)?_?(?:key)?["'']?\s{0,30}(?::|=>|=)\s{0,30}["'']?([a-z0-9/+=]{40})\b"#,
    r#"(?i)aws_?(?:account)_?(?:id)?["''`]?\s{0,30}(?::|=>|=)\s{0,30}["''`]?([0-9]{4}-?[0-9]{4}-?[0-9]{4})"#,
    r#"(?i)(?:aws.?session|aws.?session.?token|aws.?token)["''`]?\s{0,30}(?::|=>|=)\s{0,30}["''`]?([a-z0-9/+=]{16,200})[^a-z0-9/+=]"#,
    r#"(?i)amzn\.mws\.([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})"#,
    r#"(?i)(?:AccountName|SharedAccessKeyName|SharedSecretIssuer)\s*=\s*([^;]{1,80})\s*;\s*.{0,10}\s*(?:AccountKey|SharedAccessKey|SharedSecretValue)\s*=\s*([^;]{1,100})(?:;|$)"#,
    r#"(https://[a-zA-Z0-9-]+\.azconfig\.io);Id=(.{4}-.{2}-.{2}:[a-zA-Z0-9+/]{18,22});Secret=([a-zA-Z0-9+/]{36,50}=)"#,
    r#"(?i)codeclima.{0,50}\b([a-f0-9]{64})\b"#,
    r#"\bcio[a-zA-Z0-9]{32}\b"#,
    r#"(?i)\b(doo_v1_[a-f0-9]{64})\b"#,
    r#"(?i)\b(dop_v1_[a-f0-9]{64})\b"#,
    r#"(?i)\b(dor_v1_[a-f0-9]{64})\b"#,
    r#"\b(dt0[a-zA-Z]{1}[0-9]{2}\.[A-Z0-9]{24}\.[A-Z0-9]{64})\b"#,
    r#"(?i)\b(?:facebook|fb).?(?:api|app|application|client|consumer|customer|secret|key).?(?:key|oauth|sec|secret)?.{0,2}\s{0,20}.{0,2}\s{0,20}.{0,2}\b([a-z0-9]{32})\b"#,
    r#"\b(EAACEdEose0cBA[a-zA-Z0-9]+)\b"#,
    r#"(?i)figma.{0,20}\b([0-9a-f]{4}-[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})\b"#,
    r#"(?i)secret.{0,20}\b([0-9a-z]{32,64})\b"#,
    r#"(?i)(?:api_key|apikey|access_key|accesskey).{0,3}[ \t]*(?::|=|:=|=>|,|'|")[ \t]*.{0,3}\b([0-9a-z][0-9a-z\-._/+]{30,62}[0-9a-z])\b"#,
    r#"(?:username|USERNAME|user|USER)[ \t]*=[ \t]*["']([a-zA-Z0-9.@_\-+]{3,30})["']\s*[,;]?\s*(?:\s*(?:\#|//)[^\n\r]*[\n\r])*(?:password|pass|PASSWORD|PASS)[ \t]*=[ \t]*["']([^"']{5,30})["']"#,
    r#"(?:username|USERNAME|user|USER)[ \t]*=[ \t]*([a-zA-Z0-9.@_\-+]{3,30})\s*;?\s*(?:\s*(?:\#|//)[^\n\r]*[\n\r])*(?:password|pass|PASSWORD|PASS)[ \t]*=[ \t]*(\S{5,30})(?:\s|$)"#,
    r#"\b(ghp_[a-zA-Z0-9]{36})\b"#,
    r#"\b(gho_[a-zA-Z0-9]{36})\b"#,
    r#"\b((?:ghu|ghs)_[a-zA-Z0-9]{36})\b"#,
    r#"\b(ghr_[a-zA-Z0-9]{76})\b"#,
    r#"(?i)(?:github).?(?:api|app|application|client|consumer|customer)?.?(?:id|identifier|key).{0,2}\s{0,20}.{0,2}\s{0,20}.{0,2}\b([a-z0-9]{20})\b"#,
    r#"(?i)github.?(?:api|app|application|client|consumer|customer|secret|key).?(?:key|oauth|sec|secret)?.{0,2}\s{0,20}.{0,2}\s{0,20}.{0,2}\b([a-z0-9]{40})\b"#,
    r#"\b(github_pat_[0-9a-zA-Z_]{82})\b"#,
    r#"\b(GR1348941[0-9a-zA-Z_-]{20})(?:\b|$)"#,
    r#"\b(glpat-[0-9a-zA-Z_-]{20})(?:\b|$)"#,
    r#"\b(glptt-[0-9a-f]{40})\b"#,
    r#"(?i)\b([0-9]+-[a-z0-9_]{32})\.apps\.googleusercontent\.com"#,
    r#"\b(GOCSPX-[a-zA-Z0-9_-]{28})(?:[^a-zA-Z0-9_-]|$)"#,
    r#"(?i)client.?secret.{0,10}\b([a-z0-9_-]{24})(?:[^a-z0-9_-]|$)"#,
    r#"\b(ya29\.[0-9A-Za-z_-]{20,1024})(?:[^0-9A-Za-z_-]|$)"#,
    r#"\b(AIza[0-9A-Za-z_-]{35})\b"#,
    r#"(?i)credentials\s*\{(?:\s*//.*)*\s*(?:username|password)\s+['"]([^'"]{1,60})['"](?:\s*//.*)*\s*(?:username|password)\s+['"]([^'"]{1,60})['"]"#,
    r#"\b(eyJrIjoi[A-Za-z0-9]{60,100})\b"#,
    r#"\b(glc_eyJrIjoi[A-Za-z0-9]{60,100})\b"#,
    r#"\b(glsa_[a-zA-Z0-9]{32}_[a-fA-F0-9]{8})\b"#,
    r#"(\$1\$[./A-Za-z0-9]{8}\$[./A-Za-z0-9]{22})"#,
    r#"(\$2[abxy]\$\d+\$[./A-Za-z0-9]{53})"#,
    r#"(?i)heroku.{0,20}key.{0,20}\b([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})\b"#,
    r#"(?i)jenkins.{0,10}(?:crumb)?.{0,10}\b([0-9a-f]{32,36})\b"#,
    r#"\b(ey[a-zA-Z0-9_-]+\.ey[a-zA-Z0-9_-]+\.[a-zA-Z0-9_-]+)(?:[^a-zA-Z0-9_-]|$)"#,
    r#"(?i)linkedin.?(?:api|app|application|client|consumer|customer)?.?(?:id|identifier|key).{0,2}\s{0,20}.{0,2}\s{0,20}.{0,2}\b([a-z0-9]{12,14})\b"#,
    r#"(?i)linkedin.?(?:api|app|application|client|consumer|customer|secret|key).?(?:key|oauth|sec|secret)?.{0,2}\s{0,20}.{0,2}\s{0,20}.{0,2}\b([a-z0-9]{16})\b"#,
    r#"(?i)(?:mailchimp|mc).{0,20}\b([a-f0-9]{32}-us[0-9]{1,3})\b"#,
    r#"(?i)(?:mailgun|mg).{0,20}key-([a-z0-9]{32})\b"#,
    r#"(?i)(?s)mapbox.{0,30}(pk\.[a-z0-9\-+/=]{32,128}\.[a-z0-9\-+/=]{20,30})(?:[^a-z0-9\-+/=]|$)"#,
    r#"(?i)(?s)mapbox.{0,30}([st]k\.[a-z0-9\-+/=]{32,128}\.[a-z0-9\-+/=]{20,30})(?:[^a-z0-9\-+/=]|$)"#,
    r#"(?i)outlook\.office\.com/webhook/([a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}@[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12})/IncomingWebhook/([a-f0-9]{32})/([a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12})"#,
    r#"(?:(machine\s+[^\s]+)|default)\s+login\s+([^\s]+)\s+password\s+([^\s]+)"#,
    r#"(?i)\b([a-z0-9]{6}[a-f0-9]{30}nral)\b"#,
    r#"(?i)associated with your New Relic account\.\s+license_key:\s*([a-f0-9]{40})\b"#,
    r#"(?i)\b(nrak-[a-z0-9]{27})\b"#,
    r#"(?i)\b(nraa-[a-f0-9]{27})\b"#,
    r#"(?i)\b(nrii-[a-z0-9_-]{32})(?:[^a-z0-9_-]|$)"#,
    r#"(?i)\b(nriq-[a-z0-9_-]{32})(?:[^a-z0-9_-]|$)"#,
    r#"(?i)\b(nrra-[a-f0-9]{42})\b"#,
    r#"(?i)\b(px-api-[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12})\b"#,
    r#"(?i)\b(px-dep-[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12})\b"#,
    r#"\b(npm_[A-Za-z0-9]{36})\b"#,
    r#"\b(oy2[a-z0-9]{43})\b"#,
    r#"(?i)(?:User|User Id|UserId|Uid)\s*=\s*([^\s;]{3,100})\s*;[ \t]*.{0,10}[ \t]*(?:Password|Pwd)\s*=\s*([^\s;]{3,100})\s*(?:[;"']|$)"#,
    r#"(?i)(?s)(?:okta|ssws).{0,40}\b(00[a-z0-9_-]{39})[a-z0-9_]\b"#,
    r#"\b(sk-[a-zA-Z0-9]{48})\b"#,
    r#"-----BEGIN .{0,20} ?PRIVATE KEY ?.{0,20}-----\s*((?:[a-zA-Z0-9+/=\s"',]|\r|\n){50,})\s*-----END .{0,20} ?PRIVATE KEY ?.{0,20}-----"#,
    r#"\b(PMAK-[a-zA-Z0-9]{24}-[a-zA-Z0-9]{34})\b"#,
    r#"(?i)psexec.{0,100}-u\s*(\S+)\s+-p\s*(\S+)"#,
    r#"\b(pypi-AgEIcHlwaS5vcmc[a-zA-Z0-9_-]{50,})(?:[^a-zA-Z0-9_-]|$)"#,
    r#"(?i)\b(rubygems_[a-f0-9]{48})\b"#,
    r#"(?i)sauce.{0,50}\b([a-f0-9-]{36})\b"#,
    r#"\b(sgp_[a-zA-Z0-9]{64})\b"#,
    r#"\b(SG\.[0-9A-Za-z_-]{22}\.[0-9A-Za-z_-]{43})\b"#,
    r#"\b((?:[a-zA-Z0-9-]+\.)*[a-zA-Z0-9-]+\.myshopify\.com)\b"#,
    r#"\b(shpss_[a-fA-F0-9]{32})\b"#,
    r#"\b(shpat_[a-fA-F0-9]{32})\b"#,
    r#"\b(shpca_[a-fA-F0-9]{32})\b"#,
    r#"\b(shppa_[a-fA-F0-9]{32})\b"#,
    r#"\b(xox[baprs]-[a-zA-Z0-9]{10,48})\b"#,
    r#"\b(xox[pboa]-[0-9]{12}-[0-9]{12}-[0-9]{12}-[a-z0-9]{32})\b"#,
    r#"(?i)https://hooks.slack.com/services/(T[a-z0-9_]{8}/B[a-z0-9_]{8,12}/[a-z0-9_]{24})"#,
    r#"(?i)sonar.{0,5}login.{0,5}\s*\b([a-f0-9]{40})\b"#,
    r#"(?i)\b(sq0atp-[a-z0-9_-]{22})\b"#,
    r#"(?i)\b(sq0csp-[a-z0-9_-]{43})\b"#,
    r#"\b(hawk\.[0-9A-Za-z_-]{20}\.[0-9A-Za-z_-]{20})\b"#,
    r#"(?i)\b((?:sk|rk)_live_[a-z0-9]{24})\b"#,
    r#"(?i)\b((?:sk|rk)_test_[a-z0-9]{24})\b"#,
    r#"\b(\d+:AA[a-zA-Z0-9_-]{32,33})(?:[^a-zA-Z0-9_-]|$)"#,
    r#"(?i)twilio.{0,20}\b(sk[a-f0-9]{32})\b"#,
    r#"(?i)\btwitter.?(?:api|app|application|client|consumer|customer)?.?(?:id|identifier|key).{0,2}\s{0,20}.{0,2}\s{0,20}.{0,2}\b([a-z0-9]{18,25})\b"#,
    r#"(?i)twitter.?(?:api|app|application|client|consumer|customer|secret|key).?(?:key|oauth|sec|secret)?.{0,2}\s{0,20}.{0,2}\s{0,20}.{0,2}\b([a-z0-9]{35,44})\b"#,
];

#[test]
fn test_noseyparker_full() {
    // Combine all patterns with | wrapping each in (?:...)
    let combined = PATTERNS
        .iter()
        .map(|p| format!("(?:{})", p))
        .collect::<Vec<_>>()
        .join("|");

    eprintln!("combined pattern length: {} chars", combined.len());
    eprintln!("number of alternatives: {}", PATTERNS.len());

    // Compile
    let re = match Regex::new(&combined) {
        Ok(r) => {
            eprintln!("compilation: SUCCESS");
            r
        }
        Err(e) => {
            eprintln!("compilation: FAILED - {}", e);
            panic!("regex compilation failed: {}", e);
        }
    };

    // Strategy
    eprintln!("strategy: {}", re.strategy_name());

    // Small text with a matching GitHub PAT
    let small_text = "here is a token ghp_abcdefghijklmnopqrstuvwxyz0123456789 in the text";
    let count_small = re.count_matches(small_text);
    eprintln!("count on small text (expect 1): {}", count_small);
    assert!(count_small >= 1, "expected at least 1 match on small text with ghp_ token");

    // Large non-matching text (~3MB)
    let chunk = "no match here at all xyz 1234 some random text that does not trigger any pattern ";
    let repeats = 3_000_000 / chunk.len();
    let big = chunk.repeat(repeats);
    eprintln!("haystack size: {} bytes ({:.1} MB)", big.len(), big.len() as f64 / 1_000_000.0);

    let count_big = re.count_matches(&big);
    eprintln!("count on 3MB non-matching text: {}", count_big);

    // Timing: 3 iterations on the non-matching text
    let start = std::time::Instant::now();
    for _ in 0..3 {
        let c = re.count_matches(&big);
        assert_eq!(c, count_big, "count should be consistent across iterations");
    }
    let elapsed = start.elapsed();
    eprintln!(
        "3 count_matches iterations on ~3MB: {:?} ({:.1} ns/byte)",
        elapsed,
        elapsed.as_nanos() as f64 / big.len() as f64 / 3.0
    );
}
