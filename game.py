import numpy as np
import math
import itertools

# Order in which corners are stored
CORNER_ORDER = [
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
]


# Single piece
class Piece:
    def __init__(self, shape: np.ndarray, boardsize: int = 20):
        # make sure all numbers in the shape are either 0 or 1
        assert np.all(np.isin(shape, [0, 1]))

        self.transforms = list(
            set(
                [
                    PieceRotation(np.rot90(reflshape, rots), boardsize=boardsize)
                    for reflshape in [shape, np.fliplr(shape), np.flipud(shape)]
                    for rots in range(4)
                ]
            )
        )


# Single rotation/reflection of a piece
class PieceRotation:
    def __init__(self, shape: np.ndarray, boardsize: int = 20):
        # Find the corners of the piece
        # Calculate a hash of the shape specified by
        # the ones and zeroes in the shape separating rows by newlines
        # 010
        # 111
        # 010

        self.hash = hash("\n".join("".join(map(str, row)) for row in shape))

        # This is a set of all the corners of the piece
        self.corners = [set() for _ in range(4)]
        self.shape = shape
        self.shape.flags.writeable = False

        self.w = shape.shape[0]
        self.h = shape.shape[1]

        adjacent_tiles = set()
        for i in range(self.w):
            for j in range(self.h):
                if shape[i, j] == 1:
                    # Add all adjacent tiles to the set if they are not part of the piece
                    for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
                        if (
                            not (  # If it is out of bounds, it is automatically empty
                                i + dx >= 0
                                and i + dx < self.w
                                and j + dy >= 0
                                and j + dy < self.h
                            )
                            or shape[i + dx, j + dy] == 0
                        ):
                            adjacent_tiles.add((i + dx, j + dy))

        # bitmask of adjacent tiles we need to check to make sure we are not
        # next to a piece of the same color
        # Different bitmask for every player
        # Each bitmask will be shifted by one row and column (which has to be undone)
        # to allow adjacent bits to be checked
        self.adjacency_bitmasks = []

        # Block bitmasks for each player
        # Only the tiles that are part of the piece are set
        self.block_bitmasks = [0 for _ in range(4)]

        # Initial bitmask calculated with 1111 for all tiles in the piece
        initial_bitmask = 0
        for x in range(self.w):
            for y in range(self.h):
                if shape[x, y] == 1:
                    initial_bitmask |= 0b1111 << (((y + 1) * boardsize + (x + 1)) * 4)
                    for player in range(4):
                        self.block_bitmasks[player] |= 1 << (
                            (y * boardsize + x) * 4 + player
                        )

        for player in range(4):
            bitmask = initial_bitmask
            for x in range(-1, self.w + 1):
                for y in range(-1, self.h + 1):
                    if (x, y) in adjacent_tiles:
                        # Rows are length boardsize
                        # each element is 4bits
                        bitmask |= (1 << player) << (
                            ((y + 1) * boardsize + (x + 1)) * 4
                        )
            self.adjacency_bitmasks.append(bitmask)

        def get_or_0(x, y):
            if x < 0 or y < 0:
                return 0
            if x >= self.w or y >= self.h:
                return 0
            return shape[x, y]

        for i in range(self.w):
            for j in range(self.h):
                if shape[i, j] == 1:
                    # This cell is filled
                    # Check if it is a corner
                    directions = [
                        (0, 1),
                        (1, 0),
                        (0, -1),
                        (-1, 0),
                    ]
                    directions = dict(
                        [((dx, dy), get_or_0(i + dx, j + dy)) for dx, dy in directions]
                    )

                    # If we have 0 in two neighboring directions, this is a corner
                    for k, corner in enumerate(CORNER_ORDER):
                        dx, dy = corner
                        if directions[(dx, 0)] == 0 and directions[(0, dy)] == 0:
                            self.corners[k].add((i, j))

    def __key__(self) -> int:
        return self.hash

    def __hash__(self) -> int:
        return self.hash

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, PieceRotation):
            return False
        return self.hash == other.hash

    def __repr__(self) -> str:
        return repr(self.shape)


