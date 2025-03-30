{
  board.keymap_index_for_key,
  # = fun { column_index | Number, row_index | Number, .. } =>
  keymap_indices = {
    sw_1_1 = board.keymap_index_for_key { row_index = 0, column_index = 0 },
    sw_2_1 = board.keymap_index_for_key { row_index = 1, column_index = 0 },
    sw_3_1 = board.keymap_index_for_key { row_index = 2, column_index = 0 },
    sw_4_1 = board.keymap_index_for_key { row_index = 3, column_index = 0 },
    sw_5_1 = board.keymap_index_for_key { row_index = 4, column_index = 0 },

    sw_8_7 = board.keymap_index_for_key { row_index = 7, column_index = 6 },
    sw_4_8 = board.keymap_index_for_key { row_index = 3, column_index = 7 },
  },
}
