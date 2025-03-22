from __future__ import annotations
from dataclasses import dataclass, replace
from fractions import Fraction
from itertools import combinations
from typing import Literal


ROLL_1_PROB = [
    ((1,), Fraction(1, 6)),
    ((2,), Fraction(1, 6)),
    ((3,), Fraction(1, 6)),
    ((4,), Fraction(1, 6)),
    ((5,), Fraction(1, 6)),
    ((6,), Fraction(1, 6)),
]

ROLL_2_PROB = [
    ((1, 1), Fraction(1, 36)),
    ((1, 2), Fraction(1, 18)),
    ((1, 3), Fraction(1, 18)),
    ((1, 4), Fraction(1, 18)),
    ((1, 5), Fraction(1, 18)),
    ((1, 6), Fraction(1, 18)),
    ((2, 2), Fraction(1, 36)),
    ((2, 3), Fraction(1, 18)),
    ((2, 4), Fraction(1, 18)),
    ((2, 5), Fraction(1, 18)),
    ((2, 6), Fraction(1, 18)),
    ((3, 3), Fraction(1, 36)),
    ((3, 4), Fraction(1, 18)),
    ((3, 5), Fraction(1, 18)),
    ((3, 6), Fraction(1, 18)),
    ((4, 4), Fraction(1, 36)),
    ((4, 5), Fraction(1, 18)),
    ((4, 6), Fraction(1, 18)),
    ((5, 5), Fraction(1, 36)),
    ((5, 6), Fraction(1, 18)),
    ((6, 6), Fraction(1, 36)),
]

ROLL_3_PROB = [
    ((1, 1, 1), Fraction(1, 216)),
    ((1, 1, 2), Fraction(1, 72)),
    ((1, 1, 3), Fraction(1, 72)),
    ((1, 1, 4), Fraction(1, 72)),
    ((1, 1, 5), Fraction(1, 72)),
    ((1, 1, 6), Fraction(1, 72)),
    ((1, 2, 2), Fraction(1, 72)),
    ((1, 2, 3), Fraction(1, 36)),
    ((1, 2, 4), Fraction(1, 36)),
    ((1, 2, 5), Fraction(1, 36)),
    ((1, 2, 6), Fraction(1, 36)),
    ((1, 3, 3), Fraction(1, 72)),
    ((1, 3, 4), Fraction(1, 36)),
    ((1, 3, 5), Fraction(1, 36)),
    ((1, 3, 6), Fraction(1, 36)),
    ((1, 4, 4), Fraction(1, 72)),
    ((1, 4, 5), Fraction(1, 36)),
    ((1, 4, 6), Fraction(1, 36)),
    ((1, 5, 5), Fraction(1, 72)),
    ((1, 5, 6), Fraction(1, 36)),
    ((1, 6, 6), Fraction(1, 72)),
    ((2, 2, 2), Fraction(1, 216)),
    ((2, 2, 3), Fraction(1, 72)),
    ((2, 2, 4), Fraction(1, 72)),
    ((2, 2, 5), Fraction(1, 72)),
    ((2, 2, 6), Fraction(1, 72)),
    ((2, 3, 3), Fraction(1, 72)),
    ((2, 3, 4), Fraction(1, 36)),
    ((2, 3, 5), Fraction(1, 36)),
    ((2, 3, 6), Fraction(1, 36)),
    ((2, 4, 4), Fraction(1, 72)),
    ((2, 4, 5), Fraction(1, 36)),
    ((2, 4, 6), Fraction(1, 36)),
    ((2, 5, 5), Fraction(1, 72)),
    ((2, 5, 6), Fraction(1, 36)),
    ((2, 6, 6), Fraction(1, 72)),
    ((3, 3, 3), Fraction(1, 216)),
    ((3, 3, 4), Fraction(1, 72)),
    ((3, 3, 5), Fraction(1, 72)),
    ((3, 3, 6), Fraction(1, 72)),
    ((3, 4, 4), Fraction(1, 72)),
    ((3, 4, 5), Fraction(1, 36)),
    ((3, 4, 6), Fraction(1, 36)),
    ((3, 5, 5), Fraction(1, 72)),
    ((3, 5, 6), Fraction(1, 36)),
    ((3, 6, 6), Fraction(1, 72)),
    ((4, 4, 4), Fraction(1, 216)),
    ((4, 4, 5), Fraction(1, 72)),
    ((4, 4, 6), Fraction(1, 72)),
    ((4, 5, 5), Fraction(1, 72)),
    ((4, 5, 6), Fraction(1, 36)),
    ((4, 6, 6), Fraction(1, 72)),
    ((5, 5, 5), Fraction(1, 216)),
    ((5, 5, 6), Fraction(1, 72)),
    ((5, 6, 6), Fraction(1, 72)),
    ((6, 6, 6), Fraction(1, 216)),
]

