use super::*;

#[test]
fn parse_sources_known_input_returns_expected_output(){
    let known_input = r#"
Source #1
	State: SUSPENDED
	Name: alsa_output.usb-SteelSeries_SteelSeries_Arctis_1_Wireless-00.analog-stereo.monitor
	Description: Monitor of SteelSeries Arctis 1 Wireless Analogue Stereo
	Driver: module-alsa-card.c
	Sample Specification: s16le 2ch 44100Hz
	Channel Map: front-left,front-right
	Owner Module: 21
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

Source #2
	State: SUSPENDED
	Name: alsa_input.usb-SteelSeries_SteelSeries_Arctis_1_Wireless-00.mono-fallback
	Description: SteelSeries Arctis 1 Wireless Mono
	Driver: module-alsa-card.c
	Sample Specification: s16le 1ch 44100Hz
	Channel Map: mono
	Owner Module: 21
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

Source #3
	State: SUSPENDED
	Name: alsa_output.pci-0000_09_00.1.hdmi-stereo-extra1.monitor
	Description: Monitor of GK106 HDMI Audio Controller Digital Stereo (HDMI 2)
	Driver: module-alsa-card.c
	Sample Specification: s16le 2ch 44100Hz
	Channel Map: front-left,front-right
	Owner Module: 22
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
	State: IDLE
	Name: alsa_output.pci-0000_0c_00.3.analog-stereo.monitor
	Description: Monitor of Family 17h (Models 00h-0fh) HD Audio Controller Analogue Stereo
	Driver: module-alsa-card.c
	Sample Specification: s16le 2ch 44100Hz
	Channel Map: front-left,front-right
	Owner Module: 23
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
		pcm
"#;

    let sources = parse_sources(known_input);

    assert_eq!(sources.len(), 4);
}