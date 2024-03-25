import pytest
from game import *
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


def test_piece_placements():

    initial_board = GameState()

    shape = np.array(
        [
            [0, 1, 0],
            [1, 1, 1],
            [0, 1, 0],
        ]
    )
    piece = Piece(shape)
    positions = initial_board.get_positions(0, piece)
    assert all(len(i) == 0 for i in positions)

    shape = np.array(
        [
            [1, 1],
            [1, 0],
        ]
    )
    piece = Piece(shape)
    positions = initial_board.get_positions(0, piece)
    assert sum(len(i) for i in positions) == 3  # 3 possible placements

    new_board = GameState()
    new_board.board = 2 << 4  # Place a piece by player 1 at (1, 0)

    shape = np.array(
        [
            [1, 1],
            [1, 0],
        ]
    )
    piece = Piece(shape)
    positions = new_board.get_positions(0, piece)
    assert sum(len(i) for i in positions) == 1  # 1 possible placement
    # find the position
    for i, trans in enumerate(piece.transforms):
        if len(positions[i]) == 0:
            continue
        else:
            assert positions[i][0] == (0, 0)
            # Place this piece
            new_board.place(0, trans, positions[i][0])
            assert (
                str(new_board)
                == "12"
                + 18 * "-"
                + "\n"
                + "11"
                + 18 * "-"
                + "\n"
                + ("-" * 20 + "\n") * 18
            )


def test_left_right_bitmasks():
    state = GameState()
    state.board = state.left_bitmask
    assert str(state) == ("-" + "4" * 19 + "\n") * 20
    state.board = state.right_bitmask
    assert str(state) == ("4" * 19 + "-" + "\n") * 20
