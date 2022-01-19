use crate::pulse::pactl_list_format::*;

#[test]
fn parse_single_object_empty_string_returns_empty_hashmap() {
    let object = parse_single_object("");
    assert_eq!(object.property_count(), 0);
}

#[test]
fn parse_single_object_known_input_returns_expected_output() {
    let example_input = r#"
Sink #3
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

    assert_eq!(object.property_count(), 17);
	assert_eq!(object.name, "Sink");
	assert_eq!(object.index, 3);
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

#[test]
fn parse_pactl_list_output_empty_string_input_returns_empty_list() {
    let objects = parse_pactl_list_output("");
    assert_eq!(objects.len(), 0);
}

#[test]
fn parse_pactl_list_output_known_input_returns_expected_output() {
	let known_input = r#"
Source #1
	State: SUSPENDED
	Name: Combined_Output.monitor
	Description: Monitor of Combined_Output
	Driver: module-null-sink.c
	Sample Specification: s16le 2ch 44100Hz
	Channel Map: front-left,front-right
	Owner Module: 21
	Mute: no
	Volume: front-left: 65536 / 100% / 0.00 dB,   front-right: 65536 / 100% / 0.00 dB
	        balance 0.00
	Base Volume: 65536 / 100% / 0.00 dB
	Monitor of Sink: Combined_Output
	Latency: 0 usec, configured 0 usec
	Flags: DECIBEL_VOLUME LATENCY 
	Properties:
		device.description = "Monitor of Combined_Output"
		device.class = "monitor"
		device.icon_name = "audio-input-microphone"
	Formats:
		pcm

Source #2
	State: RUNNING
	Name: Recorded_Sink.monitor
	Description: Monitor of Recorded_Sink
	Driver: module-null-sink.c
	Sample Specification: s16le 2ch 44100Hz
	Channel Map: front-left,front-right
	Owner Module: 22
	Mute: no
	Volume: front-left: 65536 / 100% / 0.00 dB,   front-right: 65536 / 100% / 0.00 dB
	        balance 0.00
	Base Volume: 65536 / 100% / 0.00 dB
	Monitor of Sink: Recorded_Sink
	Latency: 0 usec, configured 66666 usec
	Flags: DECIBEL_VOLUME LATENCY 
	Properties:
		device.description = "Monitor of Recorded_Sink"
		device.class = "monitor"
		device.icon_name = "audio-input-microphone"
	Formats:
		pcm

Source #3
	State: SUSPENDED
	Name: alsa_output.pci-0000_09_00.1.hdmi-stereo-extra1.monitor
	Description: Monitor of GK106 HDMI Audio Controller Digital Stereo (HDMI 2)
	Driver: module-alsa-card.c
	Sample Specification: s16le 2ch 44100Hz
	Channel Map: front-left,front-right
	Owner Module: 24
	Mute: no
	Volume: front-left: 65536 / 100% / 0.00 dB,   front-right: 65536 / 100% / 0.00 dB
	        balance 0.00
	Base Volume: 65536 / 100% / 0.00 dB
	Monitor of Sink: alsa_output.pci-0000_09_00.1.hdmi-stereo-extra1
	Latency: 0 usec, configured 0 usec
	Flags: DECIBEL_VOLUME LATENCY 
	Properties:
		device.description = "Monitor of GK106 HDMI Audio Controller Digital Stereo (HDMI 2)"
		device.class = "monitor"
		alsa.card = "0"
		alsa.card_name = "HDA NVidia"
		alsa.long_card_name = "HDA NVidia at 0xf7080000 irq 127"
		alsa.driver_name = "snd_hda_intel"
		device.bus_path = "pci-0000:09:00.1"
		sysfs.path = "/devices/pci0000:00/0000:00:03.1/0000:09:00.1/sound/card0"
		device.bus = "pci"
		device.vendor.id = "10de"
		device.vendor.name = "NVIDIA Corporation"
		device.product.id = "0e0b"
		device.product.name = "GK106 HDMI Audio Controller"
		device.string = "0"
		module-udev-detect.discovered = "1"
		device.icon_name = "audio-card-pci"
	Formats:
		pcm

Source #4
	State: SUSPENDED
	Name: alsa_output.usb-SteelSeries_SteelSeries_Arctis_1_Wireless-00.analog-stereo.monitor
	Description: Monitor of SteelSeries Arctis 1 Wireless Analogue Stereo
	Driver: module-alsa-card.c
	Sample Specification: s16le 2ch 44100Hz
	Channel Map: front-left,front-right
	Owner Module: 25
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
		pcm

Source #5
	State: SUSPENDED
	Name: alsa_input.usb-SteelSeries_SteelSeries_Arctis_1_Wireless-00.mono-fallback
	Description: SteelSeries Arctis 1 Wireless Mono
	Driver: module-alsa-card.c
	Sample Specification: s16le 1ch 44100Hz
	Channel Map: mono
	Owner Module: 25
	Mute: no
	Volume: mono: 65536 / 100% / 0.00 dB
	        balance 0.00
	Base Volume: 52057 /  79% / -6.00 dB
	Monitor of Sink: n/a
	Latency: 0 usec, configured 0 usec
	Flags: HARDWARE HW_MUTE_CTRL HW_VOLUME_CTRL DECIBEL_VOLUME LATENCY 
	Properties:
		alsa.resolution_bits = "16"
		device.api = "alsa"
		device.class = "sound"
		alsa.class = "generic"
		alsa.subclass = "generic-mix"
		alsa.name = "USB Audio"
		alsa.id = "USB Audio"
		alsa.subdevice = "0"
		alsa.subdevice_name = "subdevice #0"
		alsa.device = "0"
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
		device.string = "hw:1"
		device.buffering.buffer_size = "176400"
		device.buffering.fragment_size = "88200"
		device.access_mode = "mmap+timer"
		device.profile.name = "mono-fallback"
		device.profile.description = "Mono"
		device.description = "SteelSeries Arctis 1 Wireless Mono"
		module-udev-detect.discovered = "1"
		device.icon_name = "audio-card-usb"
	Ports:
		analog-input-mic: Microphone (priority: 8700)
	Active Port: analog-input-mic
	Formats:
		pcm

Source #6
	State: IDLE
	Name: alsa_output.pci-0000_0c_00.3.analog-stereo.monitor
	Description: Monitor of Family 17h (Models 00h-0fh) HD Audio Controller Analogue Stereo
	Driver: module-alsa-card.c
	Sample Specification: s16le 2ch 44100Hz
	Channel Map: front-left,front-right
	Owner Module: 26
	Mute: no
	Volume: front-left: 65536 / 100% / 0.00 dB,   front-right: 65536 / 100% / 0.00 dB
	        balance 0.00
	Base Volume: 65536 / 100% / 0.00 dB
	Monitor of Sink: alsa_output.pci-0000_0c_00.3.analog-stereo
	Latency: 0 usec, configured 1999818 usec
	Flags: DECIBEL_VOLUME LATENCY 
	Properties:
		device.description = "Monitor of Family 17h (Models 00h-0fh) HD Audio Controller Analogue Stereo"
		device.class = "monitor"
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
		device.string = "2"
		module-udev-detect.discovered = "1"
		device.icon_name = "audio-card-pci"
	Formats:
		pcm"#;
	
	let objects = parse_pactl_list_output(known_input);

    assert_eq!(objects.len(), 6);
	for (index, object) in objects.iter().enumerate(){
		assert_eq!(object.name, "Source");
		assert_eq!(object.index, index + 1);
	}
}