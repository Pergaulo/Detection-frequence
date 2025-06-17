use std::collections::HashMap;

use midly::TrackEvent;
use midly::TrackEventKind;

use rand::thread_rng;
use rand::Rng;


pub fn generate_melody_mid(style:String,metric:u16) {

    let path = format!("../../../src/sounds/MID/{}.mid",style);

    // Load bytes first
    let data = std::fs::read(path).unwrap();

    // Parse the raw bytes
    let mut smf = midly::Smf::parse(&data).unwrap();

    let mut metric1: midly::Timing;
    if metric == 0 {
        dbg!(smf.header.timing);
        metric1 = smf.header.timing;
    }
    else {
        metric1 = midly::Timing::Metrical(metric.into());
    }


    //Transition matrix (key,delta,vel), Vec<(key_to_follow,delta,vel)>
    //let mut transition_matrix:HashMap<(u8,u32,u8), Vec<(u8,u32,u8)>> = HashMap::new();

    let mut transition_matrix2:HashMap<midly::TrackEvent, Vec<midly::TrackEvent>> = HashMap::new();
    let mut transition_matrix3:HashMap<midly::TrackEvent, Vec<midly::TrackEvent>> = HashMap::new();


    for i in 0..smf.tracks[1].len()-1 {
        if !transition_matrix2.contains_key(&smf.tracks[1][i]) {
            transition_matrix2.insert(smf.tracks[1][i], vec![smf.tracks[1][i+1]]);
        }
        else {
            if let Some(value) = transition_matrix2.get_mut(&smf.tracks[1][i]) {
                value.push(smf.tracks[1][i+1]);
            }
        }
    }

    if smf.tracks.len() > 2 {
        for i in 0..smf.tracks[2].len()-1 {
            if !transition_matrix3.contains_key(&smf.tracks[2][i]) {
                transition_matrix3.insert(smf.tracks[2][i], vec![smf.tracks[2][i+1]]);
            }
            else {
                if let Some(value) = transition_matrix3.get_mut(&smf.tracks[2][i]) {
                    value.push(smf.tracks[2][i+1]);
                }
            }
        }
    }

    let mut matrix_vec2 = vec![];
    let mut matrix_vec3 = vec![];

    for (k,v) in transition_matrix2.iter() {
        matrix_vec2.push((k,v));
    }

    if smf.tracks.len() > 2 {
        for (k,v) in transition_matrix3.iter() {
            matrix_vec3.push((k,v));
        }    
    }

    let mut rd = thread_rng();
    let (mut note,mut next) = matrix_vec2[rd.gen_range(0..transition_matrix2.len())];
    

    let mut note2:&TrackEvent = &note.clone();
    let mut next2:&Vec<TrackEvent>;
    
    if smf.tracks.len() > 2 {
        (note2,next2) = matrix_vec3[rd.gen_range(0..transition_matrix3.len())];
    }
    
    let mut track1 = midly::Track::new();
    let mut track2 = midly::Track::new();
    let mut n = 200;

    while n > 0 {

        track1.push(*note);

        if smf.tracks.len() > 2 {
            track2.push(*note2);
        }

        while transition_matrix2.get(note).is_none() {
            println!("None");
            note = matrix_vec2[rd.gen_range(0..transition_matrix2.len())].0;
        }
        note = &transition_matrix2.get(note).unwrap()[rd.gen_range(0..transition_matrix2[&note].len())];

        if smf.tracks.len() > 2 {
            while transition_matrix3.get(note2).is_none() {
                note2 = matrix_vec3[rd.gen_range(0..transition_matrix3.len())].0;
            }
            note2 = &transition_matrix3.get(note2).unwrap()[rd.gen_range(0..transition_matrix3[&note2].len())];
        }

        n -= 1;
    }

    let end_kind = TrackEventKind::Meta(midly::MetaMessage::EndOfTrack);
    let end = TrackEvent{kind:end_kind,delta:0.into()};
    track1.push(end);
    track2.push(end);

    let mut tracks = vec![track1];

    if smf.tracks.len() > 2 {
        println!("Two Tracks");
        //tracks.push(track2);
    }


    let smf = midly::Smf {
        header: midly::Header {
            format: midly::Format::SingleTrack,
            timing: metric1, //80 //1024 //192 //145(ode) //256
        },
        tracks: tracks,
    };

    smf.save("generated.mid").unwrap();
}


// POC 80

//BT 140

//ODE Defaut or 230