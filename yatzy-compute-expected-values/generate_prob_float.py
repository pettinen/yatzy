import itertools
from fractions import Fraction


def print_probabilities(n: int) -> None:
    probs = {}

    for dice in itertools.product(range(1, 7), repeat=n):
        dice = tuple(sorted(dice))
        probs.setdefault(dice, Fraction())
        probs[dice] += Fraction(1, 6 ** n)

    print(f"pub const ROLL_{n}_PROB: [([Die; {n}], f64); {len(probs)}] = [")
    for dice in sorted(probs):
        print(f"    ([{", ".join(str(die) for die in dice)}], {float(probs[dice])}),")
    print("];")


def main() -> None:
    print("use yatzy::Die;")
    print()
    print_probabilities(1)
    print()
    print_probabilities(2)
    print()
    print_probabilities(3)
    print()
    print_probabilities(4)
    print()
    print_probabilities(5)


if __name__ == "__main__":
    main()
