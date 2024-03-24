import numpy as np
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
    def __init__(self, shape: np.ndarray):
        # make sure all numbers in the shape are either 0 or 1
        assert np.all(np.isin(shape, [0, 1]))

        self.transforms = set(
            [
                PieceRotation(np.rot90(reflshape, rots))
                for reflshape in [shape, np.fliplr(shape), np.flipud(shape)]
                for rots in range(4)
            ]
        )


# Single rotation/reflection of a piece
class PieceRotation:
    def __init__(self, shape: np.ndarray):
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

        dims = shape.shape

        def get_or_0(x, y):
            if x < 0 or y < 0:
                return 0
            if x >= dims[0] or y >= dims[1]:
                return 0
            return shape[x, y]

        for i in range(dims[0]):
            for j in range(dims[1]):
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


# classes representing the state of the game
class GameState:
    def __init__(self, boardsize: int = 20):
        # Board state is represented as follows:
        # 0 = 0x0000 = empty
        # 1 = 0x0001 = player 1
        # 8 = 0x0008 = player 2
        # 64 = 0x0040 = player 3
        # 512 = 0x0200 = player 4
        self.board = np.zeros((boardsize, boardsize), dtype=np.uint16)
        # Treat the board as 4 pieces
        self.corners: list[list[set[str]]] = [
            [set() for _ in range(4)] for _ in range(4)
        ]

        # Player 1 gets (0, 0) corner going in the (1, 1) direction
        self.corners[0][3].add((0, 0))

        # Player 2 gets (board, 0) corner going in the (-1, 1) direction
        self.corners[1][1].add((boardsize - 1, 0))

        # Player 3 gets (0, board) corner going in the (1, -1) direction
        self.corners[2][2].add((0, boardsize - 1))

        # Player 4 gets (board, board) corner going in the (-1, -1) direction
        self.corners[3][0].add((boardsize - 1, boardsize - 1))
