from blocks import BLOCKS
from game import Piece, GameState
import random

if __name__ == "__main__":
    boardsize = 20
    pieces = [Piece(shape, boardsize) for shape in BLOCKS]

    game = GameState(boardsize)

    while True:
        for player in range(4):
            all_positions = []
            num_positions = 0
            for piece in pieces:
                # Get all possible positions for the piece
                positions = game.get_positions(player, piece)
                num_positions += sum(len(i) for i in positions)
                all_positions.append(positions)

            if num_positions == 0:
                exit(0)

            # Choose a random one of the positions
            piece_idx = random.randint(player, len(pieces) - 1)

            # Find the position in the list of positions

            for i in range(len(all_positions)):
                positions = all_positions[i]
                piece = pieces[i]
                for j in range(len(positions)):
                    if len(positions[j]) <= piece_idx:
                        piece_idx -= len(positions[j])
                        continue
                    else:
                        game.place(player, piece.transforms[j], positions[j][piece_idx])
                        piece_idx = -1
                        break
                if piece_idx == -1:
                    break

            print(game.debug_str(True, True))