ROLL_4_PROB = [
    ((1, 1, 1, 1), Fraction(1, 1296)),
    ((1, 1, 1, 2), Fraction(1, 324)),
    ((1, 1, 1, 3), Fraction(1, 324)),
    ((1, 1, 1, 4), Fraction(1, 324)),
    ((1, 1, 1, 5), Fraction(1, 324)),
    ((1, 1, 1, 6), Fraction(1, 324)),
    ((1, 1, 2, 2), Fraction(1, 216)),
    ((1, 1, 2, 3), Fraction(1, 108)),
    ((1, 1, 2, 4), Fraction(1, 108)),
    ((1, 1, 2, 5), Fraction(1, 108)),
    ((1, 1, 2, 6), Fraction(1, 108)),
    ((1, 1, 3, 3), Fraction(1, 216)),
    ((1, 1, 3, 4), Fraction(1, 108)),
    ((1, 1, 3, 5), Fraction(1, 108)),
    ((1, 1, 3, 6), Fraction(1, 108)),
    ((1, 1, 4, 4), Fraction(1, 216)),
    ((1, 1, 4, 5), Fraction(1, 108)),
    ((1, 1, 4, 6), Fraction(1, 108)),
    ((1, 1, 5, 5), Fraction(1, 216)),
    ((1, 1, 5, 6), Fraction(1, 108)),
    ((1, 1, 6, 6), Fraction(1, 216)),
    ((1, 2, 2, 2), Fraction(1, 324)),
    ((1, 2, 2, 3), Fraction(1, 108)),
    ((1, 2, 2, 4), Fraction(1, 108)),
    ((1, 2, 2, 5), Fraction(1, 108)),
    ((1, 2, 2, 6), Fraction(1, 108)),
    ((1, 2, 3, 3), Fraction(1, 108)),
    ((1, 2, 3, 4), Fraction(1, 54)),
    ((1, 2, 3, 5), Fraction(1, 54)),
    ((1, 2, 3, 6), Fraction(1, 54)),
    ((1, 2, 4, 4), Fraction(1, 108)),
    ((1, 2, 4, 5), Fraction(1, 54)),
    ((1, 2, 4, 6), Fraction(1, 54)),
    ((1, 2, 5, 5), Fraction(1, 108)),
    ((1, 2, 5, 6), Fraction(1, 54)),
    ((1, 2, 6, 6), Fraction(1, 108)),
    ((1, 3, 3, 3), Fraction(1, 324)),
    ((1, 3, 3, 4), Fraction(1, 108)),
    ((1, 3, 3, 5), Fraction(1, 108)),
    ((1, 3, 3, 6), Fraction(1, 108)),
    ((1, 3, 4, 4), Fraction(1, 108)),
    ((1, 3, 4, 5), Fraction(1, 54)),
    ((1, 3, 4, 6), Fraction(1, 54)),
    ((1, 3, 5, 5), Fraction(1, 108)),
    ((1, 3, 5, 6), Fraction(1, 54)),
    ((1, 3, 6, 6), Fraction(1, 108)),
    ((1, 4, 4, 4), Fraction(1, 324)),
    ((1, 4, 4, 5), Fraction(1, 108)),
    ((1, 4, 4, 6), Fraction(1, 108)),
    ((1, 4, 5, 5), Fraction(1, 108)),
    ((1, 4, 5, 6), Fraction(1, 54)),
    ((1, 4, 6, 6), Fraction(1, 108)),
    ((1, 5, 5, 5), Fraction(1, 324)),
    ((1, 5, 5, 6), Fraction(1, 108)),
    ((1, 5, 6, 6), Fraction(1, 108)),
    ((1, 6, 6, 6), Fraction(1, 324)),
    ((2, 2, 2, 2), Fraction(1, 1296)),
    ((2, 2, 2, 3), Fraction(1, 324)),
    ((2, 2, 2, 4), Fraction(1, 324)),
    ((2, 2, 2, 5), Fraction(1, 324)),
    ((2, 2, 2, 6), Fraction(1, 324)),
    ((2, 2, 3, 3), Fraction(1, 216)),
    ((2, 2, 3, 4), Fraction(1, 108)),
    ((2, 2, 3, 5), Fraction(1, 108)),
    ((2, 2, 3, 6), Fraction(1, 108)),
    ((2, 2, 4, 4), Fraction(1, 216)),
    ((2, 2, 4, 5), Fraction(1, 108)),
    ((2, 2, 4, 6), Fraction(1, 108)),
    ((2, 2, 5, 5), Fraction(1, 216)),
    ((2, 2, 5, 6), Fraction(1, 108)),
    ((2, 2, 6, 6), Fraction(1, 216)),
    ((2, 3, 3, 3), Fraction(1, 324)),
    ((2, 3, 3, 4), Fraction(1, 108)),
    ((2, 3, 3, 5), Fraction(1, 108)),
    ((2, 3, 3, 6), Fraction(1, 108)),
    ((2, 3, 4, 4), Fraction(1, 108)),
    ((2, 3, 4, 5), Fraction(1, 54)),
    ((2, 3, 4, 6), Fraction(1, 54)),
    ((2, 3, 5, 5), Fraction(1, 108)),
    ((2, 3, 5, 6), Fraction(1, 54)),
    ((2, 3, 6, 6), Fraction(1, 108)),
    ((2, 4, 4, 4), Fraction(1, 324)),
    ((2, 4, 4, 5), Fraction(1, 108)),
    ((2, 4, 4, 6), Fraction(1, 108)),
    ((2, 4, 5, 5), Fraction(1, 108)),
    ((2, 4, 5, 6), Fraction(1, 54)),
    ((2, 4, 6, 6), Fraction(1, 108)),
    ((2, 5, 5, 5), Fraction(1, 324)),
    ((2, 5, 5, 6), Fraction(1, 108)),
    ((2, 5, 6, 6), Fraction(1, 108)),
    ((2, 6, 6, 6), Fraction(1, 324)),
    ((3, 3, 3, 3), Fraction(1, 1296)),
    ((3, 3, 3, 4), Fraction(1, 324)),
    ((3, 3, 3, 5), Fraction(1, 324)),
    ((3, 3, 3, 6), Fraction(1, 324)),
    ((3, 3, 4, 4), Fraction(1, 216)),
    ((3, 3, 4, 5), Fraction(1, 108)),
    ((3, 3, 4, 6), Fraction(1, 108)),
    ((3, 3, 5, 5), Fraction(1, 216)),
    ((3, 3, 5, 6), Fraction(1, 108)),
    ((3, 3, 6, 6), Fraction(1, 216)),
    ((3, 4, 4, 4), Fraction(1, 324)),
    ((3, 4, 4, 5), Fraction(1, 108)),
    ((3, 4, 4, 6), Fraction(1, 108)),
    ((3, 4, 5, 5), Fraction(1, 108)),
    ((3, 4, 5, 6), Fraction(1, 54)),
    ((3, 4, 6, 6), Fraction(1, 108)),
    ((3, 5, 5, 5), Fraction(1, 324)),
    ((3, 5, 5, 6), Fraction(1, 108)),
    ((3, 5, 6, 6), Fraction(1, 108)),
    ((3, 6, 6, 6), Fraction(1, 324)),
    ((4, 4, 4, 4), Fraction(1, 1296)),
    ((4, 4, 4, 5), Fraction(1, 324)),
    ((4, 4, 4, 6), Fraction(1, 324)),
    ((4, 4, 5, 5), Fraction(1, 216)),
    ((4, 4, 5, 6), Fraction(1, 108)),
    ((4, 4, 6, 6), Fraction(1, 216)),
    ((4, 5, 5, 5), Fraction(1, 324)),
    ((4, 5, 5, 6), Fraction(1, 108)),
    ((4, 5, 6, 6), Fraction(1, 108)),
    ((4, 6, 6, 6), Fraction(1, 324)),
    ((5, 5, 5, 5), Fraction(1, 1296)),
    ((5, 5, 5, 6), Fraction(1, 324)),
    ((5, 5, 6, 6), Fraction(1, 216)),
    ((5, 6, 6, 6), Fraction(1, 324)),
    ((6, 6, 6, 6), Fraction(1, 1296)),
]

