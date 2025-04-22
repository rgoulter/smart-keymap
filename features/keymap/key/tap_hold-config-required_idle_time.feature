Feature: TapHold Key (configure required_idle_time)

  The `required_idle_time` config for tap hold keys means that
   tap hold keys act as 'tap' if they are pressed before the required
   idle time has passed since the previous keymap input event (press/release).

  This helps prevent accidental 'resolved-as-hold' tap-hold key presses
   when typing quickly.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [ZMK's hold-tap keymap behaviors, require-prior-idle-ms](https://zmk.dev/docs/keymaps/behaviors/hold-tap#require-prior-idle-ms)

  - [FAK's complex hold-tap behaviors, Global quick tap](https://github.com/semickolon/fak?tab=readme-ov-file#complex-hold-tap-behaviors)

  Background:

    Let's demonstrate tap-hold "required_idle_time" behaviour
    using a keymap with a keyboard key, and a tap-hold key:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.tap_hold.required_idle_time = 100,
        keys = [
          K.A,
          K.B & K.hold K.LeftCtrl,
        ]
      }
      """

  Example: tap hold resolves as tap when pressed before required idle time

    Pressing a tap-hold key immediately resolves it as 'tap' when
     it's pressed after another key, within the required idle time.

    When the keymap registers the following input
      """
      [
        press (K.A),
        release (K.A),
      ]
      """
    And the keymap ticks 50 times
    And the keymap registers the following input
      """
      [
        press (K.B & K.hold K.LeftCtrl),
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.A),
        release (K.A),
        press (K.B),
      ]
      """

  Example: tap hold resolves as tap when tapped after required idle time

    Whereas, the tap-hold key behaves as usual if the keymap is
     idle for the required time.

    When the keymap registers the following input
      """
      [
        press (K.A),
        release (K.A),
      ]
      """
    And the keymap ticks 110 times
    And the keymap registers the following input
      """
      [
        press (K.B & K.hold K.LeftCtrl),
        release (K.B & K.hold K.LeftCtrl),
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.A),
        release (K.A),
        press (K.B),
        release (K.B),
      ]
      """


  Example: tap hold resolves as tap when tapped after required idle time

    When the keymap registers the following input
      """
      [
        press (K.A),
        release (K.A),
      ]
      """
    And the keymap ticks 110 times
    And the keymap registers the following input
      """
      [
        press (K.B & K.hold K.LeftCtrl),
      ]
      """
    And the keymap ticks 250 times
    Then the output should be equivalent to output from
      """
      [
        press (K.A),
        release (K.A),
        press (K.LeftCtrl),
      ]
      """
