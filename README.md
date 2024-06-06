# Micro AAC

**WARNING:** This software is NOT ready for everyday use.

---

Micro AAC is _Augmentative and Alternative Communication_ (AAC)
software optimized for small non-touchscreen devices.

The two most common AAC designs are a _grid of buttons_ or _text area with keyboard_.
Micro AAC takes the approach of allowing rapid entry of arbitrary letters
using a small number of buttons. It is designed to be relatively straightforward
to implement on anything with a d-pad, 5 buttons, and a TTS library.


### Design Goals

Guiding principles & design considerations:
- Make it quick to learn.
- Optimize for a small number (<10) of physical buttons.
- Make it relatively straightforward to port to new platforms.

### Test Hardware

The initial device I'm attempting to create an interface for combines these:

- [Waveshare 4 inch IPS LCD, 400x800, resistive touchscreen](https://www.waveshare.com/product/raspberry-pi/displays/lcd-oled/4inch-hdmi-lcd.htm) ($40)
- [5cm Mini HDMI to HDMI cable](https://www.amazon.com/gp/product/B0791ZYM3D/) ($14)
- Raspberry Pi Zero W ($15)

The total cost is ~$70, but does not have a battery.

This hardware was chosen not because it's a good choice, but because it was
laying on my desk when I started this project.

## License

Micro AAC is released under the MIT license.