ROLL_5_PROB = [
    ((1, 1, 1, 1, 1), Fraction(1, 7776)),
    ((1, 1, 1, 1, 2), Fraction(5, 7776)),
    ((1, 1, 1, 1, 3), Fraction(5, 7776)),
    ((1, 1, 1, 1, 4), Fraction(5, 7776)),
    ((1, 1, 1, 1, 5), Fraction(5, 7776)),
    ((1, 1, 1, 1, 6), Fraction(5, 7776)),
    ((1, 1, 1, 2, 2), Fraction(5, 3888)),
    ((1, 1, 1, 2, 3), Fraction(5, 1944)),
    ((1, 1, 1, 2, 4), Fraction(5, 1944)),
    ((1, 1, 1, 2, 5), Fraction(5, 1944)),
    ((1, 1, 1, 2, 6), Fraction(5, 1944)),
    ((1, 1, 1, 3, 3), Fraction(5, 3888)),
    ((1, 1, 1, 3, 4), Fraction(5, 1944)),
    ((1, 1, 1, 3, 5), Fraction(5, 1944)),
    ((1, 1, 1, 3, 6), Fraction(5, 1944)),
    ((1, 1, 1, 4, 4), Fraction(5, 3888)),
    ((1, 1, 1, 4, 5), Fraction(5, 1944)),
    ((1, 1, 1, 4, 6), Fraction(5, 1944)),
    ((1, 1, 1, 5, 5), Fraction(5, 3888)),
    ((1, 1, 1, 5, 6), Fraction(5, 1944)),
    ((1, 1, 1, 6, 6), Fraction(5, 3888)),
    ((1, 1, 2, 2, 2), Fraction(5, 3888)),
    ((1, 1, 2, 2, 3), Fraction(5, 1296)),
    ((1, 1, 2, 2, 4), Fraction(5, 1296)),
    ((1, 1, 2, 2, 5), Fraction(5, 1296)),
    ((1, 1, 2, 2, 6), Fraction(5, 1296)),
    ((1, 1, 2, 3, 3), Fraction(5, 1296)),
    ((1, 1, 2, 3, 4), Fraction(5, 648)),
    ((1, 1, 2, 3, 5), Fraction(5, 648)),
    ((1, 1, 2, 3, 6), Fraction(5, 648)),
    ((1, 1, 2, 4, 4), Fraction(5, 1296)),
    ((1, 1, 2, 4, 5), Fraction(5, 648)),
    ((1, 1, 2, 4, 6), Fraction(5, 648)),
    ((1, 1, 2, 5, 5), Fraction(5, 1296)),
    ((1, 1, 2, 5, 6), Fraction(5, 648)),
    ((1, 1, 2, 6, 6), Fraction(5, 1296)),
    ((1, 1, 3, 3, 3), Fraction(5, 3888)),
    ((1, 1, 3, 3, 4), Fraction(5, 1296)),
    ((1, 1, 3, 3, 5), Fraction(5, 1296)),
    ((1, 1, 3, 3, 6), Fraction(5, 1296)),
    ((1, 1, 3, 4, 4), Fraction(5, 1296)),
    ((1, 1, 3, 4, 5), Fraction(5, 648)),
    ((1, 1, 3, 4, 6), Fraction(5, 648)),
    ((1, 1, 3, 5, 5), Fraction(5, 1296)),
    ((1, 1, 3, 5, 6), Fraction(5, 648)),
    ((1, 1, 3, 6, 6), Fraction(5, 1296)),
    ((1, 1, 4, 4, 4), Fraction(5, 3888)),
    ((1, 1, 4, 4, 5), Fraction(5, 1296)),
    ((1, 1, 4, 4, 6), Fraction(5, 1296)),
    ((1, 1, 4, 5, 5), Fraction(5, 1296)),
    ((1, 1, 4, 5, 6), Fraction(5, 648)),
    ((1, 1, 4, 6, 6), Fraction(5, 1296)),
    ((1, 1, 5, 5, 5), Fraction(5, 3888)),
    ((1, 1, 5, 5, 6), Fraction(5, 1296)),
    ((1, 1, 5, 6, 6), Fraction(5, 1296)),
    ((1, 1, 6, 6, 6), Fraction(5, 3888)),
    ((1, 2, 2, 2, 2), Fraction(5, 7776)),
    ((1, 2, 2, 2, 3), Fraction(5, 1944)),
    ((1, 2, 2, 2, 4), Fraction(5, 1944)),
    ((1, 2, 2, 2, 5), Fraction(5, 1944)),
    ((1, 2, 2, 2, 6), Fraction(5, 1944)),
    ((1, 2, 2, 3, 3), Fraction(5, 1296)),
    ((1, 2, 2, 3, 4), Fraction(5, 648)),
    ((1, 2, 2, 3, 5), Fraction(5, 648)),
    ((1, 2, 2, 3, 6), Fraction(5, 648)),
    ((1, 2, 2, 4, 4), Fraction(5, 1296)),
    ((1, 2, 2, 4, 5), Fraction(5, 648)),
    ((1, 2, 2, 4, 6), Fraction(5, 648)),
    ((1, 2, 2, 5, 5), Fraction(5, 1296)),
    ((1, 2, 2, 5, 6), Fraction(5, 648)),
    ((1, 2, 2, 6, 6), Fraction(5, 1296)),
    ((1, 2, 3, 3, 3), Fraction(5, 1944)),
    ((1, 2, 3, 3, 4), Fraction(5, 648)),
    ((1, 2, 3, 3, 5), Fraction(5, 648)),
    ((1, 2, 3, 3, 6), Fraction(5, 648)),
    ((1, 2, 3, 4, 4), Fraction(5, 648)),
    ((1, 2, 3, 4, 5), Fraction(5, 324)),
    ((1, 2, 3, 4, 6), Fraction(5, 324)),
    ((1, 2, 3, 5, 5), Fraction(5, 648)),
    ((1, 2, 3, 5, 6), Fraction(5, 324)),
    ((1, 2, 3, 6, 6), Fraction(5, 648)),
    ((1, 2, 4, 4, 4), Fraction(5, 1944)),
    ((1, 2, 4, 4, 5), Fraction(5, 648)),
    ((1, 2, 4, 4, 6), Fraction(5, 648)),
    ((1, 2, 4, 5, 5), Fraction(5, 648)),
    ((1, 2, 4, 5, 6), Fraction(5, 324)),
    ((1, 2, 4, 6, 6), Fraction(5, 648)),
    ((1, 2, 5, 5, 5), Fraction(5, 1944)),
    ((1, 2, 5, 5, 6), Fraction(5, 648)),
    ((1, 2, 5, 6, 6), Fraction(5, 648)),
    ((1, 2, 6, 6, 6), Fraction(5, 1944)),
    ((1, 3, 3, 3, 3), Fraction(5, 7776)),
    ((1, 3, 3, 3, 4), Fraction(5, 1944)),
    ((1, 3, 3, 3, 5), Fraction(5, 1944)),
    ((1, 3, 3, 3, 6), Fraction(5, 1944)),
    ((1, 3, 3, 4, 4), Fraction(5, 1296)),
    ((1, 3, 3, 4, 5), Fraction(5, 648)),
    ((1, 3, 3, 4, 6), Fraction(5, 648)),
    ((1, 3, 3, 5, 5), Fraction(5, 1296)),
    ((1, 3, 3, 5, 6), Fraction(5, 648)),
    ((1, 3, 3, 6, 6), Fraction(5, 1296)),
    ((1, 3, 4, 4, 4), Fraction(5, 1944)),
    ((1, 3, 4, 4, 5), Fraction(5, 648)),
    ((1, 3, 4, 4, 6), Fraction(5, 648)),
    ((1, 3, 4, 5, 5), Fraction(5, 648)),
    ((1, 3, 4, 5, 6), Fraction(5, 324)),
    ((1, 3, 4, 6, 6), Fraction(5, 648)),
    ((1, 3, 5, 5, 5), Fraction(5, 1944)),
    ((1, 3, 5, 5, 6), Fraction(5, 648)),
    ((1, 3, 5, 6, 6), Fraction(5, 648)),
    ((1, 3, 6, 6, 6), Fraction(5, 1944)),
    ((1, 4, 4, 4, 4), Fraction(5, 7776)),
    ((1, 4, 4, 4, 5), Fraction(5, 1944)),
    ((1, 4, 4, 4, 6), Fraction(5, 1944)),
    ((1, 4, 4, 5, 5), Fraction(5, 1296)),
    ((1, 4, 4, 5, 6), Fraction(5, 648)),
    ((1, 4, 4, 6, 6), Fraction(5, 1296)),
    ((1, 4, 5, 5, 5), Fraction(5, 1944)),
    ((1, 4, 5, 5, 6), Fraction(5, 648)),
    ((1, 4, 5, 6, 6), Fraction(5, 648)),
    ((1, 4, 6, 6, 6), Fraction(5, 1944)),
    ((1, 5, 5, 5, 5), Fraction(5, 7776)),
    ((1, 5, 5, 5, 6), Fraction(5, 1944)),
    ((1, 5, 5, 6, 6), Fraction(5, 1296)),
    ((1, 5, 6, 6, 6), Fraction(5, 1944)),
    ((1, 6, 6, 6, 6), Fraction(5, 7776)),
    ((2, 2, 2, 2, 2), Fraction(1, 7776)),
    ((2, 2, 2, 2, 3), Fraction(5, 7776)),
    ((2, 2, 2, 2, 4), Fraction(5, 7776)),
    ((2, 2, 2, 2, 5), Fraction(5, 7776)),
    ((2, 2, 2, 2, 6), Fraction(5, 7776)),
    ((2, 2, 2, 3, 3), Fraction(5, 3888)),
    ((2, 2, 2, 3, 4), Fraction(5, 1944)),
    ((2, 2, 2, 3, 5), Fraction(5, 1944)),
    ((2, 2, 2, 3, 6), Fraction(5, 1944)),
    ((2, 2, 2, 4, 4), Fraction(5, 3888)),
    ((2, 2, 2, 4, 5), Fraction(5, 1944)),
    ((2, 2, 2, 4, 6), Fraction(5, 1944)),
    ((2, 2, 2, 5, 5), Fraction(5, 3888)),
    ((2, 2, 2, 5, 6), Fraction(5, 1944)),
    ((2, 2, 2, 6, 6), Fraction(5, 3888)),
    ((2, 2, 3, 3, 3), Fraction(5, 3888)),
    ((2, 2, 3, 3, 4), Fraction(5, 1296)),
    ((2, 2, 3, 3, 5), Fraction(5, 1296)),
    ((2, 2, 3, 3, 6), Fraction(5, 1296)),
    ((2, 2, 3, 4, 4), Fraction(5, 1296)),
    ((2, 2, 3, 4, 5), Fraction(5, 648)),
    ((2, 2, 3, 4, 6), Fraction(5, 648)),
    ((2, 2, 3, 5, 5), Fraction(5, 1296)),
    ((2, 2, 3, 5, 6), Fraction(5, 648)),
    ((2, 2, 3, 6, 6), Fraction(5, 1296)),
    ((2, 2, 4, 4, 4), Fraction(5, 3888)),
    ((2, 2, 4, 4, 5), Fraction(5, 1296)),
    ((2, 2, 4, 4, 6), Fraction(5, 1296)),
    ((2, 2, 4, 5, 5), Fraction(5, 1296)),
    ((2, 2, 4, 5, 6), Fraction(5, 648)),
    ((2, 2, 4, 6, 6), Fraction(5, 1296)),
    ((2, 2, 5, 5, 5), Fraction(5, 3888)),
    ((2, 2, 5, 5, 6), Fraction(5, 1296)),
    ((2, 2, 5, 6, 6), Fraction(5, 1296)),
    ((2, 2, 6, 6, 6), Fraction(5, 3888)),
    ((2, 3, 3, 3, 3), Fraction(5, 7776)),
    ((2, 3, 3, 3, 4), Fraction(5, 1944)),
    ((2, 3, 3, 3, 5), Fraction(5, 1944)),
    ((2, 3, 3, 3, 6), Fraction(5, 1944)),
    ((2, 3, 3, 4, 4), Fraction(5, 1296)),
    ((2, 3, 3, 4, 5), Fraction(5, 648)),
    ((2, 3, 3, 4, 6), Fraction(5, 648)),
    ((2, 3, 3, 5, 5), Fraction(5, 1296)),
    ((2, 3, 3, 5, 6), Fraction(5, 648)),
    ((2, 3, 3, 6, 6), Fraction(5, 1296)),
    ((2, 3, 4, 4, 4), Fraction(5, 1944)),
    ((2, 3, 4, 4, 5), Fraction(5, 648)),
    ((2, 3, 4, 4, 6), Fraction(5, 648)),
    ((2, 3, 4, 5, 5), Fraction(5, 648)),
    ((2, 3, 4, 5, 6), Fraction(5, 324)),
    ((2, 3, 4, 6, 6), Fraction(5, 648)),
    ((2, 3, 5, 5, 5), Fraction(5, 1944)),
    ((2, 3, 5, 5, 6), Fraction(5, 648)),
    ((2, 3, 5, 6, 6), Fraction(5, 648)),
    ((2, 3, 6, 6, 6), Fraction(5, 1944)),
    ((2, 4, 4, 4, 4), Fraction(5, 7776)),
    ((2, 4, 4, 4, 5), Fraction(5, 1944)),
    ((2, 4, 4, 4, 6), Fraction(5, 1944)),
    ((2, 4, 4, 5, 5), Fraction(5, 1296)),
    ((2, 4, 4, 5, 6), Fraction(5, 648)),
    ((2, 4, 4, 6, 6), Fraction(5, 1296)),
    ((2, 4, 5, 5, 5), Fraction(5, 1944)),
    ((2, 4, 5, 5, 6), Fraction(5, 648)),
    ((2, 4, 5, 6, 6), Fraction(5, 648)),
    ((2, 4, 6, 6, 6), Fraction(5, 1944)),
    ((2, 5, 5, 5, 5), Fraction(5, 7776)),
    ((2, 5, 5, 5, 6), Fraction(5, 1944)),
    ((2, 5, 5, 6, 6), Fraction(5, 1296)),
    ((2, 5, 6, 6, 6), Fraction(5, 1944)),
    ((2, 6, 6, 6, 6), Fraction(5, 7776)),
    ((3, 3, 3, 3, 3), Fraction(1, 7776)),
    ((3, 3, 3, 3, 4), Fraction(5, 7776)),
    ((3, 3, 3, 3, 5), Fraction(5, 7776)),
    ((3, 3, 3, 3, 6), Fraction(5, 7776)),
    ((3, 3, 3, 4, 4), Fraction(5, 3888)),
    ((3, 3, 3, 4, 5), Fraction(5, 1944)),
    ((3, 3, 3, 4, 6), Fraction(5, 1944)),
    ((3, 3, 3, 5, 5), Fraction(5, 3888)),
    ((3, 3, 3, 5, 6), Fraction(5, 1944)),
    ((3, 3, 3, 6, 6), Fraction(5, 3888)),
    ((3, 3, 4, 4, 4), Fraction(5, 3888)),
    ((3, 3, 4, 4, 5), Fraction(5, 1296)),
    ((3, 3, 4, 4, 6), Fraction(5, 1296)),
    ((3, 3, 4, 5, 5), Fraction(5, 1296)),
    ((3, 3, 4, 5, 6), Fraction(5, 648)),
    ((3, 3, 4, 6, 6), Fraction(5, 1296)),
    ((3, 3, 5, 5, 5), Fraction(5, 3888)),
    ((3, 3, 5, 5, 6), Fraction(5, 1296)),
    ((3, 3, 5, 6, 6), Fraction(5, 1296)),
    ((3, 3, 6, 6, 6), Fraction(5, 3888)),
    ((3, 4, 4, 4, 4), Fraction(5, 7776)),
    ((3, 4, 4, 4, 5), Fraction(5, 1944)),
    ((3, 4, 4, 4, 6), Fraction(5, 1944)),
    ((3, 4, 4, 5, 5), Fraction(5, 1296)),
    ((3, 4, 4, 5, 6), Fraction(5, 648)),
    ((3, 4, 4, 6, 6), Fraction(5, 1296)),
    ((3, 4, 5, 5, 5), Fraction(5, 1944)),
    ((3, 4, 5, 5, 6), Fraction(5, 648)),
    ((3, 4, 5, 6, 6), Fraction(5, 648)),
    ((3, 4, 6, 6, 6), Fraction(5, 1944)),
    ((3, 5, 5, 5, 5), Fraction(5, 7776)),
    ((3, 5, 5, 5, 6), Fraction(5, 1944)),
    ((3, 5, 5, 6, 6), Fraction(5, 1296)),
    ((3, 5, 6, 6, 6), Fraction(5, 1944)),
    ((3, 6, 6, 6, 6), Fraction(5, 7776)),
    ((4, 4, 4, 4, 4), Fraction(1, 7776)),
    ((4, 4, 4, 4, 5), Fraction(5, 7776)),
    ((4, 4, 4, 4, 6), Fraction(5, 7776)),
    ((4, 4, 4, 5, 5), Fraction(5, 3888)),
    ((4, 4, 4, 5, 6), Fraction(5, 1944)),
    ((4, 4, 4, 6, 6), Fraction(5, 3888)),
    ((4, 4, 5, 5, 5), Fraction(5, 3888)),
    ((4, 4, 5, 5, 6), Fraction(5, 1296)),
    ((4, 4, 5, 6, 6), Fraction(5, 1296)),
    ((4, 4, 6, 6, 6), Fraction(5, 3888)),
    ((4, 5, 5, 5, 5), Fraction(5, 7776)),
    ((4, 5, 5, 5, 6), Fraction(5, 1944)),
    ((4, 5, 5, 6, 6), Fraction(5, 1296)),
    ((4, 5, 6, 6, 6), Fraction(5, 1944)),
    ((4, 6, 6, 6, 6), Fraction(5, 7776)),
    ((5, 5, 5, 5, 5), Fraction(1, 7776)),
    ((5, 5, 5, 5, 6), Fraction(5, 7776)),
    ((5, 5, 5, 6, 6), Fraction(5, 3888)),
    ((5, 5, 6, 6, 6), Fraction(5, 3888)),
    ((5, 6, 6, 6, 6), Fraction(5, 7776)),
    ((6, 6, 6, 6, 6), Fraction(1, 7776)),
]

