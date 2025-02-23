for combo in [
"ones",
"twos",
"threes",
"fours",
"fives",
"sixes",
"one_pair",
"two_pairs",
"three_of_a_kind",
"four_of_a_kind",
"small_straight",
"large_straight",
"full_house",
"chance",
"yatzy",
]:
    print(f"""\
        else if input == "{combo}" {{
            game.select_combo(Combo::{combo});
        }}""")
