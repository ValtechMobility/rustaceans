use std::fs::File;
use std::io::{Write, BufReader};
use prj::dto::{Benutzer, Status};
use uuid::Uuid;

#[test]
fn test_writeDtoToHdd() {

    // Datenstruktur anlegen
    let user = Benutzer {
        id: Uuid::new_v4(),
        name: "Stephan".into(),
        email: "stephan@example.com".into(),
        ist_admin: true,
        tags: vec!["rustacean".into(), "dev".into()],
        status: Status::Aktiv,
    };

    // ✅ 1. In Datei schreiben (serialisieren)
    let mut file = File::create("benutzer.json");
    let json = serde_json::to_string_pretty(&user).expect("Fehler");
    // let json = serde_json::to_string(&user).expect("Fehler");
    file.expect("Dateifehler").write_all(json.as_bytes()).expect("Fehler");

    // ✅ 2. Aus Datei lesen (deserialisieren)
    let fileOp = File::open("benutzer.json").expect("Dateifehler");
    let reader = BufReader::new(fileOp);
    let loaded_user: Benutzer = serde_json::from_reader(reader).expect("Fehler");

    println!("\n✅ Aus Datei geladen:\n{:?}", loaded_user);

    assert_eq!(loaded_user.status, Status::Aktiv)

}