type Combo = Literal[
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
]

type Die = Literal[1, 2, 3, 4, 5, 6]

type Choice = Combo | tuple[Die] | tuple[Die, Die] | tuple[Die, Die, Die] | tuple[Die, Die, Die, Die] | tuple[Die, Die, Die, Die, Die]

CACHE: dict[Game, Fraction] = {}
CACHE_ROUND_START: dict[Game, Fraction] = {}

COMBOS: list[Combo] = [
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
]


class ComboAlreadyFilledError(Exception):
    pass


@dataclass(frozen=True)
class Game:
    dice: tuple[Die, Die, Die, Die, Die]
    rerolls_left: Literal[0, 1, 2]
    ones: int | None
    twos: int | None
    threes: int | None
    fours: int | None
    fives: int | None
    sixes: int | None
    one_pair: int | None
    two_pairs: int | None
    three_of_a_kind: int | None
    four_of_a_kind: int | None
    small_straight: int | None
    large_straight: int | None
    full_house: int | None
    chance: int | None
    yatzy: int | None

    def ended(self) -> bool:
        return self.round == 15

    def has_bonus(self) -> bool:
        total = 0

        if self.ones is not None:
            total += self.ones
        if self.twos is not None:
            total += self.twos
        if self.threes is not None:
            total += self.threes
        if self.fours is not None:
            total += self.fours
        if self.fives is not None:
            total += self.fives
        if self.sixes is not None:
            total += self.sixes

        return total >= 63

    def round(self) -> int:
        return (
            (0 if self.ones is None else 1)
            + (0 if self.twos is None else 1)
            + (0 if self.threes is None else 1)
            + (0 if self.fours is None else 1)
            + (0 if self.fives is None else 1)
            + (0 if self.sixes is None else 1)
            + (0 if self.one_pair is None else 1)
            + (0 if self.two_pairs is None else 1)
            + (0 if self.three_of_a_kind is None else 1)
            + (0 if self.four_of_a_kind is None else 1)
            + (0 if self.small_straight is None else 1)
            + (0 if self.large_straight is None else 1)
            + (0 if self.full_house is None else 1)
            + (0 if self.chance is None else 1)
            + (0 if self.yatzy is None else 1)
        )

    def score(self) -> int:
        total = 0

        if self.ones is not None:
            total += self.ones
        if self.twos is not None:
            total += self.twos
        if self.threes is not None:
            total += self.threes
        if self.fours is not None:
            total += self.fours
        if self.fives is not None:
            total += self.fives
        if self.sixes is not None:
            total += self.sixes

        if total >= 63:
            total += 50  # bonus

        if self.one_pair is not None:
            total += self.one_pair
        if self.two_pairs is not None:
            total += self.two_pairs
        if self.three_of_a_kind is not None:
            total += self.three_of_a_kind
        if self.four_of_a_kind is not None:
            total += self.four_of_a_kind
        if self.small_straight is not None:
            total += self.small_straight
        if self.large_straight is not None:
            total += self.large_straight
        if self.full_house is not None:
            total += self.full_house
        if self.chance is not None:
            total += self.chance
        if self.yatzy is not None:
            total += self.yatzy

        return total

    def select_combo(self, combo: Combo) -> Game:
        if combo == "ones":
            if self.ones is not None:
                raise ComboAlreadyFilledError
            return replace(self, ones=self.dice.count(1))
        if combo == "twos":
            if self.twos is not None:
                raise ComboAlreadyFilledError
            return replace(self, twos=2 * self.dice.count(2))
        if combo == "threes":
            if self.threes is not None:
                raise ComboAlreadyFilledError
            return replace(self, threes=3 * self.dice.count(3))
        if combo == "fours":
            if self.fours is not None:
                raise ComboAlreadyFilledError
            return replace(self, fours=4 * self.dice.count(4))
        if combo == "fives":
            if self.fives is not None:
                raise ComboAlreadyFilledError
            return replace(self, fives=5 * self.dice.count(5))
        if combo == "sixes":
            if self.sixes is not None:
                raise ComboAlreadyFilledError
            return replace(self, sixes=6 * self.dice.count(6))
        if combo == "one_pair":
            if self.one_pair is not None:
                raise ComboAlreadyFilledError

            pair: Die | None = None
            if self.dice.count(6) >= 2:
                pair = 6
            elif self.dice.count(5) >= 2:
                pair = 5
            elif self.dice.count(4) >= 2:
                pair = 4
            elif self.dice.count(3) >= 2:
                pair = 3
            elif self.dice.count(2) >= 2:
                pair = 2
            elif self.dice.count(1) >= 2:
                pair = 1

            if pair is not None:
                return replace(self, one_pair=2 * pair)
            return replace(self, one_pair=0)
        if combo == "two_pairs":
            if self.two_pairs is not None:
                raise ComboAlreadyFilledError
            pairs: list[Die] = []
            if self.dice.count(6) >= 2:
                pairs.append(6)
            if self.dice.count(5) >= 2:
                pairs.append(5)
            if self.dice.count(4) >= 2:
                pairs.append(4)
            if self.dice.count(3) >= 2:
                pairs.append(3)
            if self.dice.count(2) >= 2:
                pairs.append(2)
            if self.dice.count(1) >= 2:
                pairs.append(1)

            assert len(pairs) <= 2
            if len(pairs) == 2:
                return replace(self, two_pairs=2 * pairs[0] + 2 * pairs[1])
            return replace(self, two_pairs=0)
        if combo == "three_of_a_kind":
            if self.three_of_a_kind is not None:
                raise ComboAlreadyFilledError
            die: Die | None = None
            if self.dice.count(6) >= 3:
                die = 6
            elif self.dice.count(5) >= 3:
                die = 5
            elif self.dice.count(4) >= 3:
                die = 4
            elif self.dice.count(3) >= 3:
                die = 3
            elif self.dice.count(2) >= 3:
                die = 2
            elif self.dice.count(1) >= 3:
                die = 1

            if die is not None:
                return replace(self, three_of_a_kind=3 * die)
            return replace(self, three_of_a_kind=0)
        if combo == "four_of_a_kind":
            if self.four_of_a_kind is not None:
                raise ComboAlreadyFilledError
            die: Die | None = None
            if self.dice.count(6) >= 4:
                die = 6
            elif self.dice.count(5) >= 4:
                die = 5
            elif self.dice.count(4) >= 4:
                die = 4
            elif self.dice.count(3) >= 4:
                die = 3
            elif self.dice.count(2) >= 4:
                die = 2
            elif self.dice.count(1) >= 4:
                die = 1
            if die is not None:
                return replace(self, four_of_a_kind=4 * die)
            return replace(self, four_of_a_kind=0)
        if combo == "small_straight":
            if self.small_straight is not None:
                raise ComboAlreadyFilledError
            if self.dice == (1, 2, 3, 4, 5):
                return replace(self, small_straight=15)
            return replace(self, small_straight=0)
        if combo == "large_straight":
            if self.large_straight is not None:
                raise ComboAlreadyFilledError
            if self.dice == (2, 3, 4, 5, 6):
                return replace(self, large_straight=20)
            return replace(self, large_straight=0)
        if combo == "full_house":
            if self.full_house is not None:
                raise ComboAlreadyFilledError
            if self.dice[0] == self.dice[1] and self.dice[1] != self.dice[2] and self.dice[2] == self.dice[3] == self.dice[4]:
                return replace(self, full_house=2 * self.dice[0] + 3 * self.dice[2])
            elif self.dice[0] == self.dice[1] == self.dice[2] and self.dice[2] != self.dice[3] and self.dice[3] == self.dice[4]:
                return replace(self, full_house=3 * self.dice[0] + 2 * self.dice[3])
            return replace(self, full_house=0)
        if combo == "chance":
            if self.chance is not None:
                raise ComboAlreadyFilledError
            return replace(self, chance=sum(self.dice))
        if combo == "yatzy":
            if self.yatzy is not None:
                raise ComboAlreadyFilledError
            if self.dice.count(self.dice[0]) == 5:
                return replace(self, yatzy=50)
            return replace(self, yatzy=0)
        raise Exception("invalid combo")

    def set_dice_and_rerolls(self, dice: tuple[Die, Die, Die, Die, Die], rerolls_left: Literal[0, 1, 2]) -> Game:
        return replace(self, dice=dice, rerolls_left=rerolls_left)


