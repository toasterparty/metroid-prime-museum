// this code is out of context - it goes in resource_tracing.rs

let mut remove: Vec<u32> = Vec::new();


for obj in scly_layer.objects.iter() {
    if let Some(actor) = obj.property_data.as_actor() {
        if actor.cmdl == 0x5391EDB6 || actor.cmdl == 0x6E5D6796 {
            remove.push(obj.instance_id & 0x00FFFFFF);
        }
        continue;
    }

    if let Some(streamed_audio) = obj.property_data.as_streamed_audio() {
        let name = streamed_audio
            .audio_file_name
            .to_str()
            .ok()
            .unwrap()
            .to_string()
            .to_lowercase();

        for skip_name in vec![
            "cra_mprime1",
            "cra_mprime2",
            "gen_ShortBattle",
            "ice_thardus",
            "int_escape",
            "int_parasitequeen",
            "min_omegapirate",
            "pir_battle",
            "pir_isogi",
            "rui_flaaghra",
            "rui_hivetotem",
        ] {
            if name.contains(&skip_name.to_lowercase()) {
                remove.push(obj.instance_id & 0x00FFFFFF);
                break;
            }
        }

        continue;
    }

    if vec![
        0xE,
        0x16,
        0x21,
        0x24,
        0x25,
        0x26,
        0x27,
        0x28,
        0x2D,
        0x2E,
        0x36,
        0x37,
        0x3B,
        0x3D,
        0x43,
        0x44,
        0x4B,
        0x4D,
        0x54,
        0x58,
        0x5A,
        0x5C,
        0x5F,
        0x64,
        0x66,
        0x67,
        0x6B,
        0x6D,
        0x6E,
        0x6F,
        0x70,
        0x72,
        0x77,
        0x78,
        0x79,
        0x7A,
        0x7C,
        0x7F,
        0x86,
    ].contains(&obj.property_data.object_type()) {
        remove.push(obj.instance_id & 0x00FFFFFF);
    }
}

if !remove.is_empty() {
    let name =  name.replace("\0", "");
    println!("{}:{}:{:?}", f, name, remove);
}