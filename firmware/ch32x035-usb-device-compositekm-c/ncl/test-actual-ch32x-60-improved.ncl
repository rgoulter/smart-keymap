{
  board.keymap_index_for_key,
  keymap_indices = {
    # Row 1: k101..k114 → 0..13
    sw_1_1 = board.keymap_index_for_key { row_index = 0, column_index = 0 },
    sw_1_14 = board.keymap_index_for_key { row_index = 0, column_index = 13 },

    # Row 2: k201..k214 → 14..27
    sw_2_1 = board.keymap_index_for_key { row_index = 1, column_index = 0 },
    sw_2_14 = board.keymap_index_for_key { row_index = 1, column_index = 13 },

    # Row 3: k301..k313 → 28..40; hole C14
    sw_3_1 = board.keymap_index_for_key { row_index = 2, column_index = 0 },
    sw_3_13 = board.keymap_index_for_key { row_index = 2, column_index = 12 },
    sw_3_14 = board.keymap_index_for_key { row_index = 2, column_index = 13 },

    # Row 4: k401..k412 → 41..52; holes C13-C14
    sw_4_1 = board.keymap_index_for_key { row_index = 3, column_index = 0 },
    sw_4_12 = board.keymap_index_for_key { row_index = 3, column_index = 11 },
    sw_4_13 = board.keymap_index_for_key { row_index = 3, column_index = 12 },
    sw_4_14 = board.keymap_index_for_key { row_index = 3, column_index = 13 },

    # Row 5: k501..k513 → 53..65; hole C14
    sw_5_1 = board.keymap_index_for_key { row_index = 4, column_index = 0 },
    sw_5_13 = board.keymap_index_for_key { row_index = 4, column_index = 12 },
    sw_5_14 = board.keymap_index_for_key { row_index = 4, column_index = 13 },
  },
}