def get_choices(game: Game) -> list[Choice]:
    choices: list[Choice] = []

    for combo in COMBOS:
        if getattr(game, combo) is None:
            choices.append(combo)

    if game.rerolls_left > 0:
        for k in range(1, 6):
            for dice in combinations(game.dice, k):
                choices.append(dice)

    return choices


def expected_value_1_left_0_rerolls(game: Game) -> Fraction:
    assert game.round() == 14
    assert game.rerolls_left == 0

    if game in CACHE:
        return CACHE[game]

    for combo in COMBOS:
        if getattr(game, combo) is None:
            game_copy = game.select_combo(combo)
            break

    value = Fraction(game_copy.score())
    CACHE[game] = value
    return value


def expected_value_1_left_1_reroll(game: Game) -> Fraction:
    assert game.round() == 14
    assert game.rerolls_left == 1

    if game in CACHE:
        return CACHE[game]

    choices = get_choices(game)

    max_value = Fraction(0)

    for choice in choices:
        if isinstance(choice, str):
            game_copy = game.select_combo(choice)
            value = Fraction(game_copy.score())
            if value > max_value:
                max_value = value
        elif isinstance(choice, tuple):
            retained_dice = list(game.dice)
            for die in choice:
                retained_dice.remove(die)
            match len(choice):
                case 1:
                    options = ROLL_1_PROB
                case 2:
                    options = ROLL_2_PROB
                case 3:
                    options = ROLL_3_PROB
                case 4:
                    options = ROLL_4_PROB
                case 5:
                    options = ROLL_5_PROB
                case _:
                    raise Exception("unexpected amount of rerolled dice in `choice`")

            value = Fraction(0)
            for rerolled_dice, prob in options:
                new_dice = tuple(sorted([*retained_dice, *rerolled_dice]))
                game_copy = game.set_dice_and_rerolls(new_dice, 0)
                value += prob * expected_value_1_left_0_rerolls(game_copy)

            if value > max_value:
                max_value = value
        else:
            raise Exception("unexpected `choice`")

    CACHE[game] = max_value
    return max_value


