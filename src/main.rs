/*fn main() {
    println!("Hello, world!");
}*/

/*fn main() {
    println!("Hello, world!");
}*/
use std::fs::File;
use xml::writer::{EmitterConfig,XmlEvent};

// xml-rs = "0.8.0"
fn create_nested_nodes(
    depth: usize,
    _max_children: usize,
) -> Result<Vec<XmlEvent<'static>>, Box<dyn std::error::Error>> {
    if depth == 0 {
        return Ok(Vec::new());
    }
    let mut events = Vec::new();

    events.push(XmlEvent::start_element("ShippingNotification").into());

    let child_events = create_nested_nodes(depth - 1, 1)?;
    events.extend(child_events);
    events.push(XmlEvent::start_element("Header").into());

    let child_events = create_nested_nodes(depth - 1, 1)?;
    events.extend(child_events);
    events.push(XmlEvent::start_element("ShipmentType").into());
    events.push(XmlEvent::characters("ASN").into());
    events.push(XmlEvent::end_element().into());

    events.push(XmlEvent::start_element("DespatchNumber").into());
    events.push(XmlEvent::characters("2132").into());
    events.push(XmlEvent::end_element().into());

    //END Header
    events.push(XmlEvent::end_element().into());

    //================================================================================================

    let  _max_children = 2;

    for _ in 0.._max_children{

    let child_events = create_nested_nodes(depth - 1,_max_children)?;
    events.extend(child_events);
    events.push(XmlEvent::start_element("MaterialPositions").into());

    let child_events = create_nested_nodes(depth - 1, 1)?;
    events.extend(child_events);
    events.push(XmlEvent::start_element("MaterialPosition").into());

    let child_events = create_nested_nodes(depth - 1, 1)?;
    events.extend(child_events);
    events.push(XmlEvent::start_element("CustomerNumber").into());
    events.push(XmlEvent::characters("0938590740000").into());
    events.push(XmlEvent::end_element().into());

    let child_events = create_nested_nodes(depth - 1, 1)?;
    events.extend(child_events);
    events.push(XmlEvent::start_element("DeliveryNoteNumber").into());
    events.push(XmlEvent::characters("W22004035").into());
    events.push(XmlEvent::end_element().into());
//-
    let child_events = create_nested_nodes(depth - 1, 1)?;
    events.extend(child_events);
    events.push(XmlEvent::start_element("SinglePacks").into());

    let child_events = create_nested_nodes(depth - 1, 1)?;
    events.extend(child_events);
    events.push(XmlEvent::start_element("SinglePack").into());
//-
    events.push(XmlEvent::end_element().into());
    events.push(XmlEvent::end_element().into());
    events.push(XmlEvent::end_element().into());

    // Header
    events.push(XmlEvent::end_element().into());
}

    /*for _ in 0..max_children {
        let child_events = create_nested_nodes(depth - 1, max_children)?;
        events.extend(child_events);
        events.push(XmlEvent::start_element("Header1").into());
        events.push(XmlEvent::characters("Eigenschaftxx des Kindknotensxxx1").into());
        events.push(XmlEvent::end_element().into());
    }*/

    // Header
    events.push(XmlEvent::end_element().into());
    Ok(events)
}

fn create_dynamic_xml(depth: usize, max_children: usize) -> Result<(), Box<dyn std::error::Error>> {

    let file = File::create("output.xml")?;
    let mut writer = EmitterConfig::new().perform_indent(true).create_writer(file);

    writer.write(XmlEvent::start_element("ShippingNotifications"))?;

    let nested_nodes = create_nested_nodes(depth, max_children)?;
    for event in nested_nodes {
        writer.write(event)?;
    }
    writer.write(XmlEvent::end_element())?;

    Ok(())
}

fn main() {
    //TODO: "read the csv and the index and write this on the right nested XML tree"
    //let file = File::open("/home/xbias_user/CLionProjects/iCellsLab_XML_CSV_ReadWrite/test.csv").expect("Hallo");

    let file = File::open("C:/Daten/Development/LEA_XMLWRITE_AD_TRY_C/TEST.CSV").expect("Hallo");

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(file);
    for record_ in reader.records() {
        let record = record_.unwrap();
        println!("Hallo");
        let index = 0;
        let _xx = &record[index];
        let _xy = &record[1];
        println!("{}",_xx);
        println!("{}",_xy);
    }

    if let Err(err) = create_dynamic_xml(1, 2) {
        eprintln!("Fehler beim Erstellen der XML-Datei: {}", err);
        print!("hallo");
    }
}




