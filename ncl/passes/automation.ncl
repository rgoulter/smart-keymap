# Pass: fold automation instruction lists into a flat table + per-key refs
#
# Inputs (from driver merge): chorded_keys, keymap_ncl
# Outputs: automation_transform { automation_instructions, keys }
{
  chorded_keys,
  keymap_ncl,

  # One slot in `automation_transform.keys` after the instruction fold.
  #
  # Usually an ordinary keymap key (keyboard, null, layered, chorded, …). Automation
  # keys — e.g. `K.string_macro "a"` (`{ automation_instructions.on_press = […] }`
  # before this stage) — are rewritten to `automation::Key`: `automation_instructions`
  # holds `{ start, length }` execution refs into the accumulated instruction list.
  automation_key_element_validator = fun k =>
    k
    |> match {
      { automation_instructions, .. } =>
        keymap_ncl.automation.key_validator k,
      _ =>
        keymap_ncl.nullable_key.key_validator k,
    },

  AutomationKeyElement = std.contract.from_validator automation_key_element_validator,

  AutomationTransform = {
    automation_instructions | Array keymap_ncl.automation.Instruction,
    keys | Array AutomationKeyElement,
  },

  automation_transform
    | AutomationTransform
    | doc "Fold chorded keys: flat `{ automation_instructions, keys }` — config instruction list plus post-transform key slots."
    =
      chorded_keys
      |> std.array.fold_left
        (fun { automation_instructions = prior_instructions, keys = prior_keys } k =>
          let { acc, k } =
            keymap_ncl.key.map_tree
              keymap_ncl.automation.transform_keys_to_instructions
              { automation_instructions = prior_instructions }
              k
          in
          {
            automation_instructions = acc.automation_instructions,
            keys = prior_keys @ [k],
          }
        )
        { automation_instructions = [], keys = [] },

  checks.check_automation_key_element =
    let K = import "keys.ncl" in
    {
      check_plain_keyboard_ok =
        automation_key_element_validator K.A == 'Ok,

      check_string_macro_pre_transform_is_keymap_key =
        keymap_ncl.key.key_validator (K.string_macro "a") == 'Ok,

      check_string_macro_post_transform_ok =
        let exec = { start = 0, length = 1 } in
        automation_key_element_validator {
          automation_instructions = {
            on_press = exec,
            while_pressed = exec,
            on_release = exec,
          },
        } == 'Ok,
    },
}