def expected_value_1_left_2_rerolls(game: Game) -> Fraction:
    assert game.round() == 14
    assert game.rerolls_left == 2

    if game in CACHE:
        return CACHE[game]

    choices = get_choices(game)

    max_value = Fraction(0)

    for choice in choices:
        if isinstance(choice, str):
            game_copy = game.select_combo(choice)
            value = Fraction(game_copy.score())
            if value > max_value:
                max_value = value
        elif isinstance(choice, tuple):
            retained_dice = list(game.dice)
            for die in choice:
                retained_dice.remove(die)
            match len(choice):
                case 1:
                    options = ROLL_1_PROB
                case 2:
                    options = ROLL_2_PROB
                case 3:
                    options = ROLL_3_PROB
                case 4:
                    options = ROLL_4_PROB
                case 5:
                    options = ROLL_5_PROB
                case _:
                    raise Exception("unexpected amount of rerolled dice in `choice`")

            value = Fraction(0)
            for rerolled_dice, prob in options:
                new_dice = tuple(sorted([*retained_dice, *rerolled_dice]))
                game_copy = game.set_dice_and_rerolls(new_dice, 1)
                value += prob * expected_value_1_left_1_reroll(game_copy)

            if value > max_value:
                max_value = value
        else:
            raise Exception("unexpected `choice`")

    CACHE[game] = max_value
    return max_value


