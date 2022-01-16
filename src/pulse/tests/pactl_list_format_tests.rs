use crate::pulse::pactl_list_format::*;

#[test]
fn parse_single_object_empty_string_returns_empty_hashmap() {
    let object = parse_single_object("");
    assert_eq!(object.len(), 0);
}

#[test]
fn parse_pactl_list_format_object_known_input_returns_expected_output() {
    let example_input = r#"
	State: RUNNING
	Name: alsa_output.pci-0000_0c_00.3.analog-stereo
	Description: Family 17h (Models 00h-0fh) HD Audio Controller Analogue Stereo
	Driver: module-alsa-card.c
	Sample Specification: s16le 2ch 44100Hz
	Channel Map: front-left,front-right
	Owner Module: 26
	Mute: no
	Volume: front-left: 65532 / 100% / -0.00 dB,   front-right: 65532 / 100% / -0.00 dB
	        balance 0.00
	Base Volume: 65536 / 100% / 0.00 dB
	Monitor Source: alsa_output.pci-0000_0c_00.3.analog-stereo.monitor
	Latency: 15567 usec, configured 23220 usec
	Flags: HARDWARE HW_MUTE_CTRL HW_VOLUME_CTRL DECIBEL_VOLUME LATENCY 
	Properties:
		alsa.resolution_bits = "16"
		device.api = "alsa"
		device.class = "sound"
		alsa.class = "generic"
		alsa.subclass = "generic-mix"
		alsa.name = "ALC1220 Analog"
		alsa.id = "ALC1220 Analog"
		alsa.subdevice = "0"
		alsa.subdevice_name = "subdevice #0"
		alsa.device = "0"
		alsa.card = "2"
		alsa.card_name = "HD-Audio Generic"
		alsa.long_card_name = "HD-Audio Generic at 0xf7b00000 irq 129"
		alsa.driver_name = "snd_hda_intel"
		device.bus_path = "pci-0000:0c:00.3"
		sysfs.path = "/devices/pci0000:00/0000:00:08.1/0000:0c:00.3/sound/card2"
		device.bus = "pci"
		device.vendor.id = "1022"
		device.vendor.name = "Advanced Micro Devices, Inc. [AMD]"
		device.product.id = "1457"
		device.product.name = "Family 17h (Models 00h-0fh) HD Audio Controller"
		device.string = "front:2"
		device.buffering.buffer_size = "352768"
		device.buffering.fragment_size = "176384"
		device.access_mode = "mmap+timer"
		device.profile.name = "analog-stereo"
		device.profile.description = "Analogue Stereo"
		device.description = "Family 17h (Models 00h-0fh) HD Audio Controller Analogue Stereo"
		module-udev-detect.discovered = "1"
		device.icon_name = "audio-card-pci"
	Ports:
		analog-output-lineout: Line Out (priority: 9000, available)
		analog-output-headphones: Headphones (priority: 9900, not available)
	Active Port: analog-output-lineout
	Formats:
		pcm"#;

    let object = parse_single_object(example_input);

    assert_eq!(object.len(), 17);
    assert_eq!(object["State"], "RUNNING");
    assert_eq!(object["Sample Specification"], "s16le 2ch 44100Hz");
    assert_eq!(object["Volume"], 
    r#"front-left: 65532 / 100% / -0.00 dB,   front-right: 65532 / 100% / -0.00 dB
	        balance 0.00"#);
    assert_eq!(object["Base Volume"], "65536 / 100% / 0.00 dB");
    assert_eq!(object["Flags"], "HARDWARE HW_MUTE_CTRL HW_VOLUME_CTRL DECIBEL_VOLUME LATENCY ");
    assert_eq!(object["Active Port"], "analog-output-lineout");
    assert_eq!(object["Ports"], 
    r#"
		analog-output-lineout: Line Out (priority: 9000, available)
		analog-output-headphones: Headphones (priority: 9900, not available)"#);
    assert_eq!(object["Formats"], 
	r#"
		pcm"#);
}
