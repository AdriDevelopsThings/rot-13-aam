use crate::rot13::rot13;

macro_rules! test_rot_13 {
    ($message:tt, $cipher:tt) => {
        let ciphertext = rot13($message);
        assert_eq!(ciphertext, $cipher);
        assert_eq!(rot13(&ciphertext), $message);
    };
}

#[test]
fn some_rot13_tests() {
    test_rot_13!(
        "Hello World, this is an example text with an A and a Z.",
        "Uryyb Jbeyq, guvf vf na rknzcyr grkg jvgu na N naq n M."
    );

    test_rot_13!(
        "Rust is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency. It enforces memory safety—ensuring that all references point to valid memory—without requiring the use of a garbage collector or reference counting present in other memory-safe languages. To simultaneously enforce memory safety and prevent concurrent data races, its \"borrow checker\" tracks the object lifetime of all references in a program during compilation. Rust is popularized for systems programming but also has high-level features including some functional programming constructs.",
        "Ehfg vf n zhygv-cnenqvtz, trareny-checbfr cebtenzzvat ynathntr gung rzcunfvmrf cresbeznapr, glcr fnsrgl, naq pbapheerapl. Vg rasbeprf zrzbel fnsrgl—rafhevat gung nyy ersreraprf cbvag gb inyvq zrzbel—jvgubhg erdhvevat gur hfr bs n tneontr pbyyrpgbe be ersrerapr pbhagvat cerfrag va bgure zrzbel-fnsr ynathntrf. Gb fvzhygnarbhfyl rasbepr zrzbel fnsrgl naq cerirag pbapheerag qngn enprf, vgf \"obeebj purpxre\" genpxf gur bowrpg yvsrgvzr bs nyy ersreraprf va n cebtenz qhevat pbzcvyngvba. Ehfg vf cbchynevmrq sbe flfgrzf cebtenzzvat ohg nyfb unf uvtu-yriry srngherf vapyhqvat fbzr shapgvbany cebtenzzvat pbafgehpgf."
    );

    test_rot_13!(
        "abcdefghijklmnopqrstuvwxyz ABCDEFGHIJKLMNOPQRSTUVWXYZ 1234567890 ,.-;:_",
        "nopqrstuvwxyzabcdefghijklm NOPQRSTUVWXYZABCDEFGHIJKLM 1234567890 ,.-;:_"
    );

    test_rot_13!("🏳️‍🌈", "🏳️‍🌈");
}
