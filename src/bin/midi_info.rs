extern crate standard_midi_file;
extern crate clap;

use clap::{App, Arg};
use std::path::Path;
use standard_midi_file::SMF;
use standard_midi_file::header::TimeScale;
use standard_midi_file::track::event::Event;
use std::io::BufReader;
use std::fs::File;
use std::collections::{HashSet, HashMap};
use std::cmp::max;

pub fn main() {
    let matches = App::new("Basic MIDI Info")
    .version("0.1")
    .author("Marime Gui")
    .about("Prints a few useful pieces of information about a .mid file")
    .arg(
        Arg::with_name("INPUT")
            .help("Input .mid file")
            .required(true)
            .index(1),
    )
    .get_matches();

    let input_str = matches.value_of("INPUT").unwrap();
    let input_path = Path::new(input_str);

    let smf = SMF::import(&mut BufReader::new(File::open(input_path).unwrap())).unwrap();

    println!("Midi Format {}", smf.header.format.get_value());
    println!("{} Tracks", smf.header.nb_tracks);
    match smf.header.time_division {
        TimeScale::TicksPerQuarterNote(tpqn) => println!("{} Ticks per Quarter Note", tpqn),
        TimeScale::SMPTECompatible(u, v) => println!("SMPTE {} {}", u, v),
    }

    let mut tempos = HashMap::new();
    let mut longest_time = 0;

    for (i, track) in smf.tracks.iter().enumerate() {
        println!("---------------------------");
        println!("Track {}", i);
        println!("{} bytes long", track.length);
        println!("{} events", track.track_events.len());
        let mut note_full_on = 0;
        let mut note_fake_on = 0;
        let mut note_off = 0;
        let mut unk_meta = 0;
        let mut channels = HashSet::new();
        let mut time = 0;
        for track_event in &track.track_events {
            time += track_event.delta_time.value;
            match &track_event.event {
                Event::NoteOff(n) => {
                    note_off += 1;
                    channels.insert(n.channel);
                }
                Event::NoteOn(n) => {
                    if n.velocity > 0 {
                        note_full_on += 1;
                    } else {
                        note_fake_on += 1;
                    }
                    channels.insert(n.channel);
                }
                Event::Tempo(t) => {
                    tempos.insert(time, t.value);
                }
                Event::SequenceTrackName(s) => println!("Name: {}", s.text),
                Event::UnknownMetaEvent(_) => unk_meta += 1,
                _ => {}
            }
        }
        longest_time = max(longest_time, time);
        println!("{} Real Note Ons, {} Fake Note Offs, {} Actual Note Offs", note_full_on, note_fake_on, note_off);
        println!("Channels: {:?}", channels);
        println!("{} Unknown Meta Events", unk_meta);
    }
    println!("-----------------------------------------");
    println!("Tempos: {:?}", tempos);
    println!("Longest Time: {}", longest_time);
}