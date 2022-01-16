use crate::pulse::sink::*;

#[test]
fn parse_from_pactl_list_output_empty_string_returns_none() {
    assert!(Sink::parse_from_pactl_list_output("").is_none());
}

#[test]
fn parse_from_pactl_list_output_works_with_known_input() {
    let example_input = r#"
	State: IDLE
	Name: Recorded_Sink_Name
	Description: Recorded_Sink_Desc
	Driver: module-null-sink.c
	Sample Specification: s16le 2ch 44100Hz
	Channel Map: front-left,front-right
	Owner Module: 22
	Mute: no
	Volume: front-left: 65536 / 100% / 0.00 dB,   front-right: 65536 / 100% / 0.00 dB
	        balance 0.00
	Base Volume: 65536 / 100% / 0.00 dB
	Monitor Source: Recorded_Sink.monitor
	Latency: 10085 usec, configured 66666 usec
	Flags: DECIBEL_VOLUME LATENCY SET_FORMATS 
	Properties:
		device.description = "Recorded_Sink"
		device.class = "abstract"
		device.icon_name = "audio-card"
	Formats:
		pcm"#;
    let source = Sink::parse_from_pactl_list_output(example_input).unwrap();

    assert_eq!(source.name, r#"Recorded_Sink_Name"#);
    assert_eq!(source.description, r#"Recorded_Sink_Desc"#);
}
