use edsm::json;

fn main() {
    // XXX: Very slow.
    // let systems = json("edsm/dumps/systemsWithoutCoordinates.json");
    let systems = json("tests/systemsPopulated.json");
    // let systems = json("dumps/systemsPopulated.json");
    dbg!(&systems[5]);
}
