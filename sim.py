from blocks import BLOCKS
from game import Piece, GameState, PieceRotation
import random
import time
import argparse


def algorithm(
    name: str, game: GameState, moves: list[tuple[PieceRotation, tuple[int, int]]]
) -> tuple[PieceRotation, tuple[int, int]]:
    if len(moves) == 0:
        raise ValueError("No moves available")
    if name == "random":
        return random.choice(moves)
    elif name == "greedy":
        # random sort so that the chosen move is not always the first one
        random.shuffle(moves)
        return max(moves, key=lambda move: game.raw_pieces[move[0].parent].count)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Simulate the game")
    parser.add_argument(
        "--algorithm",
        "-a",
        type=str,
        default="random",
        help="Algorithm to use for the AI (random, greedy)",
    )
    args = parser.parse_args()

    boardsize = 20

    game = GameState(boardsize)

    avg = 0

    nmoves = 0

    while True:
        fails = 0
        for player in range(4):
            nmoves += 1
            before = time.time_ns()
            moves = list(game.get_moves(player))
            after = time.time_ns()
            print(f"Player {player} has {len(moves)} moves, took {after - before} ns")
            avg += after - before

            if len(moves) == 0:
                fails += 1
                continue

            move = algorithm(args.algorithm, game, moves)

            game.place(player, *move)

            print(game.debug_str(True, True))

        if fails == 4:
            break

    print(f"Average time per move: {avg / nmoves} ns, {avg / nmoves / 1_000_000} ms")
