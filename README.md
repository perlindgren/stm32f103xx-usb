stm32f103xx-usb
===============

[usb-device](https://github.com/mvirkkunen/usb-device) implementation for STM32F103
microcontrollers.

See the examples for an example of a custom class and a minimalistic USB serial port device.

## Blue Pill

[blue-pill](https://os.mbed.com/users/hudakz/code/STM32F103C8T6_Hello) dev board.

| Blue Pill  | Nucleo       |
| ---------- | ------------ |
| 1 GND      | 3 GND        |
| 2 DCLK     | 2 SWCLK      |
| 3 DIO      | 3 SWDATA     |
| 4 +3.3     | 1 VDD_Target |

## Programmer Using the stm32nucleo

CN4:Connector
Pin
- 1 VDD_Target
- 2 SWCLK
- 3 GND
- 4 SWDATA
- 5 NRST
- 6 SWO (ITM)

## Midified Focus fader.

r1 & r2 changed from 1kOhm to 100kOhm, to reduce the pull down current. 

| Blue Pill | Fader |
| --------- | ----- |
| GND       | 1 GND |
| PC14      | 2 CH1 |
| PC15      | 3 CH2 |
| +3.3      | 4 V+  |

## Programming

The Blue Pill needs external power, we get that from the USB interface.

``` shell
> openocd -f /usr/share/openocd/scripts/interface/stlink.cfg -f /usr/share/openocd/scripts/target/stm32f1x.cfg
```

``` shell
> arm-none-eabi-gdb -x gdb.cfg -f target/thumbv7m-none-eabi/release/examples/fader
```

When debugger connected it will halt two times (`c` for continue in gdb). You may need to disconnect midi device after sleep.

## Linux Midi

See e.g., [alsa](http://tedfelix.com/linux/linux-midi.html) for some information.

List ALSA midi devices.

``` shell
> amidi -l
Dir Device    Name
IO  hw:2,0,0  MIDI Device MIDI 1
```

``` shell
> aconnect -i
client 0: 'System' [type=kernel]
    0 'Timer           '
    1 'Announce        '
client 14: 'Midi Through' [type=kernel]
    0 'Midi Through Port-0'
client 24: 'MIDI Device' [type=kernel,card=2]
    0 'MIDI Device MIDI 1'
...
```


Dump ALSA midi device data.

``` shell
> aseqdump -p <NR>
```