def expected_value_1_left_round_start(game: Game) -> Fraction:
    assert game.round() == 14

    cached_game = game.set_dice_and_rerolls((1, 1, 1, 1, 1), 2)
    if cached_game in CACHE_ROUND_START:
        return CACHE_ROUND_START[cached_game]

    value = Fraction(0)

    for dice, prob in ROLL_5_PROB:
        game_copy = game.set_dice_and_rerolls(dice, 2)
        value += prob * expected_value_1_left_2_rerolls(game_copy)

    CACHE_ROUND_START[cached_game] = value
    return value


def expected_value_2_left_0_rerolls(game: Game) -> Fraction:
    assert game.round() == 13
    assert game.rerolls_left == 0

    if game in CACHE:
        return CACHE[game]

    choices = get_choices(game)
    max_value = Fraction(0)

    for choice in choices:
        if not isinstance(choice, str):
            raise Exception("unexpected `choice`")

        game_copy = game.select_combo(choice)
        value = expected_value_1_left_round_start(game_copy)

        if value > max_value:
            max_value = value

    return max_value


def expected_value_2_left_1_reroll(game: Game) -> Fraction:
    assert game.round() == 13
    assert game.rerolls_left == 1

    if game in CACHE:
        return CACHE[game]

    choices = get_choices(game)

    max_value = Fraction(0)

    for choice in choices:
        if isinstance(choice, str):
            game_copy = game.select_combo(choice)
            value = expected_value_1_left_round_start(game_copy)
            if value > max_value:
                max_value = value
        elif isinstance(choice, tuple):
            retained_dice = list(game.dice)
            for die in choice:
                retained_dice.remove(die)
            match len(choice):
                case 1:
                    options = ROLL_1_PROB
                case 2:
                    options = ROLL_2_PROB
                case 3:
                    options = ROLL_3_PROB
                case 4:
                    options = ROLL_4_PROB
                case 5:
                    options = ROLL_5_PROB
                case _:
                    raise Exception("unexpected amount of rerolled dice in `choice`")

            value = Fraction(0)
            for rerolled_dice, prob in options:
                new_dice = tuple(sorted([*retained_dice, *rerolled_dice]))
                game_copy = game.set_dice_and_rerolls(new_dice, 0)
                value += prob * expected_value_2_left_0_rerolls(game_copy)

            if value > max_value:
                max_value = value
        else:
            raise Exception("unexpected `choice`")

    CACHE[game] = max_value
    return max_value


