// Definiert eine Funktion namens `types`.
pub fn types() {
    // Erstellt einen veränderbaren String mit dem Wert "Hello".
    let mut my_string = String::from("Hello");
    // Entfernt das letzte Zeichen des Strings und speichert es als Option<char>.
    let last_char: Option<char> = my_string.pop();
    // Druckt das letzte Zeichen, wenn es eines gibt. Sonst wird `None` gedruckt.
    println!("{:?}", last_char);

    // Annahme, dass email_str ein gültiger &str ist, den man in eine Option packen möchte.
    let email_str = "example@example.com";
    // Konvertiert &str zu String und packt es in Some, um es als Option<String> zu speichern.
    let email: Option<String> = Some(email_str.to_string());
    // Druckt die E-Mail, wenn sie vorhanden ist. In diesem Fall `Some("example@example.com")`.
    println!("{:?}", email);

    // Speichert den Wert `None` in einer Variable vom Typ Option<String>.
    let no_email: Option<String> = None;
    // Druckt `None`, da keine E-Mail vorhanden ist.
    println!("{:?}", no_email);

    // Definiert den enum Typ `Result` mit den Varianten `Ok` und `Err`.
    // In Rust ist `Result` bereits als Standard-Typ definiert, also ist diese Definition überflüssig.
    // Hier die korrekte Nutzung von `Result`:
    let success: Result<i32, String> = Ok(42);
    let failure: Result<i32, String> = Err("Ein Fehler ist aufgetreten".to_string());

    // Druckt das erfolgreiche Ergebnis und das Fehlschlag-Ergebnis.
    println!("{:?}", success);
    println!("{:?}", failure);
}
