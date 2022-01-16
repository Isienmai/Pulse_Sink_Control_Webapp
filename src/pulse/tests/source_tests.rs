use crate::pulse::source::*;

#[test]
fn parse_from_pactl_list_output_empty_string_returns_none() {
    assert!(Source::parse_from_pactl_list_output("").is_none());
}

#[test]
fn parse_from_pactl_list_output_works_with_known_input() {
    let example_input = r#"
	State: SUSPENDED
	Name: alsa_output.usb-SteelSeries_SteelSeries_Arctis_1_Wireless-00.analog-stereo.monitor
	Description: Monitor of SteelSeries Arctis 1 Wireless Analogue Stereo
	Driver: module-alsa-card.c
	Sample Specification: s16le 2ch 44100Hz
	Channel Map: front-left,front-right
	Owner Module: 28
	Mute: no
	Volume: front-left: 65536 / 100% / 0.00 dB,   front-right: 65536 / 100% / 0.00 dB
	        balance 0.00
	Base Volume: 65536 / 100% / 0.00 dB
	Monitor of Sink: alsa_output.usb-SteelSeries_SteelSeries_Arctis_1_Wireless-00.analog-stereo
	Latency: 0 usec, configured 0 usec
	Flags: DECIBEL_VOLUME LATENCY 
	Properties:
		device.description = "Monitor of SteelSeries Arctis 1 Wireless Analogue Stereo"
		device.class = "monitor"
		alsa.card = "1"
		alsa.card_name = "SteelSeries Arctis 1 Wireless"
		alsa.long_card_name = "SteelSeries SteelSeries Arctis 1 Wireless at usb-0000:0b:00.3-2, full speed"
		alsa.driver_name = "snd_usb_audio"
		device.bus_path = "pci-0000:0b:00.3-usb-0:2:1.0"
		sysfs.path = "/devices/pci0000:00/0000:00:07.1/0000:0b:00.3/usb7/7-2/7-2:1.0/sound/card1"
		udev.id = "usb-SteelSeries_SteelSeries_Arctis_1_Wireless-00"
		device.bus = "usb"
		device.vendor.id = "1038"
		device.vendor.name = "SteelSeries ApS"
		device.product.id = "12b3"
		device.product.name = "SteelSeries Arctis 1 Wireless"
		device.serial = "SteelSeries_SteelSeries_Arctis_1_Wireless"
		device.string = "1"
		module-udev-detect.discovered = "1"
		device.icon_name = "audio-card-usb"
	Formats:
		pcm"#;
    let source = Source::parse_from_pactl_list_output(example_input).unwrap();

    assert_eq!(source.name, r#"alsa_output.usb-SteelSeries_SteelSeries_Arctis_1_Wireless-00.analog-stereo.monitor"#);
    assert_eq!(source.description, r#"Monitor of SteelSeries Arctis 1 Wireless Analogue Stereo"#);
}