def expected_value_2_left_2_rerolls(game: Game) -> Fraction:
    assert game.round() == 13
    assert game.rerolls_left == 2

    if game in CACHE:
        return CACHE[game]

    choices = get_choices(game)

    max_value = Fraction(0)

    for choice in choices:
        if isinstance(choice, str):
            game_copy = game.select_combo(choice)
            value = expected_value_1_left_round_start(game_copy)
            if value > max_value:
                max_value = value
        elif isinstance(choice, tuple):
            retained_dice = list(game.dice)
            for die in choice:
                retained_dice.remove(die)
            match len(choice):
                case 1:
                    options = ROLL_1_PROB
                case 2:
                    options = ROLL_2_PROB
                case 3:
                    options = ROLL_3_PROB
                case 4:
                    options = ROLL_4_PROB
                case 5:
                    options = ROLL_5_PROB
                case _:
                    raise Exception("unexpected amount of rerolled dice in `choice`")

            value = Fraction(0)
            for rerolled_dice, prob in options:
                new_dice = tuple(sorted([*retained_dice, *rerolled_dice]))
                game_copy = game.set_dice_and_rerolls(new_dice, 1)
                value += prob * expected_value_2_left_1_reroll(game_copy)

            if value > max_value:
                max_value = value
        else:
            raise Exception("unexpected `choice`")

    CACHE[game] = max_value
    return max_value


def expected_value_2_left_round_start(game: Game) -> Fraction:
    assert game.round() == 13

    cached_game = game.set_dice_and_rerolls((1, 1, 1, 1, 1), 2)
    if cached_game in CACHE_ROUND_START:
        return CACHE_ROUND_START[cached_game]

    value = Fraction(0)

    for dice, prob in ROLL_5_PROB:
        game_copy = game.set_dice_and_rerolls(dice, 2)
        value += prob * expected_value_2_left_2_rerolls(game_copy)

    CACHE_ROUND_START[cached_game] = value
    return value


if __name__ == "__main__":
    game = Game(
        dice=(1, 2, 3, 3, 3),
        rerolls_left=2,
        ones=0,
        twos=None,
        threes=15,
        fours=20,
        fives=25,
        sixes=0,
        one_pair=0,
        two_pairs=0,
        three_of_a_kind=0,
        four_of_a_kind=0,
        small_straight=0,
        large_straight=0,
        full_house=0,
        chance=0,
        yatzy=None,
    )
    print("expected value:", expected_value_2_left_round_start(game))