def smart_lshift(imm: int, n: int) -> int:
    if n > 0:
        return imm << n
    return imm >> -n


# classes representing the state of the game
class GameState:
    def __init__(self, boardsize: int = 20):
        # Board state is represented as a number
        # split into chunks of 4 bits.
        # Players are one-hot encoded
        # 0000 = empty
        # 0001 = player 1
        # 0010 = player 2
        # 0100 = player 3
        # 1000 = player 4
        self.board = 0

        self.w = boardsize
        self.h = boardsize
        # Treat the board as 4 pieces
        self.corners: list[list[set[str]]] = [
            [set() for _ in range(4)] for _ in range(4)
        ]

        self.right_bitmask = (1 << (self.w * self.h * 4)) - 1
        self.left_bitmask = (1 << (self.w * self.h * 4)) - 1
        for i in range(self.h):
            self.right_bitmask ^= 0b1111 << (((i + 1) * self.w - 1) * 4)
            self.left_bitmask ^= 0b1111 << (i * self.w * 4)

        # A corner is represented by a position.
        # The position is the empty space diagonally adjacent to a block of the right color

        # Player 1 gets (0, 0) corner going in the (1, 1) direction
        self.corners[0][3].add((0, 0))

        # Player 2 gets (board, 0) corner going in the (-1, 1) direction
        self.corners[1][1].add((boardsize - 1, 0))

        # Player 3 gets (0, board) corner going in the (1, -1) direction
        self.corners[2][2].add((0, boardsize - 1))

        # Player 4 gets (board, board) corner going in the (-1, -1) direction
        self.corners[3][0].add((boardsize - 1, boardsize - 1))

    def get_positions(self, player: int, piece: Piece) -> tuple[tuple[int, int], int]:
        """
        Returns a list of all positions where the piece can be placed

        :param piece: The piece to be placed
        :return: A list of all positions where the piece can be placed. This comes in a (pos, transform index) tuple
        """

        positions = []
        # Pair opposite direction corners
        for trans in piece.transforms:
            # Pair opposite directions with each other
            # 0 <-> 3
            # 1 <-> 2
            poss = (
                list(itertools.product(trans.corners[0], self.corners[player][3]))
                + list(itertools.product(trans.corners[1], self.corners[player][2]))
                + list(itertools.product(trans.corners[2], self.corners[player][1]))
                + list(itertools.product(trans.corners[3], self.corners[player][0]))
            )

            valid_poss = []

            # Check if the piece can be placed at this position
            for (tx, ty), (bx, by) in poss:
                # Position of the top left corner of the piece
                px = bx - tx
                py = by - ty
                # Check if the piece is within the bounds of the board
                if px < 0 or py < 0 or px + trans.w > self.w or py + trans.h > self.h:
                    continue

                # Now, check if the piece can be placed here
                bitmask = trans.adjacency_bitmasks[player]

                # Move the piece to the right position
                bitmask = smart_lshift(bitmask, ((py - 1) * self.w + (px - 1)) * 4)

                if px == 0:
                    # We are at the left edge of the board, cancel out the right edge
                    bitmask &= self.right_bitmask
                elif px == self.w - 1:
                    # We are at the right edge of the board, cancel out the left edge
                    bitmask &= self.left_bitmask

                if bitmask & self.board != 0:
                    continue

                valid_poss.append((px, py))

            positions.append(valid_poss)

        return positions

    def place(
        self, player: int, pieceTransform: PieceRotation, pos: tuple[int, int]
    ) -> None:
        """
        Places a piece on the board

        :param player: The player placing the piece
        :param pieceTransform: The piece to be placed
        :param pos: The position to place the piece
        """
        px, py = pos
        bitmask = pieceTransform.block_bitmasks[player]
        bitmask = smart_lshift(bitmask, (py * self.w + px) * 4)
        self.board |= bitmask

    def __repr__(self) -> str:
        ret = ""
        for i in range(self.h):
            for j in range(self.w):
                onehot = (self.board >> ((i * self.w + j) * 4)) & 0b1111
                if onehot == 0:
                    ret += "-"
                else:
                    ret += str(int(math.log2(onehot)) + 1)
            ret += "\n"
        return ret
