import pytest
from game import Piece, PieceRotation
import numpy as np


def test_corners():
    shape = np.array(
        [
            [0, 1, 0],
            [1, 1, 1],
            [0, 1, 0],
        ]
    )
    piece = PieceRotation(shape)
    assert piece.corners == [
        {(0, 1), (1, 0)},  # (-1, -1) corners
        {(0, 1), (1, 2)},  # (-1, 1) corners
        {(1, 0), (2, 1)},  # (1, -1) corners
        {(1, 2), (2, 1)},  # (1, 1) corners
    ]

    shape = np.array(
        [
            [0, 1, 0],
            [1, 1, 0],
            [0, 1, 0],
        ]
    )
    piece = PieceRotation(shape)
    assert piece.corners == [
        {(0, 1), (1, 0)},  # (-1, -1) corners
        {(0, 1)},  # (-1, 1) corners
        {(1, 0), (2, 1)},  # (1, -1) corners
        {(2, 1)},  # (1, 1) corners
    ]

    shape = np.array(
        [
            [1, 1],
            [1, 1],
        ]
    )
    piece = PieceRotation(shape)
    assert piece.corners == [
        {(0, 0)},  # (-1, -1) corners
        {(0, 1)},  # (-1, 1) corners
        {(1, 0)},  # (1, -1) corners
        {(1, 1)},  # (1, 1) corners
    ]


def test_piece_rots():
    shape = np.array(
        [
            [0, 1, 0],
            [1, 1, 1],
            [0, 1, 0],
        ]
    )  # No rotations or reflections
    assert len(Piece(shape).transforms) == 1
    shape = np.array(
        [
            [0, 1, 0],
            [1, 1, 0],
            [0, 1, 0],
        ]
    )  # 4 total rotations
    assert len(Piece(shape).transforms) == 4
    shape = np.array(
        [
            [0, 0, 1],
            [1, 1, 1],
            [0, 0, 0],
        ]
    )  # All 8 reflections and rotations
    assert len(Piece(shape).transforms) == 8
